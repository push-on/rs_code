use std::io;
fn get_input() -> String {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    input 
}
fn str_to_sorted_arr(input_str: &str) -> Vec<i32> {
    let mut a: Vec<i32> = input_str 
        .split_whitespace() 
        .map(|s| s.parse().unwrap()) 
        .collect(); 
    a.sort(); 
    a 
}

fn main() {
    let input = get_input(); 
    let a = str_to_sorted_arr(&input); 

    let sum1: i32 = a.iter().take(4).sum(); 
    let sum2: i32 = a.iter().skip(1).sum(); 
    println!("{} {}", sum1, sum2); 
}
