use data_structures_and_algorithms_in_rust::_4_basic_data_structures::deque::pal_checker;

#[test]
fn pal_checker_test() {
    let pal = "rust";
    assert_eq!(false, pal_checker(pal));

    let pal = "russur";
    assert_eq!(true, pal_checker(pal));

    let pal = "radar";
    assert_eq!(true, pal_checker(pal));

    let pal = "sos";
    assert_eq!(true, pal_checker(pal));

    let pal = "rustsur";
    assert_eq!(true, pal_checker(pal));
}
