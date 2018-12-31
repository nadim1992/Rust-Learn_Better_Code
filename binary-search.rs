// Binary search

fn search(numbers: &[i32], left: usize, right: usize, find: i32) -> i32 {
    if right >= left {
        let mid: usize = left + (right - left) / 2;

        if numbers[mid] == find {
            return mid as i32;
        }

        if numbers[mid] > find {
            return search(&numbers, left, mid - 1, find);
        }

        return search(&numbers, mid + 1, right, find);
    }

    return -1;
}

fn main() {
    let numbers: [i32; 11] = [3, 17, 21, 22, 23, 29, 65, 82, 113, 243, 258];
    let length: usize = numbers.len();
    let find: i32 = 23;

    let result: i32 = search(&numbers, 0, length - 1, find);

    if result != -1 {
        println!("Item found at index {}.", result);
    } else {
        println!("item not found");
    }
}
