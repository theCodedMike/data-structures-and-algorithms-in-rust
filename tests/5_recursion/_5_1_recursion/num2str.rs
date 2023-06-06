use data_structures_and_algorithms_in_rust::_5_recursion::{num2str_rec, num2str_stk};

#[test]
fn num2str_cur_test() {
    let num = 100;
    let sb = num2str_rec(num, 2);
    let so = num2str_rec(num, 8);
    let sh = num2str_rec(num, 16);

    assert_eq!(sb, "1100100".to_string());
    assert_eq!(so, "144".to_string());
    assert_eq!(sh, "64".to_string());
}

#[test]
fn num2str_stk_test() {
    let num = 100;
    let sb = num2str_stk(num, 2);
    let so = num2str_stk(num, 8);
    let sh = num2str_stk(num, 16);

    assert_eq!(sb, "1100100".to_string());
    assert_eq!(so, "144".to_string());
    assert_eq!(sh, "64".to_string());
}
