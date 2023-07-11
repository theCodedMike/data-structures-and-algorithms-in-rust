use std::cell::RefCell;
use std::rc::Rc;

/// 链接
type Link = Option<Rc<RefCell<Node>>>;

/// 节点
struct Node {
    data: usize,
    next: Link,
}

impl Node {
    fn new(data: usize) -> Self {
        Node { data, next: None }
    }
}

/// 图
pub struct Graph {
    first: Link,
    last: Link,
}

impl Graph {
    fn new() -> Self {
        Graph {
            first: None,
            last: None,
        }
    }

    fn is_empty(&self) -> bool {
        self.first.is_none()
    }

    fn get_first(&self) -> Link {
        self.first.clone()
    }

    fn print_node(&self) {
        let mut curr = self.first.clone();
        while let Some(val) = curr {
            print!(" [{}]", val.borrow().data);
            curr = val.borrow().next.clone();
        }
        println!();
    }

    fn insert(&mut self, data: usize) {
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

pub fn dfs(graph: Vec<(Graph, usize)>) {
    let mut gp = graph;
    let mut nodes = Vec::new();
    let mut temp = Vec::new();

    gp[1].1 = 1;
    let mut curr = gp[1].0.get_first().clone();

    // 打印图
    print!("{} -> ", 1);
    while let Some(val) = curr {
        nodes.insert(0, val.borrow().data);
        curr = val.borrow().next.clone();
    }

    // 打印深度优先图
    loop {
        if nodes.is_empty() {
            break;
        } else {
            let data = nodes.pop().unwrap();
            if gp[data].1 == 0 {
                gp[data].1 = 1;
                print!("{} -> ", data);
                // 节点加入temp
                let mut curr = gp[data].0.get_first().clone();
                while let Some(val) = curr {
                    temp.push(val.borrow().data);
                    curr = val.borrow().next.clone();
                }

                while !temp.is_empty() {
                    nodes.push(temp.pop().unwrap());
                }
            }
        }
    }

    println!();
}
