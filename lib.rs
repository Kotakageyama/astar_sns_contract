#![cfg_attr(not(feature = "std"), no_std)]

mod follow;
mod message;
mod metadata;
mod post;
mod profile;

use ink_lang as ink;

#[ink::contract]
mod astar_sns_contract {

    use core::default::default;

    use ink_env::debug_println;
    use ink_env::AccountId;
    use ink_lang::codegen::Env;
    use ink_lang_ir::Message;
    use ink_prelude::vec::Vec;
    use metadata::Profile;
    use openbrush::storage::Mapping;
    use openbrush::test_utils::accounts;

    pub use crate::follow::*;
    pub use crate::message::*;
    pub use crate::metadata::*;
    pub use crate::post::*;

    // コントラクトに保存するデータを定義
    #[ink(storage)]
    pub struct AstarSnsContract {
        pub profile_map: Mapping<AccountId, Profile>,
        pub post_map: Mapping<u128, Post>,
        pub post_map_counter: u128,
        pub message_list_map: Mapping<u128, Vec<Message>>,
        pub message_list_map_counter: u128,
    }

    impl AstarSnsContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                profile_map: Maaping::default(),
                post_map: Maaping::default(),
                post_map_counter: 0,
                message_list_map: Maaping::default(),
                message_list_map_counter: 0,
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new()
        }

        #[ink(message)]
        pub fn debug(&self) {
            debug_println!("hello world!");
        }
    }
}
