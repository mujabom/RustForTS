struct Custom {
    age: u16,
    name: String,
}

enum Item {
    Number(u16),
    String(String),
    MyCustom(Custom),
}

fn append(items: &mut Vec<Item>) {
    items.push(Item::MyCustom(Custom {
        age: 20,
        name: "John".to_string(),
    }));
}
fn main() {
    let mut items: Vec<Item> = vec![];
    append(&mut items);
    //print first item
    match items.get(0) {
        Some(Item::String(s)) => println!("{}", s),
        Some(Item::Number(n)) => println!("{}", n),
        Some(Item::MyCustom(Custom { age, name })) => println!("{} {}", age, name),
        None => println!("No item found"),
    }
}
