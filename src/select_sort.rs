pub fn select_sort<T, F>(arr: &mut [T], compare: F)
where
    F: Fn(&T, &T) -> bool,
{
    for i in 0..arr.len() {
        let mut min_index = i;
        for j in i + 1..arr.len() {
            if compare(&arr[j], &arr[min_index]) {
                min_index = j;
            }
        }
        if i != min_index {
            arr.swap(i, min_index);
        }
    }
}
