#![allow(unused_variables)]

use crate::_4_basic_data_structures::exercise::{
    DoubleListHeadStack, DoubleListTailStack, ExerciseStack, SingleListHeadStack,
    SingleListTailStack, VecHeadStack, VecTailStack,
};

pub fn vec_head_stack_bench_test(size: usize) {
    let mut s = VecHeadStack::new();

    for i in 0..size {
        s.push(i);
    }
    while let Some(v) = s.pop() {
        // println!("{}", v);
    }

    for i in 0..size {
        s.push(size - i);
    }
    while let Some(v) = s.pop() {
        // println!("{}", v);
    }
}

pub fn vec_tail_stack_bench_test(size: usize) {
    let mut s = VecTailStack::new();

    for i in 0..size {
        s.push(i);
    }
    while let Some(v) = s.pop() {
        // println!("{}", v);
    }

    for i in 0..size {
        s.push(size - i);
    }
    while let Some(v) = s.pop() {
        // println!("{}", v);
    }
}

pub fn single_list_head_stack_bench_test(size: usize) {
    let mut s = SingleListHeadStack::new();

    for i in 0..size {
        s.push(i);
    }
    while let Some(v) = s.pop() {
        // println!("{}", v);
    }

    for i in 0..size {
        s.push(size - i);
    }
    while let Some(v) = s.pop() {
        // println!("{}", v);
    }
}

pub fn single_list_tail_stack_bench_test(size: usize) {
    let mut s = SingleListTailStack::new();

    for i in 0..size {
        s.push(i);
    }
    while let Some(v) = s.pop() {
        // println!("{}", v);
    }

    for i in 0..size {
        s.push(size - i);
    }
    while let Some(v) = s.pop() {
        // println!("{}", v);
    }
}

pub fn double_list_head_stack_bench_test(size: usize) {
    let mut s = DoubleListHeadStack::new();

    for i in 0..size {
        s.push(i);
    }
    while let Some(v) = s.pop() {
        // println!("{}", v);
    }

    for i in 0..size {
        s.push(size - i);
    }
    while let Some(v) = s.pop() {
        // println!("{}", v);
    }
}

pub fn double_list_tail_stack_bench_test(size: usize) {
    let mut s = DoubleListTailStack::new();

    for i in 0..size {
        s.push(i);
    }
    while let Some(v) = s.pop() {
        // println!("{}", v);
    }

    for i in 0..size {
        s.push(size - i);
    }
    while let Some(v) = s.pop() {
        // println!("{}", v);
    }
}
