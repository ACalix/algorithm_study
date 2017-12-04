// pub fn countInversions(num_list: &[i32]( {
//     if (num_list.len() < 2) { return 0 };
//     let mut first_half = num_list[0..(num_list.arr() / 2)].to_vec();
//     let mut second_half = num_list[(num_list.arr() / 2)..].to_vec();
//
//     // let x = countInversions(&first_half);
//     // let y = countInversions(//second half);
//     // let z = countSplitInversions(//x_array_result, //y_array_result);
//     return x + y + z;
// }

#[allow(dead_code)]
pub fn quicksort(num_list: &mut[i32]) {
    if num_list.len() == 1 { return };

   let pivot_index = partition(num_list);
   if pivot_index != 0 {
       quicksort(num_list.get_mut(..pivot_index).unwrap());
   }
    if pivot_index != (num_list.len() - 1) {
        quicksort(num_list.get_mut(pivot_index + 1..).unwrap());
    }
}

#[allow(dead_code)]
pub fn nth_largest(num_list: &[i32], mut n: usize) -> i32 {
    // Subtracting a usize beyond 0 results in panic
    if n != 0 { n -= 1 };
    let mut list = num_list.to_vec();
    let pivot_index = partition(list.as_mut_slice());

    return
        if n == pivot_index {
            list[pivot_index]
        } else if n > pivot_index {
            nth_largest(&list[(pivot_index + 1)..], n - pivot_index)
        } else {
            nth_largest(&list[..pivot_index], n)
        };
}

#[allow(dead_code)]
pub fn find_submax(unsorted_list: &[i32]) -> i32 {
    struct MaxPair {
        max: i32,
        submax: i32
    }

    let base_case = max_swap((unsorted_list[0], unsorted_list[1]));
    let mut max_pair: MaxPair = MaxPair { max: base_case.1, submax: base_case.0 };

    for i in 2..unsorted_list.len() {
        if unsorted_list[i] > max_pair.submax {
            max_pair.submax = unsorted_list[i];
            let sort_pair = max_swap((max_pair.submax, max_pair.max));
            max_pair.max = sort_pair.1;
            max_pair.submax = sort_pair.0;
        } else if unsorted_list[i] > max_pair.max {
            max_pair.submax = max_pair.max;
            max_pair.max = unsorted_list[i];
        }
    }
    return max_pair.submax;
}

#[allow(dead_code)]
pub fn partition(arr: &mut[i32]) -> usize {
    let pivot = approx_median(&arr);
    arr.swap(0, pivot);

    let mut j = 1;
    for i in 1..arr.len() {
        if arr[i] < arr[0] {
            arr.swap(j, i);
            j += 1;
        }
    }
    j -= 1;
    arr.swap(0, j);
    return j
}

// fn count_split_inversions(a: &[i32], b: &[i32]) -> i32 {
//     let mut inversions = 0;
//     let mut i = 0;
//     let mut j = 0;
//
//     for k in 0..(a.len() + b.len()) {
//         if a.len() == (i as usize) || b.len() == (j as usize) { return (inversions as i32) };
//         if a[i] > b[j] {
//             j += 1;
//         } else {
//             inversions = inversions + j;
//             i += 1;
//         }
//     }
//     return inversions as i32;
// }

fn approx_median(arr: &[i32]) -> usize {
    struct IndexList {
        first: usize,
        middle: usize,
        last: usize
    }

    let indexes: IndexList = IndexList {
        first: arr.len() - arr.len(),
        middle: if arr.len() % 2 == 0 { arr.len() / 2 - 1 } else { arr.len() / 2},
        last: arr.len() - 1
    };
    let (first, middle, last) =  (arr[indexes.first],
                                  arr[indexes.middle],
                                  arr[indexes.last]);

    let median = find_submax(&vec![first, middle, last]);
    if median == first {
        indexes.first
    } else if median == middle {
        indexes.middle
    } else {
        indexes.last
    }
}

fn max_swap<T: Ord>((x, y): (T, T)) -> (T, T) {
    return if y > x { (x, y) } else { (y, x) };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn returns_sorted_pair() {
        assert_eq!(max_swap((2, 4)), (2, 4));
        assert_eq!(max_swap((9, 3)), (3, 9));
        assert_eq!(max_swap((1, 0)), (0, 1));
    }

    #[test]
    fn sorts_base_case() {
        let mut test = vec![8, 3];
        assert_eq!(find_submax(&test), 3);
        test[0] = 0;
        assert_eq!(find_submax(&test), 0);
    }

    #[test]
    fn finds_second_biggest_num() {
        let test = vec![5, 3, 8, 2, 0, 7];
        assert_eq!(find_submax(&test), 7);
    }

    #[test]
    fn find_reasonable_pivot() {
        let before_middle = vec![5, 2, 4, 1];
        let middle = vec![7, 6, 3, 15, 2, 9, 5];
        assert_eq!(approx_median(&before_middle), 1);
        assert_eq!(approx_median(&middle), 0);
    }

    #[test]
    fn partitions_array_nums() {
        let mut num_arr = vec![7, 6, 3, 15, 2, 9, 5];
        partition(num_arr.as_mut_slice());
        assert_eq!(num_arr, vec![5, 6, 3, 2, 7, 9, 15]);
        let mut num_arr1 = vec![5, 2, 1, 9, 6, 4, 8, 7, 3];
        partition(num_arr1.as_mut_slice());
        assert_eq!(num_arr1, vec![3, 2, 1, 4, 5, 9, 8, 7, 6]);
        let mut num_arr2 = vec![5, 6, 3, 2];
        partition(num_arr2.as_mut_slice());
        assert_eq!(num_arr2, vec![2, 3, 5, 6]);
    }

    #[test]
    fn nth_largest_element() {
        let num_arr = vec![7, 6, 3, 15, 2, 9, 5];
        assert_eq!(nth_largest(&num_arr, 5), 7);
        let num_arr = vec![7, 6, 3, 15, 2, 9, 5];
        assert_eq!(nth_largest(&num_arr, 7), 15);
        let num_arr = vec![7, 6, 3, 15, 2, 9, 5];
        assert_eq!(nth_largest(&num_arr, 1), 2);
    }

    #[test]
    fn sort_array() {
        let mut num_arr = vec![7, 6, 3, 15, 2, 9, 5];
        quicksort(num_arr.as_mut_slice());
        assert_eq!(num_arr, vec![2, 3, 5, 6, 7, 9, 15]);
        let mut num_arr1 = vec![3, 2, 1, 4, 5, 9, 8, 7, 6];
        quicksort(num_arr1.as_mut_slice());
        assert_eq!(num_arr1, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let mut num_arr2 = vec![3];
        quicksort(num_arr2.as_mut_slice());
        assert_eq!(num_arr2, vec![3]);
    }

    // #[test]
    // fn split_inversions_count() {
    //     let num_arr = vec![3, 2, 1];
    //     let num_arr1 = vec![2, 1, 3];
    //     assert_eq!(count_split_inversions(&num_arr, &num_arr1), 6);
    // }
}
