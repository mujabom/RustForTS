use std::collections::HashMap;

fn main() {
    //my solution
    let mut c = vec![1, 2, 3];
    c = c.iter().map(|x| x + 1).collect();
    println!("{:?}", c);

    //copied from course sol:1
    let foo: Vec<usize> = vec![1, 2, 3].iter().map(|x| x + 1).collect();
    println!("{:?}", foo);

    //from course sol:2
    let data = vec![1, 2, 3];
    let mut iter = data.iter().map(|x| x + 1);
    let mut collected = vec![];
    while let Some(val) = iter.next() {
        collected.push(val);
    }
    println!("yay it works{:?}", collected);

    //my sol 2 using into_iter
    // let res:Vec<i32> = vec![1,2,3].into_iter().map(|x| x+1).collect();

    println!("my directly mutated map {:?}", vec![1, 2, 3].into_iter().map(|x| x + 1).collect::<Vec<i32>>());
    test_hash_map()
}


fn test_hash_map() {
    let map: HashMap<usize, &str> = vec!["this", "is", "test"].into_iter().enumerate().map(|(idx, value)| (idx, value)).collect();
    println!("{:?}", map)
}