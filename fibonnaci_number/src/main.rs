fn main() {
    let mut a: i128 = 0;
    let mut b: i128 = 1;  
    let mut c: i128;

    loop {
        c = a + b;
        println!("{}", c);
        b = a;
        a = c;
    }
}
