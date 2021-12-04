use std::fs::File;
use std::io::{BufReader, BufRead};
use std::vec::Vec;

static mut bit_sums: [(u32, u32); 12] = [(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0)]; //a list of tuples,
    //where each tuple represents one bit in a 5-bit binary number
    //and each tuple represents (#0's, #1's)
    //for calculating gamma and epsilon values

fn main() {
    let filename = "input.txt";
    parse_binary_file(&filename);
    let mut gamma="".to_string();
    let mut epsilon="".to_string();
    let mut decimal_gamma:usize;
    let mut decimal_epsilon:usize;
    
    unsafe {
        //println!("{:?}", bit_sums);
         gamma = get_gamma_rate(); 
         epsilon = get_epsilon_rate();
         decimal_gamma = usize::from_str_radix(&gamma,  2).unwrap();
         decimal_epsilon = usize::from_str_radix(&epsilon,  2).unwrap();
    }
    let power_consumption = decimal_gamma * decimal_epsilon;
    // println!("{}", power_consumption);
    let mut line_bytes:Vec<String> = get_binary_file_data(&filename);
    let oxygen = oxygen_generator_rating(line_bytes);
    let oxygen_rating = usize::from_str_radix(&oxygen,  2).unwrap();
    println!("{}", oxygen_rating);

    let line_bytes_2 = get_binary_file_data(&filename); //resets data to run co2
    let co2 = co2_scrubber_rating(line_bytes_2);
    let co2_rating = usize::from_str_radix(&co2,  2).unwrap();
    println!("{}", co2_rating);
    let life_support = oxygen_rating*co2_rating;
    println!("life support: {}", life_support);
}

fn parse_binary_file(filename:&str) {
    let file = File::open(filename).ok().unwrap();
    let lines = BufReader::new(file).lines();
    for line in lines {
        let goodline = line.unwrap();
        if !goodline.is_empty() {
            unsafe{adding_bits(goodline);}
        }
    }
}

fn get_binary_file_data(filename:&str) -> Vec<String>{
    let file = File::open(filename).ok().unwrap();
    let lines = BufReader::new(file).lines();
    let mut line_bytes = Vec::new();
    for line in lines {
        let goodline = line.unwrap();
        if !goodline.is_empty() {
            line_bytes.push(String::from(goodline.as_str()));
        }
    }
    return line_bytes;
}

unsafe fn adding_bits(byteline:String) {
    let mut sum_index = 0;
    for d in byteline.chars() {
        match d.to_digit(10).unwrap() {
            0 => {
                bit_sums[sum_index].0+=1;
            },
            1 => {
                bit_sums[sum_index].1+=1;
            },
            _ => {} //error checking b/c d could be not digit
        }
        sum_index+=1;
    }
}

/*
* returns a byte built from the most common bits, that is
* compares the number of 0's to 1's in each digit spot, and
* chooses the greater count as the digit for the resultant byte
*/
unsafe fn get_gamma_rate() -> String {
    let mut gamma_rate:String = "".to_string();
    for tuple in bit_sums.iter() {
        if tuple.0 > tuple.1 {
            gamma_rate = gamma_rate + &"0".to_string();
        }
        else {
            gamma_rate = gamma_rate + &"1".to_string();
        }
    }
    return gamma_rate;
}

/*
* returns a byte built from the least common bits, that is
* compares the number of 0's to 1's in each digit spit, and
* chooses the lesser count as the digit for the resultant byte
*/
unsafe fn get_epsilon_rate() -> String {
    let mut epsilon_rate:String = "".to_string();
    for tuple in bit_sums.iter() {
        if tuple.0 < tuple.1 {
            epsilon_rate = epsilon_rate + &"0".to_string();
        }
        else {
            epsilon_rate = epsilon_rate + &"1".to_string();
        }
    }
    return epsilon_rate;
}

fn oxygen_generator_rating(mut list_bytes:Vec<String>) -> String {
    let mut index:usize = 0;
    while &list_bytes.len() > &1 {
        let counts = find_most_common_bit(&list_bytes, index);
        let mut list_index:usize = 0;
        let mut removed_indices:Vec<usize> = Vec::new();
        for b in list_bytes.iter() {
            if counts.0 == counts.1 {
                if b.as_str().chars().nth(index).unwrap()== '0' {
                    removed_indices.push(list_index);
                }
            } else if counts.0 > counts.1 {
                if b.as_str().chars().nth(index).unwrap() == '1' {
                    removed_indices.push(list_index);
                }
            } else { //counts.0 < counts.1
                if b.as_str().chars().nth(index).unwrap() =='0' {
                    removed_indices.push(list_index);
                }
            }
            list_index+=1;
        }
        while removed_indices.len() > 0 {
            if &list_bytes.len() > &1 {
                list_bytes.remove(removed_indices.pop().unwrap());
            } else {
                break;
            }
        }
        // println!("{:?}", list_bytes);
        index+=1;
        println!("{:?}", counts);
    }
    return list_bytes.pop().unwrap();
}

fn co2_scrubber_rating(mut list_bytes:Vec<String>) -> String {
    let mut index:usize = 0;
    while &list_bytes.len() > &1 {
        let counts = find_most_common_bit(&list_bytes, index);
        let mut list_index:usize = 0;
        let mut removed_indices:Vec<usize> = Vec::new();
        for b in list_bytes.iter() {
            if counts.0 == counts.1 {
                if b.as_str().chars().nth(index).unwrap() == '1' {
                    removed_indices.push(list_index);
                }
            } else if counts.0 > counts.1 {
                if b.as_str().chars().nth(index).unwrap() == '0' {
                    removed_indices.push(list_index);
                }
            } else { //counts.0 < counts.1
                if b.as_str().chars().nth(index).unwrap() == '1' {
                    removed_indices.push(list_index);
                }
            }
            list_index+=1;
        }
        while removed_indices.len() > 0 {
            if &list_bytes.len() > &1 {
                list_bytes.remove(removed_indices.pop().unwrap());
            } else {
                break;
            }
        }
        println!("{:?}", counts);
        index+=1;
    }
    return list_bytes.pop().unwrap();
}

fn find_most_common_bit(list_bytes:&Vec<String>, index:usize) -> (u32, u32) {
    let mut counts:(u32, u32) = (0,0);
    for b in list_bytes.iter() {
        match b.as_str().chars().nth(index).unwrap().to_digit(10).unwrap() {
            0 => counts.0+=1,
            1 => counts.1+=1,
            _ =>{}
        }
    }
    return counts;
}