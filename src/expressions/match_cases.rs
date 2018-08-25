pub fn basic_match() {
    let number = 13;

    match number {
        1 => println!("This is a singular value"),
        1 | 2 | 3 => println!("This is either 1, 2 or 3"),
        12...19 => println!("Teen"),
        _ => println!("Something other than a number"),
    }

    let boolean = false;
    //Can also store match expressions
    let binary = match boolean {
        //Should provide all match cases
        false => 0,
        true => 1,
    };
    println!("{} -> {}", boolean, binary);
}
