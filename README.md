Object Sorting Library
Usage
This Rust library provides sorting algorithms for various types of objects.
use object_sorting_library::{quick_sort, select_sort, insert_sort, merge_sort};

fn main() {
    // Sort integers using Quick Sort
    let mut numbers = vec![4, 2, 3, 1];
    quick_sort(&mut numbers, |a, b| a < b);
    println!("Quick Sort: {:?}", numbers);

    // Sort characters using Selection Sort
    let mut chars = vec!['b', 'd', 'a', 'c'];
    select_sort(&mut chars, |a, b| a < b);
    println!("Select Sort: {:?}", chars);

    // Sort strings using Insertion Sort
    let mut strings = vec!["banana", "apple", "cherry", "date"];
    insert_sort(&mut strings, |a, b| a < b);
    println!("Insert Sort: {:?}", strings);

    // Sort floats using Merge Sort
    let mut floats = vec![4.3, 2.1, 3.5, 1.9];
    merge_sort(&mut floats, |a, b| a < b);
    println!("Merge Sort: {:?}", floats);
}
