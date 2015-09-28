/*
  Example use of merge sort.
*/

// Use external crate(s)
extern crate merge_sort;

fn main () {
  // Example mutable vector of i32 elements
  let mut example_list: Vec<i32> = vec![234234, 9, 4, 5, 99, 1,-3, 0, -1];

  // Print the original vector to STDOUT
  println!("Original list: {:?}", example_list);

  // Merge the list
  merge_sort::sort(&mut example_list);

  // Print the sorted vector to STDOUT
  println!("Sorted list: {:?}", example_list);
}