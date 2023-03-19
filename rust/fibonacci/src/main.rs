fn main() {
    let mut num1: u64 = 0;
    let mut num2: u64 = 1;

    for _ in 0..10 {
        num2 = num1 + num2;
        num1 = num2 - num1;
        println!("{}", num1);
    }
}
