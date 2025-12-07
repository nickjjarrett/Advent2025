use core::num;
use std::{fs, iter::Sum, vec};



pub fn part1(path: &str)
{
    let file = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    let split: std::str::Split<'_, char> = file.split('\n');

    let mut nums_done = false;

    let mut numbers: Vec<usize> = vec![]; 
    let mut operators: Vec<String> = vec![];

    for line in split
    {
        for dat in line.split_ascii_whitespace()
        {
            if dat == "*" || dat == "+"
            {
                operators.push(dat.to_string());
            }
            else {
                
                numbers.push(dat.to_string().parse::<usize>().expect("msg"));

            }
        }    
    }
    let num_per_q = numbers.len()/operators.len();
    let num_qs = numbers.len()/num_per_q;

    let mut good_nums: Vec<Vec<usize>> = vec![];

    for i in 0..num_qs
    {
        let mut q_nums: Vec<usize> = vec![];
        for j in 0..num_per_q
        {
            q_nums.push(numbers[i + num_qs*j]);
        }
        good_nums.push(q_nums);
    }

    let mut total = 0;
    for (i,op) in operators.iter().enumerate()
    {
        total += do_maths(op, &good_nums[i]);
    }
    
    println!("Total: {}", total);


}

fn do_maths(op: &String, nums: &Vec<usize>) -> usize
{
    let mut result = 0;
    if op == "+"
    {
        for num in nums
        {
            result += num;
        }
    }
    else if op == "*"
    {
        result = 1;
        for num in nums
        {
            result *= num;
        }
    }
    return result;
}