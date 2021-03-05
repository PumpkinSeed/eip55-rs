use crypto::digest::Digest;
use crypto::sha3::Sha3;

const PREFIX: &str = "0x";

pub fn checksum(address: &str) -> String {
    let stripped = String::from(address.to_ascii_lowercase().trim_start_matches(PREFIX));
    let mut keccak = Sha3::keccak256();
    keccak.input_str(&stripped);
    let hash = keccak.result_str();

    let mut checksum = String::new();
    checksum.push_str(PREFIX);

    for (pos, char) in hash.chars().enumerate() {
        if pos > 39 {
            break;
        }
        if u32::from_str_radix(&char.to_string()[..], 16).unwrap() > 7 {
            checksum.push_str(&address[pos+2..pos+3].to_ascii_uppercase());
        } else {
            checksum.push_str(&address[pos+2..pos+3]);
        }
    }

    checksum
}

pub fn validate_address(address: &str) -> bool {
    checksum(address) == address
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_address() {
        // Base
        assert_eq!(validate_address("0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed"), true);
        // All caps
        assert_eq!(validate_address("0x52908400098527886E0F7030069857D2E4169EE7"), true);
        assert_eq!(validate_address("0x8617E340B3D01FA5F11F306F4090FD50E238070D"), true);
        // All Lower
        assert_eq!(validate_address("0xde709f2102306220921060314715629080e2fb77"), true);
        assert_eq!(validate_address("0x27b1fdb04752bbc536007a920d24acb045561c26"), true);
        // Normal
        assert_eq!(validate_address("0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed"), true);
        assert_eq!(validate_address("0xfB6916095ca1df60bB79Ce92cE3Ea74c37c5d359"), true);
        assert_eq!(validate_address("0xdbF03B407c01E7cD3CBea99509d93f8DDDC8C6FB"), true);
        assert_eq!(validate_address("0xD1220A0cf47c7B9Be7A2E6BA89F429762e7b9aDb"), true);

        // False
        assert_eq!(validate_address("0xD1220a0cf47c7B9Be7A2E6BA89F429762e7b9aDb"), false);
        assert_eq!(validate_address("0xdbF03B407c01e7cD3CBea99509d93f8DDDC8C6FB"), false);
        assert_eq!(validate_address("0xfb6916095ca1df60bB79Ce92cE3Ea74c37c5D359"), false);
        assert_eq!(validate_address("0x5aAeb6053f3E94C9b9A09f33669435E7Ef1BeAed"), false);
    }
}
