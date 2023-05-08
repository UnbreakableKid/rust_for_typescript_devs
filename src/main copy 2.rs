struct Custom {
    age: usize,
    name: String,
}

enum Item {
    Number(usize),
    String(String),
    MyCustom(Custom),
}

fn append(items: &mut Vec<Item>) {
    items.push(Item::String("hello fem".into()));
}

fn main() {
    let mut manel: Vec<Item> = vec![Item::String("test".into())];

    append(&mut manel);
}
