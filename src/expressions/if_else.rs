pub fn test() {
    let n = 5;
    if n > 0 {
        println!("{} is positive", n);
    } else if n < 0 {
        println!("{} is negative", n);
    } else {
        println!("{} is zero", n);
    }

    let big_n = if n > 0 { n * 10 } else { 0 };
    println!("{} - {}", n, big_n);
}
