pub mod _3_algorithm_analysis;
pub mod _4_basic_data_structures;
pub mod _5_recursion;
pub mod _6_searching;
pub mod _7_sorting;
pub mod _8_tree;

pub fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
