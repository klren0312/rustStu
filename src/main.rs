fn main() {
    println!("Hello, world!");
    println!("{} days", -128i8);
    println!("{0}, this is {1}", "test1", "laji");
    println!("{} of {:b} people know binary, the other half don't", 1, 3);

    println!("{number:>width$}", number=1, width=210);

     // 创建一个包含单个 `i32` 的结构体（structure）。命名为 `Structure`。
     #[allow(dead_code)]
     #[derive(Debug)]
     struct Structure(i32);
 
     // 但是像结构体这样的自定义类型需要更复杂的方式来处理。
     // 下面语句无法运行。
     println!("This struct `{:?}` won't print...", Structure(3));

    println!("Pi is roughly {pi:.*}", 3, pi = 3.1415926)
}
