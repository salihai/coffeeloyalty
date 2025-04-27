use crate::admin::{has_administrator, read_administrator, write_administrator};
use crate::allowance::{read_allowance, spend_allowance, write_allowance};
use crate::balance::{read_balance, receive_balance, spend_balance};
use crate::coffee::{read_coffee_points, read_free_coffee, write_coffee_points, write_free_coffee};
use crate::metadata::{read_decimal, read_name, read_symbol, write_metadata};
use crate::storage_types::{DataKey, INSTANCE_BUMP_AMOUNT, INSTANCE_LIFETIME_THRESHOLD};
use soroban_sdk::token::{self, Interface as _};
use soroban_sdk::{contract, contractimpl, Address, Env, String};
use soroban_token_sdk::metadata::TokenMetadata;
use soroban_token_sdk::TokenUtils;

fn check_nonnegative_amount(amount: i128) {
    if amount < 0 {
        panic!("negative amount is not allowed: {}", amount)
    }
}

// Bir hesabın dondurulup dondurulmadığını kontrol eden yardımcı fonksiyon
fn is_account_frozen(e: &Env, account: &Address) -> bool {
    let key = DataKey::Frozen(account.clone());
    e.storage().instance().get::<_, bool>(&key).unwrap_or(false)
}

// Özel olayları yayınlamak için yardımcı fonksiyon
fn emit_custom_event(e: &Env, event_type: &str, admin: Address, account: Address) {
    e.events().publish((event_type, admin, account), ());
}

#[contract]
pub struct Token;

#[contractimpl]
impl Token {
    pub fn initialize(e: Env, admin: Address, decimal: u32, name: String, symbol: String) {
        if has_administrator(&e) {
            panic!("already initialized")
        }
        write_administrator(&e, &admin);
        if decimal > u8::MAX.into() {
            panic!("Decimal must fit in a u8");
        }

        write_metadata(
            &e,
            TokenMetadata {
                decimal,
                name,
                symbol,
            },
        )
    }

    pub fn mint(e: Env, to: Address, amount: i128) {
        check_nonnegative_amount(amount);
        let admin = read_administrator(&e);
        admin.require_auth();

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        receive_balance(&e, to.clone(), amount);
        TokenUtils::new(&e).events().mint(admin, to, amount);
    }

    pub fn set_admin(e: Env, new_admin: Address) {
        let admin = read_administrator(&e);
        admin.require_auth();

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        write_administrator(&e, &new_admin);
        TokenUtils::new(&e).events().set_admin(admin, new_admin);
    }

    // Bir hesabı dondur (sadece yönetici yapabilir)
    pub fn freeze_account(e: Env, account: Address) {
        // Sadece yönetici hesapları dondurabilir
        let admin = read_administrator(&e);
        admin.require_auth();

        // Kontrat örneğinin TTL süresini uzat
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        // Hesabı dondurulmuş olarak ayarla
        let key = DataKey::Frozen(account.clone());
        e.storage().instance().set(&key, &true);

        // Dondurma olayını yayınla
        emit_custom_event(&e, "freeze_account", admin, account);
    }

    // Bir hesabın dondurulmasını kaldır (sadece yönetici yapabilir)
    pub fn unfreeze_account(e: Env, account: Address) {
        // Sadece yönetici hesapların dondurulmasını kaldırabilir
        let admin = read_administrator(&e);
        admin.require_auth();

        // Kontrat örneğinin TTL süresini uzat
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        // Dondurulmuş durumu kaldır
        let key = DataKey::Frozen(account.clone());
        e.storage().instance().remove(&key);

        // Dondurma kaldırma olayını yayınla
        emit_custom_event(&e, "unfreeze_account", admin, account);
    }

    // Kullanıcıya kahve puanı ekler (sadece yönetici)
    pub fn add_coffee_point(e: Env, account: Address) {
        let admin = read_administrator(&e);
        admin.require_auth();

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        // Mevcut puanları oku ve 1 ekle
        let current_points = read_coffee_points(&e, &account);
        let new_points = current_points + 1;
        write_coffee_points(&e, &account, new_points);

        // Olay yayınla
        e.events()
            .publish(("add_coffee_point", admin, account), new_points);
    }

