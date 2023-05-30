use data_structures_and_algorithms_in_rust::_3_algorithm_analysis as algorithm_analysis;

#[test]
fn anagram_solution2_test() {
    let s1 = "rust";
    let s2 = "trus";
    assert_eq!(true, algorithm_analysis::anagram_solution2(s1, s2));

    let s1 = "reuse";
    let s2 = "rsuee";
    assert_eq!(true, algorithm_analysis::anagram_solution2(s1, s2));

    let s1 = "www";
    let s2 = "www";
    assert_eq!(true, algorithm_analysis::anagram_solution2(s1, s2));

    let s1 = "abcdefghi";
    let s2 = "abcdefghh";
    assert_ne!(true, algorithm_analysis::anagram_solution2(s1, s2));
}

#[test]
fn custom_anagram_solution2_test() {
    let s1 = "rust";
    let s2 = "trus";
    assert_eq!(true, algorithm_analysis::custom_anagram_solution2(s1, s2));

    let s1 = "reuse";
    let s2 = "rsuee";
    assert_eq!(true, algorithm_analysis::custom_anagram_solution2(s1, s2));

    let s1 = "www";
    let s2 = "www";
    assert_eq!(true, algorithm_analysis::custom_anagram_solution2(s1, s2));

    let s1 = "abcdefghi";
    let s2 = "abcdefghh";
    assert_ne!(true, algorithm_analysis::custom_anagram_solution2(s1, s2));
}

#[test]
fn anagram_solution3_test() {
    let s1 = "rust";
    let s2 = "trus";
    assert_eq!(true, algorithm_analysis::anagram_solution3(s1, s2));

    let s1 = "reuse";
    let s2 = "rsuee";
    assert_eq!(true, algorithm_analysis::anagram_solution3(s1, s2));

    let s1 = "www";
    let s2 = "www";
    assert_eq!(true, algorithm_analysis::anagram_solution3(s1, s2));

    let s1 = "abcdefghi";
    let s2 = "abcdefghh";
    assert_ne!(true, algorithm_analysis::anagram_solution3(s1, s2));
}

#[test]
fn custom_anagram_solution3_test() {
    let s1 = "rust";
    let s2 = "trus";
    assert_eq!(true, algorithm_analysis::custom_anagram_solution3(s1, s2));

    let s1 = "reuse";
    let s2 = "rsuee";
    assert_eq!(true, algorithm_analysis::custom_anagram_solution3(s1, s2));

    let s1 = "www";
    let s2 = "www";
    assert_eq!(true, algorithm_analysis::custom_anagram_solution3(s1, s2));

    let s1 = "abcdefghi";
    let s2 = "abcdefghh";
    assert_ne!(true, algorithm_analysis::custom_anagram_solution3(s1, s2));
}

#[test]
fn anagram_solution4_test() {
    let s1 = "rust";
    let s2 = "trus";
    assert_eq!(true, algorithm_analysis::anagram_solution4(s1, s2));

    let s1 = "reuse";
    let s2 = "rsuee";
    assert_eq!(true, algorithm_analysis::anagram_solution4(s1, s2));

    let s1 = "www";
    let s2 = "www";
    assert_eq!(true, algorithm_analysis::anagram_solution4(s1, s2));

    let s1 = "abcdefghi";
    let s2 = "abcdefghh";
    assert_ne!(true, algorithm_analysis::anagram_solution4(s1, s2));
}

#[test]
fn custom_anagram_solution4_test() {
    let s1 = "rust";
    let s2 = "trus";
    assert_eq!(true, algorithm_analysis::custom_anagram_solution4(s1, s2));

    let s1 = "reuse";
    let s2 = "rsuee";
    assert_eq!(true, algorithm_analysis::custom_anagram_solution4(s1, s2));

    let s1 = "www";
    let s2 = "www";
    assert_eq!(true, algorithm_analysis::custom_anagram_solution4(s1, s2));

    let s1 = "abcdefghi";
    let s2 = "abcdefghh";
    assert_ne!(true, algorithm_analysis::custom_anagram_solution4(s1, s2));
}
