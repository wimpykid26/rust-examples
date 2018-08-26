pub fn simple() {
    fizzbuzz(120);
}

fn divisible_by(a: u32, b: u32) -> bool {
    if b == 0 {
        return false;
    }

    a % b == 0
}

fn fizzbuzz(n: u32) -> () {
    match divisible_by(n, 20) {
        true => println!("Numbers are mutually divisible"),
        false => println!("Numbers are not mutually divisible"),
    }
}
