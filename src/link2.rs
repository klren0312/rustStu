fn main() {
    let reference = &4;
    match reference {
        &val => println!("Got a value via destructuring: {:?}", val)
    }

    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val)
    }
}