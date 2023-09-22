fn main() {
    let numbers = [5, 8, 2, 10, 30];
    let mut x = 0;
    let length = numbers.len();
    let mut greatest: i32 = numbers[0]; // Initialize with the first element

    while x < length {
        if numbers[x] > greatest {
            greatest = numbers[x]; // Update if the current element is greater
        }
        x = x + 1;
    }

    println!("{}", greatest);
}
