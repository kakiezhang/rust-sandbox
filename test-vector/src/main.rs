fn main() {
    let mut vs = vec![88, 17, 34, 47, 78];
    while vs.len() > 0 {
        let e = vs.remove(0);
        println!("e: {:?}", e);
    }
}
