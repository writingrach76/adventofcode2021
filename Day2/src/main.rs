use std::fs::File;
use std::io::{BufReader, BufRead};
use std::vec::Vec;

fn main() {
    let filename = String::from("input.txt");
    let mut submarine = SubmarineLocator{x:0, depth:0, aim:0};
    submarine = find_cords(filename, submarine);
    println!("{}", submarine.x);
    println!("{}", submarine.depth);
    println!("{}", submarine.x*submarine.depth);
}

/*
*  Representing the location of a submarine that can move forward, up, and down
*  and the aim of the submarine in the water 
*/
struct SubmarineLocator {
    x : i32,
    depth : i32,
    aim : i32
}

/*
* Getting the cordinates for a submarine per an input file with movement commands and values
* separated by a space
* Rules for submarine cords:
* down X increases your aim by X units.
* up X decreases your aim by X units.
* forward X does two things:
*   It increases your horizontal position by X units.
*   It increases your depth by your aim multiplied by X.
*/
fn find_cords(filename:String, mut submarine:SubmarineLocator) -> SubmarineLocator {
    let file = File::open(filename).ok().unwrap();
    let lines = BufReader::new(file).lines();

    for line in lines {
        let goodline = line.unwrap();
        if !goodline.is_empty() {
            let v: Vec<&str> = goodline.split(' ').collect();
            let direction = v[0];
            let changed_cord = v[1].parse::<i32>().ok().unwrap();
            match direction {
                "forward" => {
                    submarine.x += changed_cord;
                    submarine.depth += (submarine.aim * changed_cord);
                },
                "down" => {
                    submarine.aim += changed_cord;
                },
                "up" => {
                    submarine.aim -= changed_cord;
                },
                _=>(),
            }
        }
    }
    return submarine;
}