#[cfg(test)]
mod byte_converter_tests {
    use std::path::PathBuf;

    use bytecon::ByteConverter;
    use rand::SeedableRng;
    use rand_chacha::{ChaCha20Rng, ChaCha8Rng};


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
    
    #[test]
    fn test_p2b7_array() {
        let obj = [1, 2, 3];
        let cloned_obj = obj.clone_via_bytes().unwrap();
        assert_eq!(obj, cloned_obj);
    }

    #[test]
    fn test_t4l8_rand_chacha_8_entropy() {
        let obj = ChaCha8Rng::from_entropy();
        let cloned_obj = obj.clone_via_bytes().unwrap();
        assert_eq!(obj, cloned_obj);
    }

    #[test]
    fn test_e5p1_rand_chacha_8_seeded() {
        let obj = ChaCha8Rng::from_seed([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32]);
        let cloned_obj = obj.clone_via_bytes().unwrap();
        assert_eq!(obj, cloned_obj);
    }

    #[test]
    fn test_u4x2_rand_chacha_20_entropy() {
        let obj = ChaCha20Rng::from_entropy();
        let cloned_obj = obj.clone_via_bytes().unwrap();
        assert_eq!(obj, cloned_obj);
    }

    #[test]
    fn test_k9q3_rand_chacha_20_seeded() {
        let obj = ChaCha20Rng::from_seed([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32]);
        let cloned_obj = obj.clone_via_bytes().unwrap();
        assert_eq!(obj, cloned_obj);
    }
}