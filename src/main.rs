use std::fs;
 
fn main() {
    let file = fs::read_to_string("src/data.txt")
        .expect("Should have been able to read the file");

    let split = file.split('\n');
    let mut number: i16 = 50;
    let mut code = 0;
    let mut prev = 0;
 
    for line in split{
        let instr = line.split_at(1);
        let clicks = instr.1.trim().parse::<i16>().unwrap() ;
        prev = number;
        
        // Increment full rotations
        code += clicks/100;
 
        // Increment / decrement value
        if instr.0 == "L" {number -= clicks % 100}else{number += clicks%100}
       
        // +ve -> -ve crossover point e.g. 5 -> -9
        if (number.is_positive() && prev.is_negative()) || (prev.is_positive() && number.is_negative())
        {
            code+=1;
        }
        // Crossover point, e.g. 93 -> 107
        else if (number % 100) != 0 && (prev % 100) != 0
        {
            code += ((number / 100) - (prev / 100)).abs();
        }
        // Final position is zero
        else if number % 100 == 0
        {
            code+=1;
        }
    }
    print!("Final num: {}", code);
}