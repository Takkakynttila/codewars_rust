//Define String.prototype.toAlternatingCase (or a similar function/method such as
//to_alternating_case/toAlternatingCase/ToAlternatingCase
//in your selected language;
//see the initial solution for details)
//such that each lowercase letter becomes uppercase and each uppercase letter becomes lowercase.

fn main() {

    fn to_alternating_case(s: &str) -> String {
        let mut print:String = String::new(); // create empty string for result

        for c in s.chars() {
            if c != c.to_ascii_uppercase() {
                print = print + &c.to_ascii_uppercase().to_string();
            }
            else {
                print = print + &c.to_ascii_lowercase().to_string();
            }
        }
        return print;
    }
}

