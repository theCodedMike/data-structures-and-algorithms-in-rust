use data_structures_and_algorithms_in_rust::_10_exercise::_1_edit_distance::hamming_distance::{
    hamming_distance_of_number, hamming_distance_of_number2, hamming_distance_of_str,
};

/// cargo test -- --show-output test_hamming_distance_of_number
#[test]
fn test_hamming_distance_of_number() {
    let source = 1;
    let target = 2;
    println!("{}: {:08b}", source, source); // 1: 00000001
    println!("{}: {:08b}", target, target); // 2: 00000010
    let distance = hamming_distance_of_number(source, target);
    assert_eq!(distance, 2);

    let source = 1000;
    let target = 269;
    println!("{}: {:016b}", source, source); // 1000: 0000001111101000
    println!("{}: {:016b}", target, target); //  269: 0000000100001101
    let distance = hamming_distance_of_number(source, target);
    assert_eq!(distance, 6);
}

/// cargo test -- --show-output test_hamming_distance_of_number2
#[test]
fn test_hamming_distance_of_number2() {
    let source = 1;
    let target = 2;
    println!("{}: {:08b}", source, source); // 1: 00000001
    println!("{}: {:08b}", target, target); // 2: 00000010
    let distance = hamming_distance_of_number2(source, target);
    assert_eq!(distance, 2);

    let source = 1000;
    let target = 269;
    println!("{}: {:016b}", source, source); // 1000: 0000001111101000
    println!("{}: {:016b}", target, target); //  269: 0000000100001101
    let distance = hamming_distance_of_number2(source, target);
    assert_eq!(distance, 6);
}

/// cargo test test_hamming_distance_of_str
#[test]
fn test_hamming_distance_of_str() {
    let source = "abce";
    let target = "edcf";
    let distance = hamming_distance_of_str(source, target);
    assert_eq!(distance, Some(3));

    let source = "abce";
    let target = "edf";
    let distance = hamming_distance_of_str(source, target);
    assert_eq!(distance, None);
}
