use data_structures_and_algorithms_in_rust::_10_exercise::_1_edit_distance::levenshtein_distance::{edit_distance, edit_distance2};

/// cargo test test_edit_distance
#[test]
fn test_edit_distance() {
    let source = "";
    let target = "adcf";
    let distance = edit_distance(source, target);
    assert_eq!(distance, 4);

    let source = "abce";
    let target = "";
    let distance = edit_distance(source, target);
    assert_eq!(distance, 4);

    let source = "abce";
    let target = "adcf";
    let distance = edit_distance(source, target);
    assert_eq!(distance, 2);

    let source = "bdfc";
    let target = "adcf";
    let distance = edit_distance(source, target);
    assert_eq!(distance, 3);
}

/// cargo test test_edit_distance2
#[test]
fn test_edit_distance2() {
    let source = "";
    let target = "adcf";
    let distance = edit_distance2(source, target);
    assert_eq!(distance, 4);

    let source = "abce";
    let target = "";
    let distance = edit_distance2(source, target);
    assert_eq!(distance, 4);

    let source = "abce";
    let target = "adcf";
    let distance = edit_distance2(source, target);
    assert_eq!(distance, 2);

    let source = "bdfc";
    let target = "adcf";
    let distance = edit_distance2(source, target);
    assert_eq!(distance, 3);
}
