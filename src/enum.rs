#![allow(dead_code)]

enum WebEvent {
    PageLoad,
    KeyPress(char),
}


fn inspect (event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::KeyPress(c) => println!("pressed {}", c),
    }
}

fn main () {
    let pressed = WebEvent::KeyPress('x');
    let load = WebEvent::PageLoad;

    inspect(pressed);
    inspect(load);
}