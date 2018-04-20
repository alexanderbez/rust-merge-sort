//! Merge sort implementation in Rust!
//! Running complexity: O(nlog(n)) where n is the total number of elements.
//! Space complexity: O(2*n) where n is the total number of elements.

/// Merges two ordered sub-lists. These ordered sub-lists are differentiated
/// via a start, middle and end index within a primary mutable list, `l1`.
/// The ordering result is copied to the worker list, `l2`.
fn merge(l1: &Vec<i32>, s: usize, m: usize, e: usize, l2: &mut Vec<i32>) {
    let mut ptr1 = s;
    let mut ptr2 = m;

    for i in s..e {
        // Continue to compare elements within each sub-list until one sub-list
        // is exhausted. If a sub-list is exhausted, then the remaing elements
        // in the other list are copied over assuming they're already in order.
        if (ptr1 < m) && (ptr2 >= e || l1[ptr1] <= l1[ptr2]) {
            l2[i] = l1[ptr1];
            ptr1 += 1;
        } else {
            l2[i] = l1[ptr2];
            ptr2 += 1;
        }
    }
}

/// Copies all elements from a worker mutable list, `l2`, to a mutable primary
/// list, `l1`. The index ranges are regarded to represent an ordered sublist
/// within the worker list, `l2`.
fn merge_copy(l1: &Vec<i32>, s: usize, e: usize, l2: &mut Vec<i32>) {
    (s..e).for_each(|i| l1[i] = l2[i]);
}

/// Splits a mutable list into two sub-lists. The split is done recursively
/// until only n sub-lists remain where n is the total number of elements in
/// the original list.
/// These sub-lists are then merged together keeping order.
fn merge_split(l1: &mut Vec<i32>, s: usize, e: usize, l2: &mut Vec<i32>) {
    if e - s > 1 {
        let m: usize = (e + s) / 2;

        merge_split(l1, s, m, l2);
        merge_split(l1, m, e, l2);
        merge(l1, s, m, e, l2);
        merge_copy(l1, s, e, l2);
    }
}

/// Performs merge sort on a mutable vector of `i32` elements.
pub fn sort(l1: &mut Vec<i32>) {
    let size: usize          = l1.len();
    let mut worker: Vec<i32> = vec![0; size];

    merge_split(l1, 0, size, &mut worker);
}
