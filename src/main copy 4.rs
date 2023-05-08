fn manel(numbs: Vec<usize>, index: usize) -> usize {
    // let value = numbs.get(index);
    // return match value {
    //     Some(x) => x * 5,
    //     None => index * 5,
    // };

    return numbs.get(index).unwrap_or(&index) * 5;
}

fn main() {
    let nums = vec![1, 2, 3];

    println!("{:?}", manel(nums, 1));
}
