fn main() {
    //my solution before course
   case1();

    //more changes
    case2();

}

fn case1(){
    let file = std::fs::read_to_string("src/project/lines.txt");
    match file {
        Ok(content) => {
            content.lines()
                .enumerate()
                .filter(|(i,_)| i % 2 == 0)
                .for_each(|(_,line)| {
                    println!(".{}", line);
                });
        }
        Err(_) => {
            println!("Failed to read file");
        }
    }
}

fn case2(){
    let file = std::fs::read_to_string("src/project/lines.txt");
    match file {
        Ok(content) => {
            content.lines()
                .enumerate()
                .filter(|(i,_)| i % 2 == 0)
                .skip(2)
                .take(2)
                .for_each(|(_,line)| {
                    println!(".{}", line);
                });
        }
        Err(_) => {
            println!("Failed to read file");
        }
    }
}