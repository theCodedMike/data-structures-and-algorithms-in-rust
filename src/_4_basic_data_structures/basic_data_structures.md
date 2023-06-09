# 基本数据结构

## 本章目标
- 理解抽象数据类型 Vec、栈、队列、双端队列
- 能够使用 Rust 实现堆栈、队列、双端队列
- 了解基本线性数据结构实现的性能
- 了解前缀、中缀和后缀表达式格式
- 使用栈来实现后缀表达式并计算值
- 使用栈将中缀表达式转换为后缀表达式
- 能够识别问题该使用栈、队列还是双端队列
- 能够使用节点和引用将抽象数据类型实现为链表
- 能够比较自己实现链表与 Rust 自带的 Vec 的性能


## 总结
> 本章主要学习了栈、队列、双端队列、链表、Vec 这些线性数据结构。栈是维持后进先出
> （LIFO）排序的数据结构，其基本操作是 push，pop，is_empty。栈对于设计计算解析表达式
> 算法非常有用，栈可以提供反转特性，在操作系统函数调用，网页保存方面非常有用。前缀
> 中缀和后缀表达式都是表达式，可以用栈来处理，但计算机不用中缀表达式。队列是维护先
> 进先出（FIFO）排序的简单数据结构，其基本操作是 enqueue，dequeue，is_empty，队列在系
> 统任务调度方面很实用，可以帮助构建定时仿真。双端队列是允许类似栈和队列的混合行为
> 的数据结构，其基本操作是 is_empty，add_front，add_rear，remove_front，remove_rear。
> 链表是项的集合，其中每个项保存在相对位置。链表的实现本身就能保持逻辑顺序，不需要
> 物理顺序存储。修改链表头是一种特殊情况。Vec 是 Rust 自带的数据容器，默认实现是用的
> 动态数组，本章使用的是链表。