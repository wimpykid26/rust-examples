pub fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

pub fn callhof() {
    let upper = 1000;
    let sum_off_odd = (0..)
        .map(|n| n * n)
        .take_while(|&n_squared| n_squared < upper)
        .filter(|&n_squared| is_odd(n_squared))
        .fold(0, |acc, n_squared| acc + n_squared);
    println!("Sum of squared odd numbers is {}", sum_off_odd);
}
