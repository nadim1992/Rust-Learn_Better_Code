// Find missing number

fn get_missing_no(numbers: &[usize], length: usize) -> usize {
    let mut index: usize = 0;
    let mut total: usize = (length + 1) * (length + 2) / 2;

    while index < length {
        total -= numbers[index];
        index += 1;
    }

    return total;
}

fn main() {
    let numbers: [usize; 5] = [1, 2, 4, 5, 6];
    let length: usize = numbers.len();
    
    let result: usize = get_missing_no(&numbers, length);
    
    println!("Missing number is {}.", result);
}
