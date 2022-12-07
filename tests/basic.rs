extern crate test_runner;

use test_runner::run_test_kernel;

#[test]
fn basic_boot() {
    run_test_kernel(env!("CARGO_BIN_FILE_TEST_BASIC_basic_boot"));
}

#[test]
fn should_panic() {
    run_test_kernel(env!("CARGO_BIN_FILE_TEST_BASIC_should_panic"));
}

#[test]
fn stack_overflow() {
    run_test_kernel(env!("CARGO_BIN_FILE_TEST_BASIC_stack_overflow"));
}

#[test]
fn heap_allocation() {
    run_test_kernel(env!("CARGO_BIN_FILE_TEST_BASIC_heap_allocation"));
}
