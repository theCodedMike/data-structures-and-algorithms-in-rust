/// 字典树定义
#[derive(Debug, Default)]
pub struct Trie {
    root: Node,
}

/// 节点
#[derive(Debug, Default)]
struct Node {
    end: bool,
    children: [Option<Box<Node>>; 26], //字符节点列表
}

impl Trie {
    pub fn new() -> Self {
        Trie::default()
    }

    /// 插入单词
    pub fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for c in word.as_bytes() {
            let mut c = *c;
            // 将大写转换为小写
            if c.is_ascii_uppercase() {
                c = c.to_ascii_lowercase();
            }
            let idx = (c - b'a') as usize;
            let next = &mut node.children[idx];
            node = next.get_or_insert_with(|| Box::new(Node::default()));
        }
        node.end = true;
    }

    /// 搜索某个单词，完全匹配
    ///
    /// 忽略大小写
    pub fn search(&self, word: &str) -> bool {
        self.word_node(word).map_or(false, |n| n.end)
    }

    /// 判断是否存在以某个前缀开头的单词
    ///
    /// 忽略大小写
    pub fn start_with(&self, prefix: &str) -> bool {
        self.word_node(prefix).is_some()
    }

    /// 前缀字符串
    ///
    /// wps: word_prefix_string
    fn word_node(&self, wps: &str) -> Option<&Node> {
        let mut node = &self.root;
        for c in wps.as_bytes() {
            let mut c = *c;
            if c.is_ascii_uppercase() {
                c = c.to_ascii_lowercase();
            }
            let idx = (c - b'a') as usize;
            match &node.children[idx] {
                None => return None,
                Some(next) => node = next,
            }
        }

        Some(node)
    }
}
