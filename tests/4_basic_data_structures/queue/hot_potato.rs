use data_structures_and_algorithms_in_rust::_4_basic_data_structures::_4_2_queue::{
    hot_potato, Queue,
};

#[test]
fn hot_potato_test() {
    let names = vec!["Shieber", "David", "Susan", "Jane", "Kew", "Brad"];
    let rem = hot_potato(names, 8);
    assert_eq!("Kew", rem);
}

/// cargo test -- --show-output josephus_problem_test
///
/// -- --show-output可以查看std::io::stdout输出的内容
#[test]
fn josephus_problem_test() {
    let total = 6;
    let cycle = 8;
    let mut queue = Queue::new(total);

    for i in 1..=total {
        let _ = queue.enqueue(i);
    }

    while queue.size() > 1 {
        for _i in 1..cycle {
            let head = queue.dequeue().unwrap();
            let _ = queue.enqueue(head);
        }

        let target = queue.dequeue().unwrap();
        println!(" out: {}", target);
    }

    println!("left: {}", queue.dequeue().unwrap());
}
