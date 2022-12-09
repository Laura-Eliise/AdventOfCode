use std::{fs::read_to_string, collections::HashSet};

fn main() {
    println!("Part1: {}", part1("./src/info.txt"));
    println!("Part2: {}", part2("./src/info.txt"));
}

fn read_file(path: &str) -> String {
    let file = read_to_string(path);
    return match file {
        Ok(t) => t,
        Err(error) => panic!("Could not read the file: {:?}", error),
    };
}

fn part1(path: &str) -> u32 {
    let mut ans: u32 = 0;
    let data: String = read_file(path);

    for sack in data.split("\n") {
        let comps:(&str, &str) = sack.split_at(sack.len()/2);
        let mut comp2: HashSet<char> = HashSet::new();
        let mut comp1: Vec<char> = Vec::new();

        // remove duplicate letter from compartment 1
        for letter in comps.0.chars() {
            if !comp1.contains(&letter) {
                comp1.push(letter)
            }
        }

        // Fill the set on compartment 2 with letters
        for letter in comps.1.chars() {
            comp2.insert(letter);
        };

        // seeing if any of compartment 1 letters are in compartment 2
        for letter in comp1 {
            if comp2.contains(&letter) {
                let num = letter as u32;
                if num < 91 { // capital
                    ans += num -38
                } else { // lowercase
                    ans += num -96
                }
                break;
            }
        }
    }

    return ans
}

fn part2(path: &str) -> u32 {
    let mut ans: u32 = 0;
    let data: String = read_file(path);
    let arr: Vec<&str> = data.split("\n").collect();
    let mut i: usize = 0;

    while i < arr.len() {
        let mut sack1: Vec<char> = Vec::new();
        let mut sack2: HashSet<char> = HashSet::new();
        let mut sack3: HashSet<char> = HashSet::new();

        // remove duplicate letter from compartment 1
        for letter in arr[i].chars() {
            if !sack1.contains(&letter) {
                sack1.push(letter)
            }
        }

        // Fill the set on compartment 2 with letters
        for letter in arr[i+1].chars() {
            sack2.insert(letter);
        };
        for letter in arr[i+2].chars() {
            sack3.insert(letter);
        };

        // seeing if any of compartment 1 letters are in compartment 2
        for letter in sack1 {
            if sack2.contains(&letter) && sack3.contains(&letter) {
                let num = letter as u32;
                if num < 91 { // capital
                    ans += num -38
                } else { // lowercase
                    ans += num -96
                }
                break;
            }
        }
        
        i += 3;
    }

    return ans
}

