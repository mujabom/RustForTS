fn main() {
    //my solution
    let mut c = vec![1, 2, 3];
    c = c.iter().map(|x| x + 1).collect();
    println!("{:?}", c);

    //copied from course sol:1
    let foo:Vec<usize> = vec![1, 2, 3].iter().map(|x| x + 1).collect();
    println!("{:?}",foo);

    //from course sol:2
    let data = vec![1,2,3];
    let mut iter = data.iter().map(|x|x+1);
    let mut collected= vec![];
    while let Some(val) = iter.next() {
        collected.push(val);
    }
    println!("yay it works{:?}",collected);
}