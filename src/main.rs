fn main() {
    let list: Vec<i32> = vec![0, 1, 2, 3];

    let list: Vec<_> = list.iter().map(|x| x + 1).collect();

    print!("{:?}", list);
}
