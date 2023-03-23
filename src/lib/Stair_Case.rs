/*
 *
 **
 ***
 ****
 *****
*/

fn main() {
    let n = 9;

    for i in 1..n {
        for _j in 1..n - i {
            print!(" ")
        }
        for _j in 0..i {
            print!("*")
        }
        println!("")
    }
}
