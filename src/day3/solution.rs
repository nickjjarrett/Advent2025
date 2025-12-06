use std::fs;


fn find_max(arr: Vec<char>, limit: usize) -> usize
{
    let mut max: char = '0';
    let mut max_index  = 0;
    let mut index = 0;
    let len: usize = arr.len();

    for val in arr
    { 
        // print!("{} ", val);
        if val.is_ascii_digit()
        {
            let number = val;
            if number > max && len  - index >= limit
            {
                max_index = index;
                max = number;
            }
        }
        index+=1;
    }
    // println!("Max: {} index: {}", max, max_index);
    return max_index;
}


pub fn part1()
{
    println!("Day 3 Part 1");

    // let data_path = "src/day3/testdata.txt";
    let data_path = "src/day3/data.txt";
    let file = fs::read_to_string(data_path)
        .expect("Should have been able to read the file");

    let mut total = 0;
    
    let split = file.split('\n');
    for line in split{
        let line_2 = line.trim();
        println!("{}", line);
        if line.len() < 2
        {
            continue;
        }

        let nums: Vec<char> = line_2.chars().collect();
        let mut indeces: Vec<usize> = vec![];
        let lim: usize = 12;

        indeces.push(find_max(nums.clone(), lim));

        for i in (1..lim).rev()
        {
            // println!("i: {} index {} ", i, indeces[lim - i - 1]);
            // println!("index {} ", indeces[lim - i]);
            indeces.push(find_max(nums.clone()[(indeces[lim - i-1] + 1)..].to_vec(), i) + indeces[lim - i-1] + 1);
        }
        
        let mut ans: String = "".to_string();
        for num in indeces{
            ans.push(nums[num]);
        }
        println!("ans: {}", ans);
        total += ans.parse::<i64>().unwrap();
    }
    println!("Total: {} ", total);
}