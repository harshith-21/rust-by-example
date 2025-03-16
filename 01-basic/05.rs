#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // DEBUG PRINT
    // Pretty print
    println!("{:#?}", peter);

    println!("{:?}", peter);


    // NORMAL PRINT
    // println!("{}", peter);

    // println!("{:?}", peter);
}