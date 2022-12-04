#![cfg_attr(not(feature = "std"), no_std)]

mod metadata;

use ink_lang as ink;

#[ink::contract]
mod astar_sns_contract {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct AstarSnsContract {
        /// Stores a single `bool` value on the storage.
        value: bool,
        number: u64,
    }

    impl AstarSnsContract {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool, init_number: u64) -> Self {
            Self {
                value: init_value,
                number: init_number,
            }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default(), Default::default())
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        /// add the value to the number field
        #[ink(message)]
        pub fn add(&mut self, value: u64) {
            self.number += value;
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }

        /// コントラクトのnumberを取得します
        #[ink(message)]
        pub fn get_number(&self) -> u64 {
            self.number
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let astar_sns_contract = AstarSnsContract::default();
            assert_eq!(astar_sns_contract.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut astar_sns_contract = AstarSnsContract::new(false, 0);
            assert_eq!(astar_sns_contract.get(), false);
            astar_sns_contract.flip();
            assert_eq!(astar_sns_contract.get(), true);
        }
    }
}
