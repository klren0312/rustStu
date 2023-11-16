#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

// 拥有隐式辨别值（implicit discriminator，从 0 开始）的 enum
enum Number {
    Zero,
    One,
    Two,
}

// 拥有显式辨别值（explicit discriminator）的 enum
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}


fn main() {
    use Status::{Poor, Rich};
    use Work::*;
    let status = Poor;
    let work = Civilian;

    match status {
        Rich => println!("The rich have lots of money"),
        Poor => println!("The poor have no money...")
    }

    match work {
        // 再次注意到没有用完整路径。
        Civilian => println!("Civilians work!"),
        Soldier  => println!("Soldiers fight!"),
    }

    // `enum` 可以转成整型。
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);

}
