use sha3::{Digest, Keccak256};

const PREFIX: &str = "0x";

/// Returns the mixed-case checksum address encoding.
///
/// Output includes prefix.
///
/// # Example
/// Address may be passed with or without prefix:
/// ```
/// # use eip55::checksum;
/// // With "0x" prefix.
/// let with_checksum = checksum("0xfb6916095ca1df60bb79ce92ce3ea74c37c5d359");
/// assert_eq!(with_checksum, "0xfB6916095ca1df60bB79Ce92cE3Ea74c37c5d359");
///
/// // Without prefix.
/// let with_checksum = checksum("fb6916095ca1df60bb79ce92ce3ea74c37c5d359");
/// assert_eq!(with_checksum, "0xfB6916095ca1df60bB79Ce92cE3Ea74c37c5d359");
/// ```
pub fn checksum(address: &str) -> String {
    let trimmed = address.trim_start_matches(PREFIX);
    let stripped = String::from(trimmed.to_ascii_lowercase());

    let mut hasher = Keccak256::new();
    hasher.update(stripped);
    let hash_vec = hasher.finalize().to_vec();
    let hash = hex::encode(hash_vec);

    let mut checksum = String::new();

    for (pos, char) in hash.chars().enumerate() {
        if pos > 39 {
            break;
        }
        if u32::from_str_radix(&char.to_string()[..], 16).unwrap() > 7 {
            checksum.push_str(&trimmed[pos..pos+1].to_ascii_uppercase());
        } else {
            checksum.push_str(&trimmed[pos..pos+1].to_ascii_lowercase());
        }
    }
    format!("0x{}", checksum)
}

#[test]
fn checksum_no_prefix() {
    let input = "fb6916095ca1df60bb79ce92ce3ea74c37c5d359";
    let target = "0xfB6916095ca1df60bB79Ce92cE3Ea74c37c5d359";
    let output = checksum(input);
    assert_eq!(output, target);
}
#[test]
fn checksums() {
    let target = "0xfB6916095ca1df60bB79Ce92cE3Ea74c37c5d359";
    assert_eq!(target, checksum(&target.to_lowercase()));
    let all_caps_1 = "0x52908400098527886E0F7030069857D2E4169EE7";
    assert_eq!(all_caps_1, checksum(&all_caps_1.to_lowercase()));
    let all_caps_2 = "0x8617E340B3D01FA5F11F306F4090FD50E238070D";
    assert_eq!(all_caps_2, checksum(&all_caps_2.to_lowercase()));
    let all_lower_1 = "0xde709f2102306220921060314715629080e2fb77";
    assert_eq!(all_lower_1, checksum(&all_lower_1.to_lowercase()));
    let all_lower_2 = "0x27b1fdb04752bbc536007a920d24acb045561c26";
    assert_eq!(all_lower_2, checksum(&all_lower_2.to_lowercase()));
    let normal_1 = "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed";
    assert_eq!(normal_1, checksum(&normal_1.to_lowercase()));
    let normal_2 = "0xfB6916095ca1df60bB79Ce92cE3Ea74c37c5d359";
    assert_eq!(normal_2, checksum(&normal_2.to_lowercase()));
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

        assert_eq!(validate_address("0x000000000000000000000000000000000000dEAD"), false);
        assert_eq!(validate_address("0x000000000000000000000000000000000000dEaD"), true);
    }
}
