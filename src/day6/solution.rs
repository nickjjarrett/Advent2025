use std::{fs, vec};


pub fn part1(path: &str)
{
    let file = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    let split = file.split('\n').collect::<Vec<_>>();
    // let split = file.split('\n').collect();

    let mut data = vec![];

    for i in split
    {
        data.push(i.chars().collect::<Vec<char>>());
    }

    let mut index = 0;
    let mut indexes: Vec<usize> = vec![index];
    
    while index < data[0].len()
    {
        while !check_space_below(index, 0, &data)
        {
            index += 1;
        }
        index += 1;
        indexes.push(index);

        // index += 1;
        // println!("Number index {}", index);
    }
    // Don't need the last one
    // indexes.pop();
    
    let mut total = 0;
    for i in 1..indexes.len(){
        // Build numbers using indexes
        let mut matrix: Vec<Vec<char>> = vec![];

        // println!("i: {}",i);
        
        for j in 0..data.len()
        {
            // println!("j: {}", j);
            if i != indexes.len() - 1
            {
                matrix.push(data[j][indexes[i-1]..indexes[i]].to_vec());
            }
            else {
                
                matrix.push(data[j][indexes[i-1]..].to_vec());
            }

        }

        total += build_numbers(matrix, true);
        // break;

        // println!("operator: {}", data[data.len() - 1][i]);
    }

    println!("Total: {}", total);

    // let mut operators = split[0].split_whitespace();
    
    return;

    // let mut nums_done = false;

    // let mut numbers: Vec<usize> = vec![]; 

    // for line in split
    // {
    //     for dat in line.split_ascii_whitespace()
    //     {
    //         if dat == "*" || dat == "+"
    //         {
    //             operators.push(dat.to_string());
    //         }
    //         else {
                
    //             numbers.push(dat.to_string().parse::<usize>().expect("msg"));

    //         }
    //     }    
    // }
    // let num_per_q = numbers.len()/operators.len();
    // let num_qs = numbers.len()/num_per_q;

    // let mut good_nums: Vec<Vec<usize>> = vec![];

    // for i in 0..num_qs
    // {
    //     let mut q_nums: Vec<usize> = vec![];
    //     for j in 0..num_per_q
    //     {
    //         q_nums.push(numbers[i + num_qs*j]);
    //     }
    //     good_nums.push(q_nums);
    // }

    // let mut total = 0;
    // for (i,op) in operators.iter().enumerate()
    // {
    //     total += do_maths(op, &good_nums[i]);
    // }
    
    // println!("Total: {}", total);


}


fn build_numbers(mut matrix: Vec<Vec<char>>, slug_maths: bool) -> usize
{
    // let operation = matrix.pop().iter().collect();
    let operation = matrix[matrix.len()-1][0];
    matrix.pop();

    // println!("Op: {}", operation);
    let mut numbers: Vec<usize> = vec![];

    // for (i, line) in matrix.iter().enumerate()
    for col in 0..matrix[0].iter().len() - 1
    {
        let mut das = 0;
        // for (j, num) in line.iter().enumerate()
        for row in 0..matrix.iter().len()
        {
            let asd: char = matrix[row][col];
            // let asd: char = matrix[row][col];
            if asd != ' '
            {
                das = das*10 + asd.to_string().parse::<usize>().expect("msg");
                // print!("{}", matrix[j][i].to_string().trim().parse::<usize>().expect("msg"));
            }

        }
        numbers.push(das);
        // println!("{}",das);
        // println!("");
    }

    return do_maths(&operation.to_string(), &numbers);
    // Normal maths
    // numbers are data
    // matrix[0][0] matrix[0][1] matrix[0][2]

    // Slug maths
    // numbers are data
    // matrix[0][0] matrix[1][0] matrix[2][0]


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


fn check_space_below(index: usize, row: usize, data: &Vec<Vec<char>>) -> bool
{
    if row == data.len() - 1 || index == data[0].len() - 1
    {
        return true;
    }
    else if data[row][index] == ' '
    {
        return check_space_below(index, row + 1, data);
    }
    else 
    {
        return false;
    }
}