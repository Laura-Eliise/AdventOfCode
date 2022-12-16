use std::{fs::read_to_string};

fn main() {
    let data = read_file("./src/info.txt");

    println!("Part 1 answer is {}.", part1(data.clone()));
    println!("Part 2 answer is {}.", part2(data.clone()));
}

// [["N", "Z"], ["D", "C", "M"], ["P"]]
// [[1, 2, 1], [3, 1, 3], [2, 2, 1], [1, 1, 2]]


fn part1(raw_data: String) -> String {
    let mut ans: String = String::new();
    let (mut crates, orders) = process_data(raw_data);

    for order in orders {
        for _ in 0..order[0] {
            let mut temp = crates[order[2]-1].clone();
            temp.insert(0, crates[order[1]-1][0].clone());
            crates[order[1]-1].remove(0);
            crates[order[2]-1] = temp;
        }
    }

    for col in crates {
        if col.len() > 0 {
            ans += &col[0];
        }
    }
    return ans
}

fn part2(raw_data: String) -> String {
        let mut ans: String = String::new();
    let (mut crates, orders) = process_data(raw_data);

    for order in orders {
        for i in 0..order[0] {
            let mut temp = crates[order[2]-1].clone();
            temp.insert(i, crates[order[1]-1][0].clone());
            crates[order[1]-1].remove(0);
            crates[order[2]-1] = temp;
        }
    }

    for col in crates {
        if col.len() > 0 {
            ans += &col[0];
        }
    }
    return ans
}

fn read_file(path: &str) -> String {
    let file = read_to_string(path);
    return match file {
        Ok(t) => t,
        Err(error) => panic!("Could not read the file: {:?}", error),
    };
}
fn process_data(data: String) -> (Vec<Vec<String>>, Vec<[usize; 3]>) {
    let (raw_crates, raw_orders) = data.split_once("\n\n").unwrap();
    let mut crates: Vec<Vec<String>> = Vec::new();
    let mut orders: Vec<[usize; 3]> = Vec::new();

    /* --- Processing create input into vector of vectors --- */
    let rows: Vec<&str> = raw_crates.split("\n").collect();

    for char in rows[rows.len()-2].chars() {
        if char == ' ' {
            crates.push(Vec::new());
        }
    }
    crates.push(Vec::new());

    for index in 0..rows.len() {
        if index == rows.len()-1 { break }
        let row = rows[index];
        let mut step = 1;

        for i in 0..crates.len() {
            let char = row.get(step..step+1).unwrap();
            if char != " " { crates[i].push(char.to_string()) }
            step += 4;
        }
    }

    /* --- Processing order input into a vector --- */
    for row in raw_orders.split("\n") {
        let words: Vec<&str> = row.split(" ").collect();
        orders.push([
            words[1].parse::<usize>().unwrap(),
            words[3].parse::<usize>().unwrap(),
            words[5].parse::<usize>().unwrap(),
        ])

    }

    return (crates, orders)
}