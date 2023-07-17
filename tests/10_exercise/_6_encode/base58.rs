use data_structures_and_algorithms_in_rust::_10_exercise::_6_encode::base58::{
    DecodeError, Decoder, Encoder,
};

/// cargo test -- --show-output test_base58
#[test]
fn test_base58() {
    let res = "abc".encode_to_base58();
    println!("{}", res);
    assert_eq!(res, "ZiCa");

    let res = "ZiCa".decode_from_base58();
    let expected: Result<String, DecodeError> = Ok("abc".to_string());
    assert_eq!(res, expected);

    let res = "我爱你iloveu".encode_to_base58();
    println!("{}", res);
    assert_eq!(res, "7T5VrPqoBr9DeUXiUr2Fn");

    let res = "我爱你".encode_to_base58();
    println!("{}", res);
    assert_eq!(res, "3wCHf2LRNuMmh");
}
