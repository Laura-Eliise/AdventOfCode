use std::fs::read_to_string;

fn main() {
    let data = read_file("./src/info.txt");

    println!("Part 1 answer is {}.", part1(data.clone()));
    println!("Part 2 answer is {}.", part2(data.clone()));
}

fn read_file(path: &str) -> String {
    let file = read_to_string(path);
    return match file {
        Ok(t) => t,
        Err(error) => panic!("Could not read the file: {:?}", error),
    };
}

fn part1(data: String) -> u32 {
    let ans: u32 = 0;

    return ans
}
fn part2(data: String) -> u32 {
    let ans: u32 = 0;

    return ans
}