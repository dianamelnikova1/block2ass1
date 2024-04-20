pub fn insert_sort<T, F>(arr: &mut [T], compare: F)
where
    F: Fn(&T, &T) -> bool,
{
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && compare(&arr[j], &arr[j - 1]) {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}
