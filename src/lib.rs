use std::default;

// use constants::{IMAGE_ID_STRING, JOURNAL_OUTPUT_STRING, SEAL_STRING};
// use easy_hasher::easy_hasher::raw_sha256;
// use eth_encode_packed::ethabi::{encode, Token};
// use near::verify_integrity;
// use near_groth16_verifier::G1Point;
// Find all our documentation at https://docs.near.org
use near_sdk::{log, near_bindgen, PanicOnDefault};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
// use risczero::{Receipt, ReceiptClaim};
// use near_bigint::U256;


// pub mod verifier;
// pub mod risczero;
// pub mod utils;
pub mod constants;
// pub mod near;


// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
}

// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        // G1Point
        Self {
        }
    }
}


// Implement the contract structure
#[near_bindgen]
impl Contract {
    // Public method - returns the greeting saved, defaulting to DEFAULT_GREETING
    pub fn get_greeting(&self) -> String {
        "self.greeting".to_string().clone()
    }

    // // Public method - returns the greeting saved, defaulting to DEFAULT_GREETING
    // pub fn verify(&self) -> bool {
    //     let image_id = hex::decode(IMAGE_ID_STRING).unwrap();
    //     let seal = hex::decode(SEAL_STRING).unwrap();
    //     let journal_output = hex::decode(JOURNAL_OUTPUT_STRING).unwrap();

    //     let journal_digest = raw_sha256(encode(&[Token::Bytes(journal_output)])).to_vec();

    //     let receipt = Receipt {
    //         seal: seal.clone(),
    //         claim: ReceiptClaim::ok(image_id, journal_digest),
    //     };
    //     let claim_digest = receipt.claim.digest();

    //     verify_integrity(seal, claim_digest)
    // }

}
