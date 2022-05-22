//You ask a small girl,"How old are you?" She always says, "x years old",
//where x is a random number between 0 and 9.

//Write a program that returns the girl's age (0-9) as an integer.

//Assume the test input string is always a valid string. For example, the test input may be "1 year old" or "5 years old".
//The first character in the string is always a number.

fn main() {
    fn get_age(age: &str) -> u32 {
        // Your code here
        let c = age.chars().nth(0).unwrap();
        let c_as_string = c.to_string();
        let c_as_number = c_as_string.parse::<u32>().unwrap();
        return c_as_number;
    }
    let x = get_age("2 asdasdasd");
    println!("{}", x);
}
