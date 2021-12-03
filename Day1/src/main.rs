use std::fs::File;
use std::io::{BufReader, BufRead};
use std::ptr::null;
use queue::Queue;

fn main() {
    let filename = String::from("input.txt");
    println!("{}", findIncreases(filename));
    let filename2 = String::from("input.txt"); //b/c rust was being stupid and didn't let me use filename
                                                //but filename == filename2
    println!("{}", findIncreasesByGroup(filename2));
}

/*
* Compare two numbers and increment if second is bigger than the first
* returns number of increases 
*/
fn findIncreases(filename:String) -> i32 {
    let file = File::open(filename).ok().unwrap();
    let lines = BufReader::new(file).lines();
    let mut numberIncreases = 0;
        let mut prevNum :i32 = -1;
        for line in lines {
            let goodline = line.unwrap();
            if !goodline.is_empty() {
                let currentNum:i32 = goodline.parse().unwrap(); 
                if currentNum > prevNum {
                    numberIncreases+=1;
                }
                    prevNum = currentNum;
                
            }
        }
    
    return numberIncreases-1; //first number compared to previous does NOT count
}

/*
* operating under assumption that lines.length % 3 == 0, I think, though subtracting
* one from final return value to adjust for initial operation?
*/
fn findIncreasesByGroup(filename:String) -> i32 {
    let file = File::open(filename).ok().unwrap();
    let lines = BufReader::new(file).lines();
    let mut numberIncreases = 0;
    let mut first_total :i32 = -1;
    let mut second_total:i32 = -1;
    let mut q = Queue::new();
    q.set_capacity(3); // comparing sums in groups of 3
    let mut index = 0;
        for line in lines {
            let goodline = line.unwrap();
            if !goodline.is_empty() {
                let currentNum:i32 = goodline.parse().unwrap(); 
                //println!("{}", currentNum);
                if index == 3 {
                    first_total = sumQueue(q.clone());
                    q.dequeue();
                    index-=1;
                }
                q.queue(currentNum); // just adding if not @ 2 or 3, no calcs
                index+=1;
                if index == 3{
                second_total = sumQueue(q.clone());
                if second_total > first_total {
                    numberIncreases+=1;
                }
                }
            }
        }
    
    return numberIncreases-1; //first number compared to previous does NOT count
}

/*
* sum elements in a queue and return that sum
*/
fn sumQueue(queue:Queue<i32>) -> i32 {
    let mut sum = 0;
    let mut queue2 = queue;
    while !queue2.is_empty() {
        sum += queue2.dequeue().unwrap();
    }
    return sum;
}