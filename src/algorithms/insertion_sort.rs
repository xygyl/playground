pub fn insertion_sort<T: Copy + Ord>(arr: &mut [T]) {
    for j in 1..arr.len() {
        let i = j - 1;
        let cur = arr[j];
        while i > 0 && arr[i] > cur {
            arr[i] = arr[j];
            i -= 1;
        }
        arr[j] = cur;
    }
}
