#[derive(Debug)]
struct Item {
    count: i32,
}

fn add_one(mut item: Item) -> Item {
    item.count += 1;
    item
}

fn main() {
    let item = Item { count: 42 };
    println!("{:?}", item);
    let mut item = add_one(item);
    item.count += 5;
    println!("{:?}", item);
    // println!("{:?}", item);
}
