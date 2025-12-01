use std::fs;
 
fn main() {
    let file = fs::read_to_string("C:/Users/nick-/Documents/Code/Rust/Advent2025/src/data.txt")
        .expect("Should have been able to read the file");
 
    let split = file.split('\n');
    let mut number: i16 = 50;
    let mut code = 0;
    let mut prev = 0;
 
    for line in split{
        let instr = line.split_at(1);
        let clicks = instr.1.trim().parse::<i16>().unwrap() ;
        prev = number;
 
        code += clicks/100;
 
        if instr.0 == "L" {number -= (clicks % 100) }else{number += (clicks%100)}
       
        if (number.is_positive() && prev.is_negative()) || (prev.is_positive() && number.is_negative())
        {
            code+=1;
        }
 
        if (number % 100) != 0 && (prev % 100) != 0
        {
            code += ((number / 100) - (prev / 100)).abs();
        }
        // Need to check for crossover
        // Change of sign indicates a crossover
        // ((number / 100) - (prev / 100)).abs()
 
        // Increment based on 100 multiples (full rotations)
        // Check for sign change
        if number % 100 == 0
        {
            code+=1;
        }
    }
    print!("Final num: {}", code);
}