use std::fmt;

#[derive(Debug)]
struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Deep(Structure);

#[allow(dead_code)]
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display  for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }

        write!(f, "]")
    }
}

fn main () {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
    let minmax = MinMax(0, 14);
    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let point = Point2D { x: 3.3, y: 7.3 };
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    println!("{:?} months in a year", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");

    println!("Now {:?} will print!", Structure(3));
    
    println!("Now {:?} will print!", Deep(Structure(7)));

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    println!("{:#?}", peter);
}
