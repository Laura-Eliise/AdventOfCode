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
    let mut ans: u32 = 0;

    for pair in data.split("\n") {
        let elves: Vec<&str> = pair.split(",").collect();
        let elf1: Vec<&str> = elves[0].split("-").collect();
        let elf2: Vec<&str> = elves[1].split("-").collect();
        
        let e1_start = str_to_u32(elf1[0]);
        let e1_end = str_to_u32(elf1[1]);
        let e2_start = str_to_u32(elf2[0]);
        let e2_end = str_to_u32(elf2[1]);

        if e1_start <= e2_start && e1_end >= e2_end || 
           e1_start >= e2_start && e1_end <= e2_end {
            ans += 1;
        } 
    }

    return ans
}

fn part2(data: String) -> u32 {
    let mut ans: u32 = 0;

    for pair in data.split("\n") {
        let elves: Vec<&str> = pair.split(",").collect();
        let elf1: Vec<&str> = elves[0].split("-").collect();
        let elf2: Vec<&str> = elves[1].split("-").collect();
        
        let e1_start = str_to_u32(elf1[0]);
        let e1_end = str_to_u32(elf1[1]);
        let e2_start = str_to_u32(elf2[0]);
        let e2_end = str_to_u32(elf2[1]);

        if e1_start >= e2_start && e1_start <= e2_end ||
           e1_end >= e2_start && e1_end <= e2_end ||
           e2_start >= e1_start && e2_start <= e1_end ||
           e2_end >= e1_start && e2_end <= e1_end  {
            ans += 1;
        } 
    }

    return ans
}

fn str_to_u32(string: &str) -> u32 {
    return match string.parse::<u32>() {
        Ok(num) => num,
        Err(error) => panic!("couldn't parse string: {}", error)
    }
}