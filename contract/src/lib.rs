use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env,near_bindgen};

const PUZZLE_NUMBER: u8 = 1;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract{
    cw_solution: String,
}

#[near_bindgen]
impl Contract{
    #[init]
    pub fn new(solution: String) -> Self {
        Self {
            cw_solution: solution,
        }
    }

    pub fn get_puzzle_number(&self) -> u8{
        PUZZLE_NUMBER
    }

    pub fn set_solution(&mut self, solution: String){
        self.cw_solution = solution;
    }

    pub fn get_solution(&self) -> String{
        self.cw_solution.clone()
    }

    pub fn guess_solution(&mut self, solution: String) -> bool{
        // let hashed_input = env::sha256(solution.as_bytes());
        // let hashed_input_hex = hex::encode(&hashed_input);

        if self.cw_solution == solution {
            env::log_str("Guess Right!");
            true
        } else {
            env::log_str("Guess Wrong.");
            false
        }
    }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

 #[cfg(test)]
 mod tests{
     use super::*;
     use near_sdk::test_utils::{get_logs, VMContextBuilder};
     use near_sdk::{testing_env, AccountId};

     //Test hash
     #[test]
     fn debug_get_hash(){
         // Basic set up for a unit test
         testing_env!(VMContextBuilder::new().build());

         //Unit test
         let debug_solution = "near nomiron ref finance";
         //Hash
         let debug_hash_bytes = env::sha256(debug_solution.as_bytes());
         //Encode
         let debug_hash_string = hex::encode(debug_hash_bytes);
         println!("Hash string: {:?}",debug_hash_string);
     }

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
     fn get_context(predecessor: AccountId) -> VMContextBuilder{
         let mut builder = VMContextBuilder::new();
         builder.predecessor_account_id(predecessor);
         builder
     }
 }