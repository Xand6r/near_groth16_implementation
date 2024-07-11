use eth_encode_packed::ethabi::ethereum_types::{U128, U256};

use crate::constants::{CONTROL_ROOT, ZERO_BYTES_STRING};

pub fn get_zero_bytes() -> Vec<u8> {
    let zero_bytes = hex::decode(ZERO_BYTES_STRING).unwrap();

    zero_bytes
}

pub fn hex_to_u256(hex_string: &str) -> U256 {
    U256::from_str_radix(hex_string, 16).unwrap()
}

/// change from u256 to u128 by taking the last 128 bytes
pub fn u256_to_u128(value: U256) -> U128 {
    let U256(ref arr) = value;

    let mut ret = [0; 2];
    ret[0] = arr[0];
    ret[1] = arr[1];

    U128(ret)
}

pub fn reverse_byte_order_uint256(input: U256) -> U256 {
    let mut v = input;

    // swap bytes
    v = ((v & hex_to_u256("0xFF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00"))
        >> 8)
        | ((v & hex_to_u256("0x00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF"))
            << 8);

    // swap 2-byte long pairs
    v = ((v & hex_to_u256("0xFFFF0000FFFF0000FFFF0000FFFF0000FFFF0000FFFF0000FFFF0000FFFF0000"))
        >> 16)
        | ((v & hex_to_u256("0x0000FFFF0000FFFF0000FFFF0000FFFF0000FFFF0000FFFF0000FFFF0000FFFF"))
            << 16);

    // swap 4-byte long pairs
    v = ((v & hex_to_u256("0xFFFFFFFF00000000FFFFFFFF00000000FFFFFFFF00000000FFFFFFFF00000000"))
        >> 32)
        | ((v & hex_to_u256("0x00000000FFFFFFFF00000000FFFFFFFF00000000FFFFFFFF00000000FFFFFFFF"))
            << 32);

    // swap 8-byte long pairs
    v = ((v & hex_to_u256("0xFFFFFFFFFFFFFFFF0000000000000000FFFFFFFFFFFFFFFF0000000000000000"))
        >> 64)
        | ((v & hex_to_u256("0x0000000000000000FFFFFFFFFFFFFFFF0000000000000000FFFFFFFFFFFFFFFF"))
            << 64);

    // swap 16-byte long pairs
    v = (v >> 128) | (v << 128);

    v
}

// reverse the order of the bytes and return it as a tuple of 128 bits which constitute the original 256 bit input
pub fn split_digest(digest: Vec<u8>) -> (Vec<u8>, Vec<u8>) {
    let digest = hex_to_u256(&hex::encode(digest));
    let reversed = reverse_byte_order_uint256(digest);

    let a = u256_to_u128(reversed).as_u128().to_be_bytes().to_vec();
    let b = u256_to_u128(reversed >> 128)
        .as_u128()
        .to_be_bytes()
        .to_vec();

    (a, b)
}

pub fn get_sequence(vector: &[u8], index: usize) -> Option<String> {
    // Calculate the start and end indices for the slice
    let start = index * 32;
    let end = start + 32;

    // Check if the calculated end index is within bounds
    if start < vector.len() && end <= vector.len() {
        Some(hex::encode(&vector[start..end]))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {

    use crate::utils::{hex_to_u256, reverse_byte_order_uint256, split_digest};

    #[test]
    fn test_reverse_byte() {
        let bytes_reversed = reverse_byte_order_uint256(hex_to_u256(
            "5f8ad5ce1d0fc577d04d618880b8c77c9aced63740ca43b708f32425a95b11b7",
        ));
        assert_eq!(
            bytes_reversed,
            hex_to_u256("b7115ba92524f308b743ca4037d6ce9a7cc7b88088614dd077c50f1dced58a5f")
        )
    }

    #[test]
    fn test_split_digest() {
        let input_digest =
            hex::decode("5f8ad5ce1d0fc577d04d618880b8c77c9aced63740ca43b708f32425a95b11b7")
                .unwrap();
        let (first_half, second_half) = split_digest(input_digest);

        assert_eq!(hex::encode(first_half),"7cc7b88088614dd077c50f1dced58a5f");
        assert_eq!(hex::encode(second_half),"b7115ba92524f308b743ca4037d6ce9a");
    }
}
