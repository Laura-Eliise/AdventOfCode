use std::{fs::read_to_string, collections::HashMap};

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

fn find_dir_sizes(data: String) -> HashMap<String, u64> {
    let mut path: Vec<&str> = Vec::new();
    let mut dir_size: HashMap<String, u64> = HashMap::new();

    for raw_row in data.split("\n") {
        let row: Vec<&str> = raw_row.split(" ").collect();
        if row[0] == "$" {
            match row[1] {
                "cd" => {
                    match row[2] {
                        ".." => { path.pop(); },
                        _ => { path.push(row[2].clone()); }
                    }
                },
                _ => { continue; }
            }
        } else {
            if row[0] == "dir" { continue }
            let mut temp_path = String::from("/");
            
            for i in 0..path.len() {
                if i != 0 {
                    temp_path = temp_path.clone() + path[i] + "/"
                }

                let num: u64 = row[0].parse().unwrap();
                match dir_size.remove_entry(&temp_path) {
                    Some(val) => { dir_size.insert(temp_path.clone(), val.1 + num); },
                    None => { dir_size.insert(temp_path.clone(), num); },
                }
            }
        }
    }

    return dir_size
}

fn part1(data: String) -> u64 {
    let mut ans: u64 = 0;
    let dir_size = find_dir_sizes(data);    

    for (_, value) in dir_size.iter() {
        if value < &100000 {
            ans += value;
        }
    }

    return ans
}

fn part2(data: String) -> u64 {
    let mut ans: u64 = 70000001;
    let dir_size = find_dir_sizes(data);
    let free_space = 70000000 - dir_size.get("/").unwrap();

    for (_, value) in dir_size.iter() {
        if free_space + value > 30000000 && value < &ans {
            ans = *value;
        }
    }

    return ans
}