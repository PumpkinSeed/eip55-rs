use sha3::{Keccak256, Digest};

pub fn validate_address(address: String) {
    let mut hasher = Keccak256::new();
    hasher.update(address);
    let result = hasher.finalize();

    //let lofasz = std::str:from_utf8(result).unwrap();
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_address() {
        validate_address(String::from("asd"));
    }
}
