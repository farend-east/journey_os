extern crate test_runner;

use test_runner::run_test_kernel;

#[test]
fn simple_allocation() {
    run_test_kernel(env!("CARGO_BIN_FILE_TEST_HEAP_simple_allocation"));
}

#[test]
fn large_vec() {
    run_test_kernel(env!("CARGO_BIN_FILE_TEST_HEAP_large_vec"));
}

#[test]
fn many_boxes() {
    run_test_kernel(env!("CARGO_BIN_FILE_TEST_HEAP_many_boxes"));
}

#[test]
fn many_boxes_long_lived() {
    run_test_kernel(env!("CARGO_BIN_FILE_TEST_HEAP_many_boxes_long_lived"));
}

#[test]
fn should_panic() {
    run_test_kernel(env!("CARGO_BIN_FILE_TEST_HEAP_should_panic"));
}
