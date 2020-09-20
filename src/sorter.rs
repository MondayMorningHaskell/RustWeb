#![feature(test)]

use rand::{Rng, thread_rng};

pub fn swap(numbers: &mut Vec<i32>, i: usize, j: usize) {
    let temp = numbers[i];
    numbers[i] = numbers[j];
    numbers[j] = temp;
}

pub fn insertion_sorter(numbers: &mut Vec<i32>) {
    for i in 1..numbers.len() {
        let mut j = i;
        while j > 0 && numbers[j-1] > numbers[j] {
            swap(numbers, j, j - 1);
            j = j - 1;
        }
    }
}

pub fn partition(numbers: &mut Vec<i32>, start: usize, end: usize, partition: usize) -> usize {
    let pivot_element = numbers[partition];
    swap(numbers, partition, end - 1);
    let mut i = start;
    for j in start..(end - 1) {
        if numbers[j] < pivot_element {
            swap(numbers, i, j);
            i = i + 1;
        }
    }
    swap(numbers, i, end - 1);
    i
}

pub fn quick_sorter_helper(numbers: &mut Vec<i32>, start: usize, end: usize) {
    if start >= end {
        return;
    }

    let mut rng = thread_rng();
    let initial_partition = rng.gen_range(start, end);
    let partition_index = partition(numbers, start, end, initial_partition);
    quick_sorter_helper(numbers, start, partition_index);
    quick_sorter_helper(numbers, partition_index + 1, end);
}

pub fn quick_sorter(numbers: &mut Vec<i32>) {
    quick_sorter_helper(numbers, 0, numbers.len());
}

pub fn random_vector(i: i32) -> Vec<i32> {
    let mut numbers: Vec<i32> = Vec::new();
    let mut rng = rand::thread_rng();
    for i in 0..i {
        numbers.push(rng.gen());
    }
    return numbers;
}

extern crate test;
use test::Bencher;

#[test]
fn test_insertion_sort() {
    let mut numbers: Vec<i32> = random_vector(100);
    insertion_sorter(&mut numbers);
    for i in 0..(numbers.len() - 1) {
        assert!(numbers[i] <= numbers[i + 1]);
    }
}

#[test]
fn test_quick_sort() {
    let mut numbers: Vec<i32> = random_vector(100);
    quick_sorter(&mut numbers);
    for i in 0..(numbers.len() - 1) {
        assert!(numbers[i] <= numbers[i + 1]);
    }
}
#[bench]
fn bench_insertion_sort_100_ints(b: &mut Bencher) {
    b.iter(|| {
        let mut numbers: Vec<i32> = random_vector(100);
        insertion_sorter(&mut numbers)
    });
}

#[bench]
fn bench_quick_sort_100_ints(b: &mut Bencher) {
    b.iter(|| {
        let mut numbers: Vec<i32> = random_vector(100);
        quick_sorter(&mut numbers)
    });
}

#[bench]
fn bench_insertion_sort_1000_ints(b: &mut Bencher) {
    b.iter(|| {
        let mut numbers: Vec<i32> = random_vector(1000);
        insertion_sorter(&mut numbers)
    });
}

#[bench]
fn bench_quick_sort_1000_ints(b: &mut Bencher) {
    b.iter(|| {
        let mut numbers: Vec<i32> = random_vector(1000);
        quick_sorter(&mut numbers)
    });
}

#[bench]
fn bench_insertion_sort_10000_ints(b: &mut Bencher) {
    b.iter(|| {
        let mut numbers: Vec<i32> = random_vector(10000);
        insertion_sorter(&mut numbers)
    });
}

#[bench]
fn bench_quick_sort_10000_ints(b: &mut Bencher) {
    b.iter(|| {
        let mut numbers: Vec<i32> = random_vector(10000);
        quick_sorter(&mut numbers)
    });
}

#[bench]
fn bench_quick_sort_100000_ints(b: &mut Bencher) {
    b.iter(|| {
        let mut numbers: Vec<i32> = random_vector(100000);
        quick_sorter(&mut numbers)
    });
}
