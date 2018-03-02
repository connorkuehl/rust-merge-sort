use std::vec::Vec;

pub fn merge_sort<T: Copy + PartialOrd>(arr: &mut [T]) {
    if arr.len() == 0 {
        return;
    }

    let sorted = merge_helper(arr);
    arr.copy_from_slice(sorted.as_slice());
}

fn merge_helper<T: Copy + PartialOrd>(arr: &mut [T]) -> Vec<T> {
    let len = arr.len();
    let mut sorted = Vec::new();
    if len == 1 {
        sorted.push(arr[0]);
        return sorted;
    }

    let left  = merge_helper(&mut arr[0..(len/2)]);
    let right = merge_helper(&mut arr[(len/2)..len]);
    sorted = merge(left, right);

    sorted
}

fn merge<T: Copy + PartialOrd>(first: Vec<T>, second: Vec<T>) -> Vec<T> {
    let first_len = first.len();
    let secnd_len = second.len();

    let mut f = 0;
    let mut s = 0;

    let mut sorted = vec![];

    while f < first_len || s < secnd_len {
        if f < first_len && s < secnd_len {
            if first[f] <= second[s] {
                sorted.push(first[f]);
                f += 1;
            } else { 
                sorted.push(second[s]);
                s += 1;
            }
        }
        else if f >= first_len {
            sorted.push(second[s]);
            s += 1;
        } else {
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
}
