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
fn merge_copy(l1: &mut Vec<i32>, s: usize, e: usize, l2: &Vec<i32>) {
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
pub fn sort(l: &mut Vec<i32>) {
    let size: usize = l.len();
    let mut worker: Vec<i32> = vec![0; size];

    merge_split(l, 0, size, &mut worker);
}



/// A set of unit tests to run over the merge functions above.
/// These unit tests have complete coverage, and may be run using:
/// `cargo test`
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_basic() {
        let l1 = vec![1, 3, 5, 2, 4, 6];
        let mut l2 = vec![0; 6];

        merge(&l1, 0, 3, 6, &mut l2);

        assert_eq!(l2, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn merge_duplicates() {
        let l1 = vec![1, 1, 2, 1, 2, 2];
        let mut l2 = vec![0; 6];

        merge(&l1, 0, 3, 6, &mut l2);

        assert_eq!(l2, vec![1, 1, 1, 2, 2, 2]);
    }

    #[test]
    fn merge_empty() {
        let l1 = vec![];
        let mut l2 = vec![];

        merge(&l1, 0, 0, 0, &mut l2);

        assert!(l2.is_empty());
    }

    #[test]
    #[should_panic]
    fn merge_out_of_bound() {
        let l1 = vec![];
        let mut l2 = vec![];

        merge(&l1, 0, 1, 2, &mut l2);
    }

    #[test]
    #[should_panic]
    fn merge_smaller_output() {
        let l1 = vec![1, 2, 3];
        let mut l2 = vec![1];

        merge(&l1, 0, 1, 2, &mut l2);
    }

    #[test]
    fn merge_copy_basic() {
        let mut l1 = vec![0; 5];
        let l2 = vec![1, 2, 3, 4, 5];

        merge_copy(&mut l1, 0, l2.len(), &l2);

        assert_eq!(l1, l2);
    }

    #[test]
    fn merge_copy_selection() {
        let mut l1 = vec![0; 5];
        let l2 = vec![1, 2, 3, 4, 5];

        merge_copy(&mut l1, 1, 4, &l2);

        assert_eq!(l1, vec![0, 2, 3, 4, 0]);
    }

    #[test]
    fn merge_copy_nothing() {
        let mut l1 = vec![0; 3];
        let l2 = vec![1, 2, 3];

        merge_copy(&mut l1, 0, 0, &l2);

        assert_eq!(l1, vec![0, 0, 0]);
    }

    #[test]
    #[should_panic]
    fn merge_copy_out_of_bound() {
        let mut l1 = vec![0; 5];
        let l2 = vec![1, 2, 3, 4, 5];

        merge_copy(&mut l1, 0, 10, &l2);
    }

    #[test]
    fn merge_split_basic() {
        let mut l1 = vec![4, 3, 1, 5, 2];
        let mut l2 = vec![0; 5];

        merge_split(&mut l1, 0, 5, &mut l2);

        assert_eq!(l1, l2);
        assert_eq!(l2, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn merge_split_duplicates() {
        let mut l1 = vec![2, 1, 3, 3, 2, 3];
        let mut l2 = vec![0; 6];

        merge_split(&mut l1, 0, 6, &mut l2);

        assert_eq!(l1, l2);
        assert_eq!(l2, vec![1, 2, 2, 3, 3, 3]);
    }

    #[test]
    fn merge_split_selection() {
        let mut l1 = vec![4, 3, 1, 5, 2];
        let mut l2 = vec![0; 5];

        merge_split(&mut l1, 1, 4, &mut l2);

        assert_eq!(l1[1..4], l2[1..4]);
        assert_eq!(l2, vec![0, 1, 3, 5, 0]);
    }

    #[test]
    fn merge_split_empty() {
        let mut l1 = vec![];
        let mut l2 = vec![];

        merge_split(&mut l1, 0, 0, &mut l2);

        assert_eq!(l1, l2);
        assert!(l2.is_empty());
    }

    #[test]
    #[should_panic]
    fn merge_split_out_of_bound() {
        let mut l1 = vec![1, 2, 3];
        let mut l2 = vec![0; 3];

        merge_split(&mut l1, 0, 5, &mut l2);
    }

    #[test]
    #[should_panic]
    fn merge_split_smaller_out() {
        let mut l1 = vec![1, 2, 3];
        let mut l2 = vec![];

        merge_split(&mut l1, 0, 3, &mut l2);

        assert_eq!(l1, l2);
    }

    #[test]
    fn sort_basic() {
        let mut l = vec![3, 1, 5, 4, 2];

        sort(&mut l);

        assert_eq!(l, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn sort_duplicates() {
        let mut l = vec![3, 1, 2, 3, 2, 3];

        sort(&mut l);

        assert_eq!(l, vec![1, 2, 2, 3, 3, 3]);
    }

    #[test]
    fn sort_emtpy() {
        let mut l = vec![];

        sort(&mut l);

        assert!(l.is_empty());
    }

    #[test]
    fn sort_negative() {
        let mut l = vec![3, -3, 1, 0, -2, -1, 2];

        sort(&mut l);

        assert_eq!(l, vec![-3, -2, -1, 0, 1, 2, 3]);
    }
}
