use std::fs;

mod day2;

pub fn day1()
{
    let file = fs::read_to_string("src/data.txt")
        .expect("Should have been able to read the file");

    let split = file.split('\n');
    let mut number: i16 = 50;
    let mut code = 0;
    let mut _prev = 0;
 
    for line in split{
        let instr = line.split_at(1);
        let clicks = instr.1.trim().parse::<i16>().unwrap() ;
        _prev = number;
        
        // Increment full rotations
        code += clicks/100;
 
        // Increment / decrement value
        if instr.0 == "L" {number -= clicks % 100}else{number += clicks%100}
       
        // +ve -> -ve crossover point e.g. 5 -> -9
        if (number.is_positive() && _prev.is_negative()) || (_prev.is_positive() && number.is_negative())
        {
            code+=1;
        }
        // Crossover point, e.g. 93 -> 107
        else if (number % 100) != 0 && (_prev % 100) != 0
        {
            code += ((number / 100) - (_prev / 100)).abs();
        }
        // Final position is zero
        else if number % 100 == 0
        {
            code+=1;
        }
    }
    print!("Final num: {}", code);
}

 
fn main() {
    day2::part1();
}