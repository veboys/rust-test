fn main() {
    let (a, mut b):(bool,bool) = (true, false);
    println!("a={:?}, b={:?}!", a, b);

    let mut array:[i32;3] = [0;3];
    array[1] = 1;
    array[2] = 2;

    for i in &array {
        println!("{}", i);
    }
}
