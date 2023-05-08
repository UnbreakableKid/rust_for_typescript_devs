fn main() {
    let filename = std::env::args()
        .nth(1)
        .expect("the file name must be passed");

    let file = std::fs::read_to_string(filename).expect("unable to read the file to string");

    file.lines().for_each(|f| {
        if let Ok(x) = f.parse::<usize>() {
            println!("{}", x);
        } else {
            println!("Line not a number")
        }
    })
}
