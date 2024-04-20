pub fn merge_sort<T, F>(arr: &mut [T], compare: F)
where
    T: Clone,
    F: Fn(&T, &T) -> bool,
{
    let len = arr.len();
    if len <= 1 {
        return;
    }
    let mid = len / 2;
    let mut left = arr[..mid].to_vec();
    let mut right = arr[mid..].to_vec();

    merge_sort(&mut left, &compare);
    merge_sort(&mut right, &compare);

    merge(&mut left, &mut right, arr, &compare);
}

fn merge<T, F>(left: &mut Vec<T>, right: &mut Vec<T>, result: &mut [T], compare: &F)
where
    T: Clone,
    F: Fn(&T, &T) -> bool,
{
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if compare(&left[i], &right[j]) {
            result[k] = left[i].clone();
            i += 1;
        } else {
            result[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        result[k] = left[i].clone();
        i += 1;
        k += 1;
    }

    while j < right.len() {
        result[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}
