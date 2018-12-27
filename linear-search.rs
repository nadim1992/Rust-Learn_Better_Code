// Linear search

fn search(numbers: &[i32], find: i32) -> i32 {
    let mut index: i32 = 0;

    for item in numbers.iter() {
        if *item == find {
            return index;
        }

        index += 1;
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
        println!("Item not found!");
    }
}
