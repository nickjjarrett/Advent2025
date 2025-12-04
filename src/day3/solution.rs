use core::num;
use std::{fs, io::empty};

pub fn part1()
{
    println!("Day 3 Part 1");

    let data_path = "src/day3/testdata.txt";
    // let data_path = "src/day3/data.txt";
    let file = fs::read_to_string(data_path)
        .expect("Should have been able to read the file");

    let final_len = 2;
    let split = file.split('\n');
    for line in split{
        println!("{}", line);
        let vals = line.splitn(line.len()+1, "");   // This populates first index as empty
        
        let mut val_a = 0;
        let mut index= 0;
        // let mut maxes: Vec<(i8, usize)> = vec![];

        // find the maximum value
        for val in vals
        { 
            if !val.is_empty()
            {
                index+=1;
                let number = val.parse::<i8>().expect("Number plz");
                if number > val_a && line.len() - index >= final_len -1 
                {
                    val_a = number;
                }
            }
        }
        println!("Val a: {}", val_a);

        // find each index of max value instances
        
        // for max_val in maxes
        // {
        //     println!("val: {} idx:{}", max_val.0, max_val.1);
        // }




    }

    // Want to find the pair of numbers that makes largest value
    // Iterate through the line, keep track of max
    // if val == 9 look for the next largest value
    // want to check all instances of max
    // eg 11194111981
    // only finding first instance will miss 98



}