fn main(){
    let num = vec![1,2,3,4,5];
    let index = 7;
    let result = practice(num,index);
    println!("The result is {}",result);
}

fn practice(num:Vec<usize>,index:usize)->usize{
    return num.get(index).unwrap_or(&index) * 5;
}