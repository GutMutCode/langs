fn main() {
    // test::main();
    let v = vec![1, 2, 3, 4, 5];
    let mut v2 = v[..].to_vec();

    v2[1] = 1;

    println!("{:?}", v);
}
