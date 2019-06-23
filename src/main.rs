#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    // Printing with '{:?}' is similar to with '{}'
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Schwarzeneggrolls",
             "AhhnohlDD",
             actor = "gubernator's");

    println!("Now {:?} will print", Structure(3));
    println!("Now the deep {:?} shall print!", Deep(Structure(32)) );

    // Pretty printing
    let name = "Mary";
    let age = 17;
    let madonna = Person{name, age};

    println!("Pretty name, {:#?}", madonna);
}

