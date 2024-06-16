fn main() {
    //my solution before course
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
