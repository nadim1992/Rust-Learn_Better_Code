// Binary search (Iterative)

fn search(numbers: &[i32], find: i32) -> i32 {
    let mut left: usize = 0;
    let mut right: usize = numbers.len();

    while left <= right {
        let mid: usize = left + (right - left) / 2;

        if numbers[mid] == find {
            return mid as i32;
        }

        if numbers[mid] < find {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    return -1;
}

fn main() {
    let numbers: [i32; 11] = [3, 17, 21, 22, 23, 29, 65, 82, 113, 243, 258];
    let find: i32 = 23;

    let result: i32 = search(&numbers, find);

    if result != -1 {
        println!("Item found at index {}.", result);
    } else {
        println!("item not found");
    }
}
