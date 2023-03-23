fn main() {
    let list = [27, 58, 5, 10, 17, 9, 87, 4, 15, 50, 24, 3, 70, 2, 0];

    let mut x = list[0];
    let mut _y: i32;

    for i in 0..list.len() {
        _y = list[i];
        if x > _y {
            x = _y
        }
    }
    print!("smallest: {}", x)
}
