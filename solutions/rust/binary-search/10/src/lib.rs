pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mut l: i32 = 0;
    let mut r: i32 = (array.len() as i32) - 1;
    while l <= r {
        let m = (l + r) / 2;
        let mid = array[m as usize];
        if key == mid {
            return Some(m as usize);
        } else if key > mid {
            l = m + 1;
        } else {
            r = m - 1;
        }
    }
    None
}

