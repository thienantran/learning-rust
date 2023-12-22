Variable declaration:
rust

let x: i32 = 10;
let mut y: i32 = 20; // 'mut' makes the variable mutable

2. Functions:
   ```rust
fn main() {
    println!("Hello, world!");
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

    Control flow:
        If-else:
        rust

if x > 5 {
    println!("x is greater than 5");
} else if x == 5 {
    println!("x is equal to 5");
} else {
    println!("x is less than 5");
}

   * Loops:
     ```rust
let mut counter = 0;
loop {
    counter += 1;
    println!("Counter: {}", counter);
    if counter == 10 {
        break;
    }
}

for i in 0..10 {
    println!("i: {}", i);
}

    Data structures:
        Structs:
        rust

struct Point {
    x: f32,
    y: f32,
}

let p = Point { x: 3.0, y: 4.0 };
println!("Point: ({}, {})", p.x, p.y);

   * Enums:
     ```rust
enum Shape {
    Circle(f32),
    Rectangle(f32, f32),
}

let c = Shape::Circle(2.0);
let r = Shape::Rectangle(3.0, 4.0);

    Traits:
    rust

trait Animal {
    fn make_sound(&self);
}

struct Dog;
impl Animal for Dog {
    fn make_sound(&self) {
        println!("Woof!");
    }
}

6. Error handling:
   ```rust
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("test.txt")?;
    // ...
    Ok(())
}

    Closures:
    rust

let closure = |x: i32| x * 2;
println!("Closure result: {}", closure(5));

8. Macros:
   ```rust
macro_rules! print_args {
    ($($arg:expr),*) => {
        ($($arg),*).iter().for_each(|arg| {
            println!("{:?}", arg);
        })
    }
}

fn main() {
    let x = 10;
    let y = 20;
    print_args!(x, y);
}
