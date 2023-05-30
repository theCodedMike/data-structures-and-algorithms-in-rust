use data_structures_and_algorithms_in_rust::_4_basic_data_structures::{
    par_checker1, par_checker2, par_checker3,
};

#[test]
fn par_checker1_test() {
    let sa = "()(())";
    let sb = "()((()";
    assert_eq!(true, par_checker1(sa));
    assert_eq!(false, par_checker1(sb));
}

#[test]
fn par_checker2_test() {
    let sa = "{{([][])}()}";
    let sb = "(){)[}";
    assert_eq!(true, par_checker2(sa));
    assert_eq!(false, par_checker2(sb));
    let sa = "[[{{(())}}]]";
    let sb = "((()]))";
    assert_eq!(true, par_checker2(sa));
    assert_eq!(false, par_checker2(sb));
    let sa = "[][][](){}";
    let sb = "[{()]";
    assert_eq!(true, par_checker2(sa));
    assert_eq!(false, par_checker2(sb));
}

#[test]
fn par_checker3_test() {
    let sa = "(2+3){func}[abc]";
    let sb = "(2+3)*(3-1";
    assert_eq!(true, par_checker3(sa));
    assert_eq!(false, par_checker3(sb));
}
