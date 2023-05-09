use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };
use near_sdk::{ env, near_bindgen, AccountId };
use near_sdk::collections::UnorderedMap;

near_sdk::setup_alloc!();

// 1. Main Struct
// attributes - a declarative tag that is used to convey information to runtime about the behaviors of various elements like classes, methods, structures, enumerators, assemblies, etc.
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct KeyValue {
    pairs: UnorderedMap<String, String>,
}

// 2. Default Implementation
// every type in Rust has a Default implementation
impl Default for KeyValue {
    fn default() -> Self {
        Self {
            pairs: UnorderedMap::new(b"r".to_vec()),
        }
    }
}

// 3. Core Logic
#[near_bindgen]
impl KeyValue {
    pub fn create_update(&mut self, k: String, v: String) {
        env::log(b"created or updated");
        self.pairs.insert(&k, &v);
    }

    pub fn read(&self, k: String) -> Option<String> {
        env::log(b"read");
        return self.pairs.get(&k);
    }

    pub fn delete(&mut self, k: String) {
        env::log(b"delete");
        self.pairs.remove(&k);
    }
}

// 4. Tests
#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{ testing_env, VMContext };

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    fn create_read_pair() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = KeyValue::default();
        contract.create_update("first_key".to_string(), "hello".to_string());
        assert_eq!("hello".to_string(), contract.read("first_key".to_string()).unwrap());
    }

    #[test]
    fn rad_nonexistent_pair() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = KeyValue::default();
        assert_eq!(None, contract.read("first_key".to_string()));
    }



}