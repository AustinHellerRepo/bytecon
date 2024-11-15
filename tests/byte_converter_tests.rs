#[cfg(test)]
mod byte_converter_tests {
    use std::path::PathBuf;

    use bytecon::ByteConverter;


    #[test]
    fn test_e8v5_string() {
        let string = String::from("test value");
        let cloned_string = string.clone_via_bytes().unwrap();
        assert_eq!(string, cloned_string);
    }

    #[test]
    fn test_t3b1_pathbuf() {
        let obj = PathBuf::from("/home/path/here");
        let cloned_obj = obj.clone_via_bytes().unwrap();
        assert_eq!(obj, cloned_obj);
    }
}