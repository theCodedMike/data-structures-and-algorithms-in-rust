use data_structures_and_algorithms_in_rust::_4_basic_data_structures::vec::LVec;

#[test]
fn lvec_test() {
    let mut lvec = LVec::new();
    assert_eq!(true, lvec.is_empty());

    lvec.push(10); // [10]
    lvec.push(11); // [10, 11]
    lvec.push(12); // [10, 11, 12]
    lvec.push(14); // [10, 11, 12, 14]
    lvec.insert(0, 9); //  [9, 10, 11, 12, 14]
    lvec.insert(4, 13); // [9, 10, 11, 12, 13, 14]
    assert_eq!(6, lvec.size());
    assert_eq!(0, lvec.find(9));
    assert_eq!(1, lvec.find(10));
    assert_eq!(2, lvec.find(11));
    assert_eq!(3, lvec.find(12));
    assert_eq!(4, lvec.find(13));
    assert_eq!(5, lvec.find(14));
    assert_eq!(-1, lvec.find(15));
    assert_eq!(true, lvec.exist(12));
    assert_eq!(false, lvec.exist(15));

    assert_eq!(Some(9), lvec.remove(0)); //  [10, 11, 12, 13, 14]
    assert_eq!(Some(&10), lvec.get(0));
    assert_eq!(Some(&11), lvec.get(1));
    assert_eq!(Some(&12), lvec.get(2));
    assert_eq!(Some(&13), lvec.get(3));
    assert_eq!(Some(&14), lvec.get(4));
    assert_eq!(None, lvec.get(5));
    assert_eq!(Some(14), lvec.remove(4)); // [10, 11, 12, 13]
    assert_eq!(None, lvec.remove(5));
    assert_eq!(4, lvec.size());

    assert_eq!(Some(11), lvec.remove(1)); // [10, 12, 13]
    assert_eq!(3, lvec.size());
    assert_eq!(Some(&10), lvec.get(0));
    assert_eq!(Some(&12), lvec.get(1));
    assert_eq!(Some(&13), lvec.get(2));
    assert_eq!(None, lvec.get(3));

    let mut lvec2 = LVec::new();
    lvec2.insert(0, 8); // [8]
    lvec2.append(&mut lvec); //  [8, 10, 12, 13]
    assert_eq!(0, lvec.size());
    assert_eq!(4, lvec2.size());
    assert_eq!(0, lvec2.find(8));
    assert_eq!(1, lvec2.find(10));
    assert_eq!(2, lvec2.find(12));
    assert_eq!(3, lvec2.find(13));
    assert_eq!(Some(&8), lvec2.get(0));
    assert_eq!(Some(&10), lvec2.get(1));
    assert_eq!(Some(&12), lvec2.get(2));
    assert_eq!(Some(&13), lvec2.get(3));
    assert_eq!(None, lvec2.get(4));

    assert_eq!(Some(13), lvec2.pop()); //          [8, 10, 12]
    assert_eq!(Some(8), lvec2.remove(0)); // [10, 12]
    assert_eq!(2, lvec2.size());

    assert_eq!(0, lvec2.find(10));
    assert_eq!(1, lvec2.find(12));

    assert_eq!(Some(&10), lvec2.get(0));
    assert_eq!(Some(&12), lvec2.get(1));
    assert_eq!(None, lvec2.get(3));
}
