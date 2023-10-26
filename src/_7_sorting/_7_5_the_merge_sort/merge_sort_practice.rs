pub fn my_merge_sort<T: PartialOrd + Copy>(data: &mut [T]) {
    let len = data.len();
    if len < 2 {
        return;
    }
    let mid = len / 2;
    my_merge_sort(&mut data[..mid]);
    my_merge_sort(&mut data[mid..]);
    merge(data, mid);
}

fn merge<T: PartialOrd + Copy>(data: &mut [T], mid: usize) {
    let left = data[..mid].to_vec();
    let right = data[mid..].to_vec();
    let mut l = 0;
    let mut r = 0;

    for v in data {
        if r == right.len() || (l < left.len() && left[l] < right[r]) {
            *v = left[l];
            l += 1;
        } else {
            *v = right[r];
            r += 1;
        }
    }
}
