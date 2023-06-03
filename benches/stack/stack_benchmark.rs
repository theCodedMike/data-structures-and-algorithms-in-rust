use criterion::{black_box, criterion_group, criterion_main, Criterion};
use data_structures_and_algorithms_in_rust::_4_basic_data_structures::exercise::{
    double_list_head_stack_bench_test, double_list_tail_stack_bench_test,
    single_list_head_stack_bench_test, single_list_tail_stack_bench_test,
    vec_head_stack_bench_test, vec_tail_stack_bench_test,
};

const COUNT: usize = 100;

pub fn vec_head_stack(c: &mut Criterion) {
    c.bench_function("vec_head_stack", |b| {
        b.iter(|| vec_head_stack_bench_test(black_box(COUNT)))
    });
}

pub fn vec_tail_stack(c: &mut Criterion) {
    c.bench_function("vec_tail_stack", |b| {
        b.iter(|| vec_tail_stack_bench_test(black_box(COUNT)))
    });
}

pub fn single_list_head_stack(c: &mut Criterion) {
    c.bench_function("single_list_head_stack", |b| {
        b.iter(|| single_list_head_stack_bench_test(black_box(COUNT)))
    });
}

pub fn single_list_tail_stack(c: &mut Criterion) {
    c.bench_function("single_list_tail_stack", |b| {
        b.iter(|| single_list_tail_stack_bench_test(black_box(COUNT)))
    });
}

pub fn double_list_head_stack(c: &mut Criterion) {
    c.bench_function("double_list_head_stack", |b| {
        b.iter(|| double_list_head_stack_bench_test(black_box(COUNT)))
    });
}

pub fn double_list_tail_stack(c: &mut Criterion) {
    c.bench_function("double_list_tail_stack", |b| {
        b.iter(|| double_list_tail_stack_bench_test(black_box(COUNT)))
    });
}

criterion_group!(
    benches,
    vec_head_stack,
    vec_tail_stack,
    single_list_head_stack,
    single_list_tail_stack,
    double_list_head_stack,
    double_list_tail_stack
);
criterion_main!(benches);
