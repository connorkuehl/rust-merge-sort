use std::vec::Vec;

/// Applies a merge sort to the slice given.
/// https://en.wikipedia.org/wiki/Merge_sort
///
/// # Arguments
///
/// * `arr` - A slice to be sorted.
pub fn merge_sort<T: Copy + PartialOrd>(arr: &mut [T]) {
    if arr.len() == 0 {
        return;
    }

    let sorted = merge_helper(arr);
    arr.copy_from_slice(sorted.as_slice());
}

/// Recursively splits the slice into progressively smaller
/// slices for sorting each one. It will continue to recurse,
/// splitting the slice until the resultant slices are of size
/// 1. When the recursion is unwinding, these much smaller slices
/// are merged into a buffer according to their order and returned.
///
/// # Arguments
///
/// * `arr` - the slice to split, sort, and then merge
fn merge_helper<T: Copy + PartialOrd>(arr: &mut [T]) -> Vec<T> {
    let len = arr.len();
    
    // Only 1 item, this is the finest slice we can split to.
    if len == 1 {
        // Conveniently, a slice with 1 item is already sorted.
        return vec![arr[0]];
    }

    // Haven't reached the base case, so we will split this
    // slice into two parts (the first half and the second half).
    let left  = merge_helper(&mut arr[0..(len/2)]);
    let right = merge_helper(&mut arr[(len/2)..len]);
    
    // Merge these two slices, both of which should now already 
    // be sorted.
    merge(left, right)
}

/// Merges two slices according to the ordering of their elements (this
/// is basically the "sort" part of merge sort.
///
/// # Arguments
///
/// `first` - the first slice to merge in.
/// `second` - the second slice to merge in.
///
/// # Returns 
///
/// The sorted merge of the two slices.
fn merge<T: Copy + PartialOrd>(first: Vec<T>, second: Vec<T>) -> Vec<T> {
    // Keep track of our lengths
    let first_len = first.len();
    let secnd_len = second.len();

    // This will be used to index into the first slice for copying
    // that item.
    let mut f = 0;
    // Like above, but this one will keep track of our index for
    // items in the second slice.
    let mut s = 0;

    // This will be the sorted, merged slice!
    let mut sorted = vec![];

    // If f or s are less than their respective lengths, that means
    // we haven't visited the f'th or s'th element in the slice and
    // we're not done merging!
    while f < first_len || s < secnd_len {
        if f < first_len && s < secnd_len {
            // Simply compare the two.
            if first[f] <= second[s] {
                sorted.push(first[f]);
                f += 1;
            } else { 
                sorted.push(second[s]);
                s += 1;
            }
        }
        // We've copied all of the first slice's data in, so
        // just get the rest of the second slice's data.
        else if f >= first_len {
            sorted.push(second[s]);
            s += 1;
        } else {
            // Or maybe we've already copied all the second slice's
            // data, so we'll copy in the rest of the first slice.
            sorted.push(first[f]);
            f += 1;
        }
    }

    sorted
}

#[cfg(test)]
mod tests {
    use super::merge_sort;

    #[test]
    fn test_unsorted() {
        let mut arr = vec![1, 5, 3, 6, 2, 4];
        let expect = vec![1, 2, 3, 4, 5, 6];
        merge_sort(&mut arr);
        assert_eq!(arr, expect);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut arr = vec![ 10, 9, 8, 7, 6, 5, 4, 3, 2, 1 ];
        let expect = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        merge_sort(&mut arr);
        assert_eq!(arr, expect);
    }

    #[test]
    fn test_already_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let expect = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        merge_sort(&mut arr);
        assert_eq!(arr, expect);
    }

    #[test]
    fn test_partially_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5, 10, 8, 7, 9, 6];
        let expect = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        merge_sort(&mut arr);
        assert_eq!(arr, expect);
    }

    #[test]
    fn test_odd_length() {
        let mut arr = vec![2, 1, 3];
        let expect = vec![1, 2, 3];
        merge_sort(&mut arr);
        assert_eq!(arr, expect);
    }

    #[test]
    fn test_duplicates() {
        let mut arr = vec![3, 3, 4, 5, 4, 5, 1, 2, 2, 1];
        let expect = vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5];
        merge_sort(&mut arr);
        assert_eq!(arr, expect);
    }

    #[test]
    fn test_empty() {
        let mut arr: Vec<i32> = vec![];
        let expect = vec![];
        merge_sort(&mut arr);
        assert_eq!(arr, expect);
    }

    #[test]
    fn test_one() {
        let mut arr = vec![999];
        let expect = vec![999];
        merge_sort(&mut arr);
        assert_eq!(arr, expect);
    }
}
