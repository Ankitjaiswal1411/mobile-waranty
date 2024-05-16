use soroban_sdk::{contractimpl, Address, Env, BytesN, Vec, Val, String, panic_with};

#[derive(Clone)]
pub struct Warranty {
    imei: String,
    purchase_date: u64, // Unix timestamp
    warranty_period: u64, // Seconds
    active: bool,
}

#[contractimpl]
impl WarrantyContract {
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().has(&admin) {
            panic_with(env, "Contract already initialized");
        }
        env.storage().set(&admin, &()); 
    }

    pub fn register_warranty(env: Env, imei: String, purchase_date: u64, warranty_period: u64) {
        let admin = env.storage().get(&Address::generate_from_bytes(b"admin")).unwrap();
        if env.invoker() != admin {
            panic_with(env, "Only admin can register warranties");
        }

        if env.storage().has(&imei) {
            panic_with(env, "Warranty already registered for this IMEI");
        }

        let warranty = Warranty {
            imei,
            purchase_date,
            warranty_period,
            active: true,
        };
        env.storage().set(&warranty.imei, &warranty);
    }

    pub fn check_warranty(env: Env, imei: String) -> bool {
        let warranty = env.storage().get(&imei).unwrap_or_else(|| panic_with(env, "Warranty not found for this IMEI"));

        let expiration_date = warranty.purchase_date + warranty.warranty_period;
        expiration_date > env.ledger().timestamp() && warranty.active
    }

    pub fn deactivate_warranty(env: Env, imei: String) {
        let admin = env.storage().get(&Address::generate_from_bytes(b"admin")).unwrap();
        if env.invoker() != admin {
            panic_with(env, "Only admin can deactivate warranties");
        }

        let mut warranty = env.storage().get(&imei).unwrap_or_else(|| panic_with(env, "Warranty not found for this IMEI"));
        warranty.active = false;
        env.storage().set(&warranty.imei, &warranty);
    }

    pub fn extend_warranty(env: Env, imei: String, additional_period: u64) {
        let admin = env.storage().get(&Address::generate_from_bytes(b"admin")).unwrap();
        if env.invoker() != admin {
            panic_with(env, "Only admin can extend warranties");
        }
        
        let mut warranty = env.storage().get(&imei).unwrap_or_else(|| panic_with(env, "Warranty not found for this IMEI"));

        if !warranty.active {
            panic_with(env, "Cannot extend inactive warranty");
        }
        warranty.warranty_period += additional_period;
        env.storage().set(&warranty.imei, &warranty);
    }

}