    // 10 puana ulaşıldığında ücretsiz kahve hakkı verir (sadece yönetici)
    pub fn check_free_coffee(e: Env, account: Address) {
        let admin = read_administrator(&e);
        admin.require_auth();

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        let points = read_coffee_points(&e, &account);
        if points >= 10 {
            // 10 puanı sil ve 1 ücretsiz kahve hakkı ekle
            write_coffee_points(&e, &account, points - 10);
            let current_free_coffees = read_free_coffee(&e, &account);
            write_free_coffee(&e, &account, current_free_coffees + 1);

            // Olay yayınla
            e.events()
                .publish(("free_coffee_granted", admin, account), 1);
        } else {
            panic!("Not enough points: {}", points);
        }
    }

    // Kullanıcı ücretsiz kahve hakkını kullanır (sadece yönetici doğrulayabilir)
    pub fn redeem_free_coffee(e: Env, account: Address) {
        let admin = read_administrator(&e);
        admin.require_auth();

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        let free_coffees = read_free_coffee(&e, &account);
        if free_coffees > 0 {
            // 1 ücretsiz kahve hakkını sil
            write_free_coffee(&e, &account, free_coffees - 1);

            // Olay yayınla
            e.events()
                .publish(("free_coffee_redeemed", admin, account), 1);
        } else {
            panic!("No free coffee available");
        }
    }
}

#[contractimpl]
impl token::Interface for Token {
    fn allowance(e: Env, from: Address, spender: Address) -> i128 {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_allowance(&e, from, spender).amount
    }

    fn approve(e: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32) {
        from.require_auth();

        check_nonnegative_amount(amount);

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        write_allowance(&e, from.clone(), spender.clone(), amount, expiration_ledger);
        TokenUtils::new(&e)
            .events()
            .approve(from, spender, amount, expiration_ledger);
    }

    fn balance(e: Env, id: Address) -> i128 {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_balance(&e, id)
    }

    fn transfer(e: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        check_nonnegative_amount(amount);

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        // Göndericinin hesabı dondurulmuş mu kontrol et
        if is_account_frozen(&e, &from) {
            panic!("Hesap dondurulmuş ve token transfer edilemez");
        }

        // Transferi gerçekleştir
        spend_balance(&e, from.clone(), amount);
        receive_balance(&e, to.clone(), amount);
        TokenUtils::new(&e).events().transfer(from, to, amount);
    }

    fn transfer_from(e: Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();

        check_nonnegative_amount(amount);

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        // Göndericinin hesabı dondurulmuş mu kontrol et
        if is_account_frozen(&e, &from) {
            panic!("Hesap dondurulmuş ve token transfer edilemez");
        }

        // Transferi gerçekleştir
        spend_allowance(&e, from.clone(), spender, amount);
        spend_balance(&e, from.clone(), amount);
        receive_balance(&e, to.clone(), amount);
        TokenUtils::new(&e).events().transfer(from, to, amount)
    }

    fn burn(e: Env, from: Address, amount: i128) {
        from.require_auth();

        check_nonnegative_amount(amount);

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        // Göndericinin hesabı dondurulmuş mu kontrol et
        if is_account_frozen(&e, &from) {
            panic!("Hesap dondurulmuş ve token yakılamaz");
        }

        // Yakma işlemini gerçekleştir
        spend_balance(&e, from.clone(), amount);
        TokenUtils::new(&e).events().burn(from, amount);
    }

    fn burn_from(e: Env, spender: Address, from: Address, amount: i128) {
        spender.require_auth();

        check_nonnegative_amount(amount);

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        // Göndericinin hesabı dondurulmuş mu kontrol et
        if is_account_frozen(&e, &from) {
            panic!("Hesap dondurulmuş ve token yakılamaz");
        }

        // Yakma işlemini gerçekleştir
        spend_allowance(&e, from.clone(), spender, amount);
        spend_balance(&e, from.clone(), amount);
        TokenUtils::new(&e).events().burn(from, amount)
    }

    fn decimals(e: Env) -> u32 {
        read_decimal(&e)
    }

    fn name(e: Env) -> String {
        read_name(&e)
    }

    fn symbol(e: Env) -> String {
        read_symbol(&e)
    }
}
