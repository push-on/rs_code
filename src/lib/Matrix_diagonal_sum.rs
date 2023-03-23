fn difference(arr: &[[i32; 3]; 3]) -> i32 {
    let n = arr.len();
    let mut d1 = 0;
    let mut d2 = 0;

    for i in 0..n {
        for j in 0..n {
            if i == j {
                d1 += arr[i][j];
            }
            if i == n - j - 1 {
                d2 += arr[i][j];
            }
        }
    }

    return (d1 - d2).abs();
}

fn main() {
    let arr = [[11, 2, 4], [4, 5, 6], [10, 8, -12]];
    println!("{}", difference(&arr));
}
