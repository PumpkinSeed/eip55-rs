 #[test]
 fn test_validate_address() {
     // Base
     assert_eq!(eip55::validate_address("0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed"), true);
     assert_eq!(eip55::checksum("0x5aaeb6053f3e94c9b9a09f33669435e7ef1beaed"), "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed")
}
