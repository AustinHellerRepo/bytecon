#[cfg(test)]
mod byte_converter_tests {
    use bytecon::ByteConverter;


    #[test]
    fn test_e8v5_string() {
        let string = String::from("test value");
        let cloned_string = string.clone_via_bytes().unwrap();
        assert_eq!(string, cloned_string);
    }
}