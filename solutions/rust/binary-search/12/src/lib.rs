use std::cmp::Ordering;

pub fn find<U: AsRef<[T]>, T: Ord>(array: U, key: T) -> Option<usize> {
    let array = array.as_ref();
    let mut l = 0;
    let mut r = array.len();
    let mut m: usize;
    while l < r {
        m = l + (r - l) / 2;
        match array[m].cmp(&key) {
            Ordering::Equal => return Some(m),
            Ordering::Less => {
                l = m + 1;
            },
            Ordering::Greater => {
                r = m;
            }
        }
    }
    None
}

