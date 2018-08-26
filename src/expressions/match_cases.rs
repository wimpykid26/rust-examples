#[allow(dead_code)]
pub enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
}

pub fn enum_match() {
    let color = Color::RGB(17, 32, 21);

    match color {
        Color::Red => println!("Red"),
        Color::Blue => println!("Blue"),
        Color::Green => println!("Green"),
        Color::RGB(r, g, b) => println!("Red : {} Green : {} Blue : {}", r, g, b),
    }
}

pub fn ref_match() {
    let value = &5;
    match value {
        &val => println!("Got a value from a reference {:?}", val),
    }
    //Dereferencing to get actual value of reference
    match *value {
        val => println!("Got a value after derefencing {}", val),
    }

    //Alternatively can create reference using ref operator

    let ref reference = 5;

    let value_1 = 72;
    let mut value_2 = 34;

    match value_1 {
        //Use ref keyword to get reference
        ref val => println!("Got a value from a reference {}", val),
    }

    let x = match value_2 {
        ref mut m => {
            *m = *m + 10;
        }
    };

    println!("{:?}", x);
}
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
