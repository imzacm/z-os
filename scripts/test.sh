#! /bin/bash

echo 'Run all tests with default features'
cargo test
echo 'Run heap allocation tests with alloc_linked_crate feature'
cargo test --no-default-features --features alloc_linked_crate --test heap_allocation
echo 'Run heap allocation tests with alloc_linked_internal feature'
cargo test --no-default-features --features alloc_linked_internal --test heap_allocation
echo 'Run heap allocation tests with alloc_bump_internal feature'
cargo test --no-default-features --features alloc_bump_internal --test heap_allocation
echo 'Run heap allocation tests with alloc_fixed_internal feature'
cargo test --no-default-features --features alloc_fixed_internal --test heap_allocation
