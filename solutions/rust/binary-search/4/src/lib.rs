pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mut vec = array.to_vec();
    vec.sort();
    let mut l = 0;
    let mut r = vec.len();
    while l < r {
        let m = l + (r - l) / 2;
        let mid = vec[m];
        if key == mid {
            return Some(m);
        } else if key > mid {
            l = m + 1;
        } else {
            r = m - 1;
        }
    }
    return None
}

