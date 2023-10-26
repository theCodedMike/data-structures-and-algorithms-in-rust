pub fn my_quick_sort<T>(data: &mut [T])
where
    T: PartialOrd + Sized + Clone,
{
    quick_sort(data, 0, data.len() as isize - 1);
}

fn quick_sort<T>(data: &mut [T], start: isize, end: isize)
where
    T: PartialOrd + Sized + Clone,
{
    if start < end {
        let pivot = partition(data, start, end);
        quick_sort(data, start, pivot - 1);
        quick_sort(data, pivot + 1, end);
    }
}

fn partition<T>(data: &mut [T], mut start: isize, mut end: isize) -> isize
where
    T: PartialOrd + Sized + Clone,
{
    let pivot = data[start as usize].clone();
    while start < end {
        while start < end && data[end as usize] >= pivot {
            end -= 1;
        }
        data.swap(start as usize, end as usize);
        while start < end && data[start as usize] <= pivot {
            start += 1;
        }
        data.swap(start as usize, end as usize);
    }
    start
}
