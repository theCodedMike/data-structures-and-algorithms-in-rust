use crate::_4_basic_data_structures::queue::Queue;

pub fn hot_potato(names: Vec<&str>, num: usize) -> &str {
    // 初始化队列、名字入队
    let mut q = Queue::new(names.len());
    for name in names {
        let _res = q.enqueue(name);
    }

    while q.size() > 1 {
        // 出入栈名字，相当于传递山芋
        for _i in 0..num {
            let name = q.dequeue().unwrap();
            let _res = q.enqueue(name);
        }

        // 出入栈达到num次，删除1人
        q.dequeue();
    }

    q.dequeue().unwrap()
}
