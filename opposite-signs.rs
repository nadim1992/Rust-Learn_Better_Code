// Opposite signs

fn opposite_signs(x: i8, y: i8) -> bool {
    return ( x ^ y ) < 0;
}

fn main() {
    let x: i8 = 10;
    let y: i8 = -15;
    
    let result: bool = opposite_signs(x, y);
    
    if result {
        println!("Signs are opposite.");
    } else {
        println!("Signs are not opposite.");
    }
}
