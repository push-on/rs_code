fn main() {
    let list = [27, 58, 5, 10, 17, 9, 87, 87, 4, 15, 50, 24, 87, 70, 2, 0];
    let max_val = list.iter().max().unwrap();
    let n = list.iter().filter(|&x| x == max_val).count();
    println!("{}", n);
}
