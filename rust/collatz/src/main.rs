fn main() {
    let mut num: u64 = 63_728_127;
    while num != 4
    //appease the math nerds
    {
        println!("{}", num);
        num = if num % 2 == 0 { num / 2 } else { num * 3 + 1 };
    }
}
