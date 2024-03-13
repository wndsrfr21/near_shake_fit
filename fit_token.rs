// SPDX-License-Identifier: MIT
#![cfg_attr(not(target_arch = "wasm32"), allow(unused_imports))]

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::env;
use near_sdk::ext_contract;
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::near_bindgen;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;
use near_sdk::Balance;

// Define the NEP-141 token contract.
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Token {
    owner_id: AccountId,
    balances: UnorderedMap<AccountId, Balance>,
    total_supply: Balance,
}

impl Default for Token {
    fn default() -> Self {
        panic!("Token should be initialized before usage")
    }
}

#[near_bindgen]
impl Token {
    #[init]
    pub fn new(owner_id: ValidAccountId, total_supply: U128) -> Self {
        let mut balances = UnorderedMap::new(b"b".to_vec());
        balances.insert(&owner_id.as_ref(), &total_supply.0);
        Self {
            owner_id: owner_id.into(),
            balances,
            total_supply: total_supply.0,
        }
    }

    pub fn transfer(&mut self, receiver_id: ValidAccountId, amount: U128) {
        let sender_id = env::predecessor_account_id();
        let sender_balance = self.get_balance(&sender_id);
        let receiver_balance = self.get_balance(&receiver_id.as_ref());

        assert!(sender_balance >= amount.0, "Not enough balance to transfer");

        self.balances.insert(&sender_id, &(sender_balance - amount.0));
        self.balances.insert(&receiver_id.as_ref(), &(receiver_balance + amount.0));
    }

    pub fn get_balance(&self, account_id: &AccountId) -> Balance {
        self.balances.get(account_id).unwrap_or(0)
    }

    pub fn total_supply(&self) -> U128 {
        U128(self.total_supply)
    }
}
