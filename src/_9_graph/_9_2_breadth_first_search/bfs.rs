use std::cell::RefCell;
use std::rc::Rc;

/// 因为节点存在多个共享的链接，Box不可共享，Rc才可共享
/// 又因为Rc不可变，所以使用具有内部可变性的RefCell包裹
type Link = Option<Rc<RefCell<Node>>>;
/// 节点
pub struct Node {
    data: usize,
    next: Link,
}

impl Node {
    fn new(data: usize) -> Self {
        Node { data, next: None }
    }
}

/// 图定义及其实现
pub struct Graph {
    first: Link,
    last: Link,
}
impl Graph {
    pub fn new() -> Self {
        Graph {
            first: None,
            last: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.first.is_none()
    }

    pub fn get_first(&self) -> Link {
        self.first.clone()
    }

    /// 打印节点
    pub fn print_node(&self) {
        let mut curr = self.first.clone();
        while let Some(val) = curr {
            print!(" [{}]", val.borrow().data);
            curr = val.borrow().next.clone();
        }
        println!();
    }

    /// 插入节点
    pub fn insert(&mut self, data: usize) {
        let node = Rc::new(RefCell::new(Node::new(data)));
        if self.is_empty() {
            self.first = Some(node.clone());
        } else {
            self.last.as_mut().map(|last| {
                last.borrow_mut().next = Some(node.clone());
            });
        }
        self.last = Some(node);
    }
}

/// 根据data构建图
pub fn create_graph(data: [[usize; 2]; 20]) -> Vec<(Graph, usize)> {
    let mut arr = Vec::new();
    for _ in 0..9 {
        arr.push((Graph::new(), 0));
    }

    for i in 1..9 {
        for j in 0..data.len() {
            if data[j][0] == i {
                arr[i].0.insert(data[j][1]);
            }
        }
        print!("[{}] ->", i);
        arr[i].0.print_node();
    }

    arr
}

pub fn bfs(graph: Vec<(Graph, usize)>) {
    let mut gp = graph;
    let mut nodes = Vec::new();

    gp[1].1 = 1;
    let mut curr = gp[1].0.get_first().clone();

    // 打印图
    print!("{} -> ", 1);
    while let Some(val) = curr {
        nodes.push(val.borrow().data);
        curr = val.borrow().next.clone();
    }

    // 打印宽度优先图
    loop {
        if nodes.len() == 0 {
            break;
        }
        let data = nodes.remove(0);
        if gp[data].1 == 0 {
            gp[data].1 = 1;
            print!("{} -> ", data);
            let mut curr = gp[data].0.get_first().clone();
            while let Some(val) = curr {
                nodes.push(val.borrow().data);
                curr = val.borrow().next.clone();
            }
        }
    }

    println!();
}
