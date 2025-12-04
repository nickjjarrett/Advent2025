use std::fs;

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
        let vals: std::str::SplitN<'_, &str> = line.trim().splitn(line.len()+1, "");   // This populates first index as empty
        // let a = [1, 2, 3];
        let mut max = 0;
        let mut max_index = 0;
        let mut index= 0;
        // let t_vals = vals.clone();
        // let mut maxes: Vec<(i8, usize)> = vec![];

        // find the maximum value
        for val in vals
        { 
            if !val.is_empty()
            {
                index+=1;
                let number = val.parse::<i8>().expect("Number plz");
                if number > max && line.len() - index >= final_len
                {
                    max_index = index;
                    max = number;
                }
            }
        }
        println!("Val a: {} idx: {}", max, max_index);

        let mut max_2 = 0;

        // check for max number after first max
        // for val in vals.into_iter().skip(max_index)
        // {
        //     println!("val: {}", val);
        //     let number = val.parse::<i8>().expect("Number plz");
        //     if number > max_2
        //     {
        //         max_2 = number;
        //     }
        // }
        println!("max2: {}", max_2);



    }

    // Want to find the pair of numbers that makes largest value
    // Iterate through the line, keep track of max
    // if val == 9 look for the next largest value



}