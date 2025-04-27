use crate::storage_types::DataKey;
use soroban_sdk::{Address, Env};

// Kullanıcının kahve puanlarını okur
pub fn read_coffee_points(e: &Env, account: &Address) -> i128 {
    let key = DataKey::CoffeePoints(account.clone());
    e.storage().instance().get(&key).unwrap_or(0)
}

// Kullanıcının kahve puanlarını günceller
pub fn write_coffee_points(e: &Env, account: &Address, points: i128) {
    let key = DataKey::CoffeePoints(account.clone());
    e.storage().instance().set(&key, &points);
}

// Kullanıcının ücretsiz kahve haklarını okur
pub fn read_free_coffee(e: &Env, account: &Address) -> i128 {
    let key = DataKey::FreeCoffee(account.clone());
    e.storage().instance().get(&key).unwrap_or(0)
}

// Kullanıcının ücretsiz kahve haklarını günceller
pub fn write_free_coffee(e: &Env, account: &Address, count: i128) {
    let key = DataKey::FreeCoffee(account.clone());
    e.storage().instance().set(&key, &count);
}
