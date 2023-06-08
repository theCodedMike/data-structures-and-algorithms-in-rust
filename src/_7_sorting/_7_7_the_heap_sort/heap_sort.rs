/// 计算父节点下标宏
macro_rules! parent {
    ($child:ident) => {
        $child >> 1
    };
}

/// 计算左子节点下标宏
macro_rules! left_child {
    ($parent:ident) => {
        ($parent << 1) + 1
    };
}

/// 计算右子节点下标宏
macro_rules! right_child {
    ($parent:ident) => {
        ($parent + 1) << 1
    };
}

pub fn heap_sort(nums: &mut [i32]) {
    let len = nums.len();
    if len <= 1 {
        return;
    }

    let last_parent = parent!(len);
    // 第一次建大顶堆
    for i in (0..=last_parent).rev() {
        move_down(nums, i);
    }

    for end in (1..len).rev() {
        nums.swap(0, end);
        // 重建堆
        move_down(&mut nums[..end], 0);
    }
}

// 小的数据项下移
fn move_down(nums: &mut [i32], mut parent: usize) {
    let last = nums.len() - 1;
    loop {
        let left = left_child!(parent);
        let right = right_child!(parent);
        if left > last {
            break;
        }

        // right <= last 确保存在右子节点
        let child = if right <= last && nums[left] < nums[right] {
            right
        } else {
            left
        };

        // 子节点大于父节点，交换数据
        if nums[child] > nums[parent] {
            nums.swap(parent, child);
        }

        // 更新父子关系
        parent = child;
    }
}
