//Make a function that will return a greeting statement that uses an input; your program should return,
//"Hello, <name> how are you doing today?".

//[Make sure you type the exact thing I wrote or the program may not execute properly]

fn main() {
    fn greet(name: &str) -> String {
        return format!("Hello, {} how are you doing today?", name);
    }
    let x = greet("Jorge");
    println!("{}", x);
}
