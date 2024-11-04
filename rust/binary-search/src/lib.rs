pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let min = 0;
    let max = array.len();
    if array.is_empty() || (array.len() == 1 && array[0] != key) {
        return None;
    }

    let middle = (max - min) / 2 + min;
    if array[middle] == key {
        return Some(middle);
    }

    if array[middle] > key {
        return find(&array[min..middle], key);
    }
    find(&array[middle..max], key).map(|x| x + middle)
}
