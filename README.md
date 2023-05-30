#### Study notes, thanks to [Shieber](https://github.com/QMHTMY/RustBook)

## Data Structures and Algorithms in Rust

### 序 Preface

### Rust基础 Rust Basic
- 本章目标 Objectives
- 安装Rust及其工具链 Install Rust and its Toolchain
- 学习资源 Learning Resources
- 复习 Review
- 总结 Summary


### 计算机科学 Computer Science
- 本章目标 Objectives
- 快速开始 Getting Started
- 什么是计算机科学? What is Computer Science?
- 什么是编程? What is Programming?
- 为什么学习数据结构? Why Study Data Structures and Abstract Data Types?
- 为什么学习算法? Why Study Algorithms?
- 总结 Summary


### 算法分析 Algorithm Analysis
- 本章目标 Objectives
- 什么是算法分析? What is Algorithm Analysis?
- 大O分析法 Big-O Notation Analysis
- 乱序字符串检查 Anagram Detection
  - 穷举法 Brute Force
  - 检查法 Checking Off
  - 排序并比较法 Sort and Compare
  - 计数并比较法 Count and Compare
- Rust数据结构的性能 Performance of Rust Data Structures
  - 标量和复合类型 Scalar and Complex Data Structures
  - 集合类型 Collection Data Structures
- 总结 Summary


### 基本数据结构 Basic Data Structures
- 本章目标 Objectives
- 线性数据结构 Linear Structures
- 栈 Stack
  - 栈的抽象数据类型 The Stack Abstract Data Type
  - 使用Rust实现栈 Implementing a Stack in Rust
  - 括号匹配 Simple Balanced Parentheses
  - 进制转换 Converting Decimal Numbers to Binary Numbers
  - 前、中、后缀表达式 Prefix, Infix, Postfix Expressions
  - 中缀转前、后缀表达式 Conversion of Infix Expressions to Prefix and Postfix
- 队列 Queue
  - 队列的抽象数据类型 The Queue Abstract Data Type
  - 使用Rust实现队列 Implementing a Queue in Rust
  - 烫手山芋 Hot Potato
- 双端队列 Deque
  - 双端队列的抽象数据类型 The Deque Abstract Data Type
  - 使用Rust实现双端队列 Implementing a Deque in Rust
  - 回文检测 Palindrome Checker
- 链表 LinkedList
  - 链表的抽象数据类型 The LinkedList Abstract Type
  - 使用Rust实现链表 Implementing a LinkedList in Rust
  - 链表栈 LinkedList Stack
- 动态数组 Vec
  - Vec的抽象数据类型 The Vec Abstract Data Type
  - 使用Rust实现Vec Implementing a Vec in Rust
- 总结 Summary


### 递归 Recursion
- 本章目标 Objectives
- 什么是递归? What is Recursion?
  - 递归三定律 The Three Laws of Recursion
  - 到任意进制的转换 Converting an Integer to a String in Any Base
  - 汉诺塔 Tower of Hanoi
- 尾递归 Tail Recursion
  - 递归和迭代 Recursion VS. Iteration
- 动态规划 Dynamic Programming
  - 什么是动态规划? What is Dynamic Programming?
  - 动态规划与递归 Dynamic Programming VS. Recursion
- 总结 Summary


### 查找 Searching
- 本章目标 Objectives
- 什么是查找? What is Searching?
- 顺序查找 The Sequential Search
  - 使用Rust实现顺序查找 Implementing a Sequential Search in Rust
  - 顺序查找复杂度 Analysis of Sequential Search
- 二分查找 The Binary Search
  - 使用Rust实现二分查找 Implementing a Binary Search
  - 二分查找复杂度 Analysis of Binary Search
  - 内插查找 The Interpolation Search
  - 指数查找 The Exponential Search
- 哈希查找 The Hash Search
  - 哈希函数 Hash Functions
  - 解决冲突 Collision Resolution
  - 使用Rust实现HashMap Implementing a HashMap in Rust
  - HashMap复杂度 Analysis of HashMap
- 总结 Summary


### 排序 Sorting
- 本章目标 Objectives
- 什么是排序? What is Sorting?
- 冒泡排序 The Bubble Sort
- 快速排序 The Quick Sort
- 插入排序 The insertion Sort
- 希尔排序 The Shell Sort
- 归并排序 The Merge Sort
- 选择排序 The Selection Sort
- 堆排序 The Heap Sort
- 桶排序 The Bucket Sort
- 计数排序 The Counting Sort
- 基数排序 The Radix Sort
- 蒂姆排序 The Tim Sort
- 总结 Summary


### 树 Tree
- 本章目标 Objectives
- 什么是树? What is Tree?
  - 树的定义Vocabularies and Definitions of Tree
  - 树的表示Tree Representation
  - 分析树Parse Tree
  - 树的遍历 Tree Traversals
- 二叉堆 Binary Heap
  - 二叉堆的抽象数据类型 The Binary Heap Abstract Data Type
  - 使用Rust实现二叉堆 Implementing a Binary Heap in Rust
  - 二叉堆分析 Analysis of Binary Heap
- 二叉查找树 Binary Search Tree
  - 二叉查找树的抽象数据类型 The Binary Search Tree Abstract Data Type
  - 使用Rust实现二叉查找树 Implementing a Binary Search Tree in Rust
  - 二叉查找树分析 Analysis of Binary Search Tree
- 平衡二叉树 Balanced Binary Search Tree
  - AVL平衡二叉树 AVL Tree
  - 使用Rust实现平衡二叉树 Implementing an AVL Tree in Rust
  - 平衡二叉树分析 Analysis of AVL Tree
- 总结 Summary


### 图 Graph
- 本章目标
- 什么是图
  - 图的定义
- 图的存储形式
  - 邻接矩阵
  - 邻接表
- 图的抽象数据类型
- 图的实现
  - 图解决字梯问题
- 广度优先搜索
  - 实现广度优先搜索
  - 广度优先搜索分析
  - 骑士之旅
  - 图解决骑士之旅问题
- 深度优先搜索
  - 实现深度优先搜索
  - 深度优先搜索分析
  - 拓扑排序
- 强连通分量
- 最短路径问题
  - Dijkstra算法
  - 实现Dijkstra算法
  - Dijkstra算法分析
- 总结

### 实践 Practice
- 本章目标
- 编辑距离
  - 汉明距离
  - 莱温斯坦距离
- 字典树
- 过滤器
  - 布隆过滤器
  - 布谷鸟过滤器
- 缓存淘汰算法LRU
- 一致性哈希算法
- Base58编码
- 区块链
  - 区块链及比特币原理
  - 基础区块链
  - 工作量证明
  - 区块链存储
  - 交易
  - 账户
  - 梅根哈希
  - 矿工及挖矿
  - 比特币奖励
  - 回顾
- 总结