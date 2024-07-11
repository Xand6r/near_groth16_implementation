use hex::encode;
use near_bigint::U256;
use near_groth16_verifier::{G1Point, G2Point, Proof};

use crate::{
    constants::{self, CONTROL_ROOT},
    utils::{get_sequence, split_digest},
    verifier::get_verifier,
};

pub fn decode_seal_abi(seal: Vec<u8>) -> Proof {
    let seal = (&seal[4..]).to_vec();

    let a = G1Point {
        x: U256::from_str_radix(&get_sequence(&seal, 0).unwrap(), 16).unwrap(),
        y: U256::from_str_radix(&get_sequence(&seal, 1).unwrap(), 16).unwrap(),
    };
    let b = G2Point {
        x: [
            U256::from_str_radix(&get_sequence(&seal, 2).unwrap(), 16).unwrap(),
            U256::from_str_radix(&get_sequence(&seal, 3).unwrap(), 16).unwrap(),
        ],
        y: [
            U256::from_str_radix(&get_sequence(&seal, 4).unwrap(), 16).unwrap(),
            U256::from_str_radix(&get_sequence(&seal, 5).unwrap(), 16).unwrap(),
        ],
    };
    let c = G1Point {
        x: U256::from_str_radix(&get_sequence(&seal, 6).unwrap(), 16).unwrap(),
        y: U256::from_str_radix(&get_sequence(&seal, 7).unwrap(), 16).unwrap(),
    };

    let proof = Proof { a, b, c };

    proof
}

pub fn verify_integrity(seal: Vec<u8>, claim_digest: Vec<u8>)-> bool {
    let radix = 16;
    let (claim0, claim1) = split_digest(claim_digest);
    let (CONTROL_ROOT_0, CONTROL_ROOT_1) = split_digest(hex::decode(CONTROL_ROOT).unwrap());

    let CONTROL_ROOT_0 = u128::from_str_radix(&hex::encode(CONTROL_ROOT_0), radix).unwrap();
    let CONTROL_ROOT_1 = u128::from_str_radix(&hex::encode(CONTROL_ROOT_1), radix).unwrap();
    let claim0 = u128::from_str_radix(&hex::encode(claim0), radix).unwrap();
    let claim1 = u128::from_str_radix(&hex::encode(claim1), radix).unwrap();
    let BN254_CONTROL_ID = U256::from_str_radix(constants::BN254_CONTROL_ID, radix).unwrap();

    let proof = decode_seal_abi(seal);

    //perform the actual verification here
    let verifier = get_verifier();

    let pub_signals = vec![
        U256::from_dec_str(&format!("{}", CONTROL_ROOT_0)).unwrap(),
        U256::from_dec_str(&format!("{}", CONTROL_ROOT_1)).unwrap(),
        U256::from_dec_str(&format!("{}", claim0)).unwrap(),
        U256::from_dec_str(&format!("{}", claim1)).unwrap(),
        BN254_CONTROL_ID,
    ];

    let verified = verifier.verify(pub_signals, proof);
    verified
    // println!("{:?}", pub_signals);
}

#[cfg(test)]
mod tests {
    use easy_hasher::easy_hasher::raw_sha256;
    use eth_encode_packed::ethabi::{encode, Token};
    use near_bigint::U256;
    use near_groth16_verifier::{G1Point, G2Point};

    use crate::{
        constants::{IMAGE_ID_STRING, JOURNAL_OUTPUT_STRING, SEAL_STRING},
        near::get_sequence,
        risczero::{Receipt, ReceiptClaim},
    };

    use super::{decode_seal_abi, verify_integrity};

    #[test]
    fn test_receipt_digest() {
        let image_id = hex::decode(IMAGE_ID_STRING).unwrap();
        let seal = hex::decode(SEAL_STRING).unwrap();
        let journal_output = hex::decode(JOURNAL_OUTPUT_STRING).unwrap();

        let journal_digest = raw_sha256(encode(&[Token::Bytes(journal_output)])).to_vec();

        let receipt = Receipt {
            seal: seal.clone(),
            claim: ReceiptClaim::ok(image_id, journal_digest),
        };
        let claim_digest = receipt.claim.digest();

        verify_integrity(seal, claim_digest);
    }

    #[test]
    fn test_decode_seal_abi() {
        let seal = hex::decode(SEAL_STRING).unwrap();
        let seal = (&seal[4..]).to_vec();

        let x = U256::from_str_radix(&get_sequence(&seal, 4).unwrap(), 16).unwrap();
        let y = U256::from_str_radix(&get_sequence(&seal, 5).unwrap(), 16).unwrap();

        let a = G1Point {
            x: U256::from_str_radix(&get_sequence(&seal, 0).unwrap(), 16).unwrap(),
            y: U256::from_str_radix(&get_sequence(&seal, 1).unwrap(), 16).unwrap(),
        };
        let b = G2Point {
            x: [
                U256::from_dec_str(
                    "11559732032986387107991004021392285783925812861821192530917403151452391805634",
                )
                .unwrap(),
                U256::from_dec_str(
                    "10857046999023057135944570762232829481370756359578518086990519993285655852781",
                )
                .unwrap(),
            ],
            y: [
                U256::from_dec_str(
                    "4082367875863433681332203403145435568316851327593401208105741076214120093531",
                )
                .unwrap(),
                U256::from_dec_str(
                    "8495653923123431417604973247489272438418190587263600148770280649306958101930",
                )
                .unwrap(),
            ],
        };

        println!("{}", x);
        println!("{}", y);
        // let _ = decode_seal_abi(seal);
    }
}
