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

    for i in 0..data.len() {
        let mut is_unique = true;
        let mut sub: Vec<&str> = data.get(i..i+4).unwrap().split("").collect();
        sub.sort();

        for i in 2..5 {
            if sub[i] == sub[i+1] {
                is_unique = false;
                break;
            }
        }

        if is_unique {
            return i as u32 + 4
        }
    }

    return ans
}
fn part2(data: String) -> u32 {
    let ans: u32 = 0;

    for i in 0..data.len() {
        let mut is_unique = true;
        let mut sub: Vec<&str> = data.get(i..i+14).unwrap().split("").collect();
        sub.sort();

        for i in 2..15 {
            if sub[i] == sub[i+1] {
                is_unique = false;
                break;
            }
        }

        if is_unique {
            return i as u32 + 14
        }
    }

    return ans
}