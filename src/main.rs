use ink_lang as ink;

#[ink::contract]
mod my_contract {
    #[ink(storage)]
    pub struct MyContract {
        value: i32,
    }

    impl MyContract {
        #[ink(constructor)]
        pub fn new(initial_value: i32) -> Self {
            Self { value: initial_value }
        }

        #[ink(message)]
        pub fn get(&self) -> i32 {
            self.value
        }

        #[ink(message)]
        pub fn set(&mut self, new_value: i32) {
            self.value = new_value;
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn test_default() {
            let my_contract = MyContract::new(42);
            assert_eq!(my_contract.get(), 42);
        }

        #[ink::test]
        fn test_set() {
            let mut my_contract = MyContract::new(0);
            my_contract.set(100);
            assert_eq!(my_contract.get(), 100);
        }
    }
}
