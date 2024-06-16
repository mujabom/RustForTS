fn main(){
    let result = multiply_with_5(Some(1034388888));

    match result {
        Some(num) => println!("Result is {}", num),
        None => println!("No result")
    }
}

fn multiply_with_5(inp:Option<u32>) -> Option<usize> {
    let multiplier:usize = 334535345;
    Some(inp? as usize * multiplier)
}