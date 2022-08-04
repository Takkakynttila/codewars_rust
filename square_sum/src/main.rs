fn main() {
    fn square_sum(vec: Vec<i32>) -> i32 {
        let mut x = 0;
        for i in vec {
            let h = i32::pow(i, 2);
            x = x + h;
        }
        x
    }
    let y = square_sum(vec![5, 3, 4]);
    println!("{}", y);
}
