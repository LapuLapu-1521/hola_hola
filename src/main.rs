fn main() {
    // This is an example of a line comment
    // Notice how there are two slashes at the beginning of the line
    println!("Hello, world!");
    println!("I'm a Rustacean!");

    println!("{} days", 31);
    println!("{0}, this is {1}. {1} this is {0}", "Alice", "Bob");

    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // Special formatting
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    // output "     1"
    println!("{number:>width$}", number=1, width=6);
    // output "000001"
    println!("{number:>0width$}", number=1, width=6);
    println!("My name is {0}, {1} {0}", "Bond", "James");

    #[allow(dead_code)]
    struct Structure(i32);

//    println!("This struct `{}` won't print...", Structure(3));

    println!("Pi is roughly {:.*}", 3, 3.14159365358979);

}

