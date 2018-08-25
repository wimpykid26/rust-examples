pub fn infinite() {
    let mut count = 0u32;

    loop {
        count = count + 1;
        if count == 3 {
            println!("count is 3");
            continue;
        }

        println!("{}", count);

        if count == 5 {
            break;
        }
    }
}

pub fn labelled_loop() {
    'outer: loop {
        println!("Entered the outer body");

        'inner: loop {
            println!("Entered the inner body");
            break 'outer;
        }
        println!("This point will never be reached");
    }
}
