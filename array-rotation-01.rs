// Array rotation (Using temp array)

fn left_rotate_by_one(numbers: &mut [i8], length: usize) {
    let mut index: usize = 0;
    let temp = numbers[0];

    while index < length - 1 {
        numbers[index] = numbers[index + 1];
        index = index + 1;
    }

    numbers[index] = temp;
}

fn left_rotate(numbers: &mut [i8], rotate: i8, length: usize) {
    let mut index: i8 = 0;

    while index < rotate {
        left_rotate_by_one(numbers, length);
        index = index + 1;
    }
}

fn main() {
    let mut numbers: [i8; 7] = [1, 2, 3, 4, 5, 6, 7];
    let length: usize = numbers.len();
    let rotate: i8 = 2;

    left_rotate(&mut numbers, rotate, length);

    println!("{:?}", numbers);
}
