use data_structures_and_algorithms_in_rust::_4_basic_data_structures::stack::{
    infix_to_postfix, postfix_eval,
};

#[test]
fn infix_to_postfix_test() {
    let infix = "( A + B ) * C )";
    let postfix = infix_to_postfix(infix);
    assert_eq!(None, postfix);

    let infix = "A + B";
    let postfix = infix_to_postfix(infix);
    assert_eq!(Some(String::from("A B + ")), postfix);

    let infix = "A + B * C";
    let postfix = infix_to_postfix(infix);
    assert_eq!(Some(String::from("A B C * + ")), postfix);

    let infix = "( A + B ) * C";
    let postfix = infix_to_postfix(infix);
    assert_eq!(Some(String::from("A B + C * ")), postfix);

    let infix = "A + B - C";
    let postfix = infix_to_postfix(infix);
    assert_eq!(Some(String::from("A B + C - ")), postfix);

    let infix = "A * B + C";
    let postfix = infix_to_postfix(infix);
    assert_eq!(Some(String::from("A B * C + ")), postfix);

    let infix = "A * B / C";
    let postfix = infix_to_postfix(infix);
    assert_eq!(Some(String::from("A B * C / ")), postfix);

    let infix = "A + B * C - D";
    let postfix = infix_to_postfix(infix);
    assert_eq!(Some(String::from("A B C * + D - ")), postfix);

    let infix = "A * B - C / D";
    let postfix = infix_to_postfix(infix);
    assert_eq!(Some(String::from("A B * C D / - ")), postfix);

    let infix = "A + B + C + D";
    let postfix = infix_to_postfix(infix);
    assert_eq!(Some(String::from("A B + C + D + ")), postfix);

    let infix = "( A + B ) * ( C - D )";
    let postfix = infix_to_postfix(infix);
    assert_eq!(Some(String::from("A B + C D - * ")), postfix);

    let infix = "( A + B ) * ( C + D )";
    let postfix = infix_to_postfix(infix);
    assert_eq!(Some(String::from("A B + C D + * ")), postfix);

    let infix = "( A + B ) * C - ( D + E ) / ( F + G )";
    let postfix = infix_to_postfix(infix);
    assert_eq!(Some(String::from("A B + C * D E + F G + / - ")), postfix);
}

#[test]
fn postfix_eval_test() {
    let postfix = "1 2 + 1 2 + *";
    let res = postfix_eval(postfix);
    assert_eq!(Some(9), res);

    let postfix = "4 5 6 * +";
    let res = postfix_eval(postfix);
    assert_eq!(Some(34), res);
}
