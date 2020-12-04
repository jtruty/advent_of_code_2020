use std::env;
use std::fs;

fn part1() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];    
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    
    let mut found = false;
    for x in contents.lines() {
        let int_x = x.parse::<i32>().unwrap();
        for y in contents.lines() {
            let int_y = y.parse::<i32>().unwrap();
            if int_x + int_y == 2020 {
                println!("x * y = {}", int_x * int_y);
                found = true;
                break;
            }
        }
        if found {
            break;
        }
    }
}

fn part2() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];    
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    
    let mut found = false;
    for x in contents.lines() {
        let int_x = x.parse::<i32>().unwrap();
        for y in contents.lines() {
            let int_y = y.parse::<i32>().unwrap();
            for z in contents.lines() {
                let int_z = z.parse::<i32>().unwrap();
                if int_x + int_y + int_z == 2020 {
                    println!("x * y * z = {}", int_x * int_y * int_z);
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
        if found {
            break;
        }
    }
}

fn main() {
    //part1();
    part2();
}
