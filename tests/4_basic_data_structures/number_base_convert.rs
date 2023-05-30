use data_structures_and_algorithms_in_rust::_4_basic_data_structures::{
    base_converter, divide_by_two,
};

#[test]
fn divide_by_two_test() {
    let bin_str = divide_by_two(10);
    assert_eq!("1010", &bin_str);
    let bin_str = divide_by_two(21);
    assert_eq!("10101", &bin_str);
    let bin_str = divide_by_two(320);
    assert_eq!("101000000", &bin_str);
}

#[test]
fn base_converter_test() {
    let dec = 4582;
    let bin_str = base_converter(dec, 2);
    assert_eq!("1000111100110", &bin_str);

    let oct_str = base_converter(dec, 8);
    assert_eq!("10746", &oct_str);

    let dec_str = base_converter(dec, 10);
    assert_eq!("4582", &dec_str);

    let hex_str = base_converter(dec, 16);
    assert_eq!("11E6", &hex_str);
}
