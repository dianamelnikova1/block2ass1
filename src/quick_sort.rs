pub fn quick_sort<T, F>(arr: &mut [T], compare: F)
where
    F: Fn(&T, &T) -> bool,
{
    if arr.len() <= 1 {
        return;
    }
    let pivot_index = partition(arr, &compare);
    quick_sort(&mut arr[0..pivot_index], &compare);
    quick_sort(&mut arr[pivot_index + 1..], &compare);
}

fn partition<T, F>(arr: &mut [T], compare: &F) -> usize
where
    F: Fn(&T, &T) -> bool,
{
    let pivot_index = arr.len() - 1;
    let mut i = 0;
    for j in 0..pivot_index {
        if compare(&arr[j], &arr[pivot_index]) {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, pivot_index);
    i
}
