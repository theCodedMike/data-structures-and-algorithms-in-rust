///
/// 归并排序
///
pub fn merge_sort(nums: &mut [i32]) {
    if nums.len() > 1 {
        let mid = nums.len() / 2;
        // 排序前半部分
        merge_sort(&mut nums[..mid]);
        // 排序后半部分
        merge_sort(&mut nums[mid..]);
        // 合并排序结果
        merge(nums, mid);
    }
}
fn merge(nums: &mut [i32], mid: usize) {
    // 标记前半部分数据
    let mut i = 0;
    // 标记后半部分
    let mut k = mid;
    let mut temp = vec![];

    let len = nums.len();
    for _ in 0..len {
        if k == len || i == mid {
            break;
        }

        // 将数据放在临时集合temp
        if nums[i] < nums[k] {
            temp.push(nums[i]);
            i += 1;
        } else {
            temp.push(nums[k]);
            k += 1;
        }
    }

    // 合并的两部分数据长度大概率不一样长
    // 所以要将未处理完集合的数据全部加入
    if i < mid && k == len {
        for j in i..mid {
            temp.push(nums[j]);
        }
    } else if i == mid && k < len {
        for j in k..len {
            temp.push(nums[j]);
        }
    }

    // temp数据放回nums，完成排序
    for j in 0..len {
        nums[j] = temp[j];
    }
}
