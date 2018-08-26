use std::mem;

pub fn capturing() {
    let color = "green";

    let print = || println!("{}", color);
    print();
    //Since println is not restrictive, color can be used again.
    println!("{}", color);
    let mut count = 0;
    //mutable borrow occurs here
    let mut increment = || {
        //since we have used count, mutable borrow occurs and now count is in increment's scope
        count = count + 10;
        println!("{}", count);
    };

    increment();
    //Cannot borrow mutable again.
    // let reborrow = &mut count;

    let movable = Box::new(3);
    let consume = || {
        println!("Movable is {}", movable);
        mem::drop(movable);
    };
    consume();
    //Cannot use movable again as consume takes ownership of movable.
    // println!("{}", movable);
}
