pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mut l = 0;
    let mut r = array.len();
    dbg!(l);
    dbg!(r);
    while l < r {
        let m = l + (r - l) / 2;
        dbg!(m);
        let mid = array[m];
        dbg!(mid);
        if key == mid {
            return Some(m);
        } else if key > mid {
            l = (mid as usize) + 1;
        } else {
            r = (mid as usize) + 1;
        }
    }
    return None
}
