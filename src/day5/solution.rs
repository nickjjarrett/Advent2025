use std::fs;


pub fn part1(path: &str)
{
    let file = fs::read_to_string(path)
        .expect("Should have been able to read the file");
    let split: std::str::Split<'_, char> = file.split('\n');

    let mut ranges: Vec<(usize, usize)> = vec![];
    let mut ingredients: Vec<usize> = vec![];

    let mut ranges_done = false;

    for line in split{
        
        if line.len() == 1{
            ranges_done=true;
            continue;
        }
        if ranges_done{
            ingredients.push(line.trim().parse::<usize>().expect("NaN"));
        }
        else {
            ranges.push(parse_ranges(line));
        }

    }

    let mut num = 0;
    for i in ingredients
    {
        for range in &ranges
        {
            if i >= range.0 && i <= range.1
            {
                num+=1;
                break;
            }
        }
    }
    println!("Num: {}", num);

    let mut valid_nums: Vec<usize> = vec![];
    // Part 2
    // Consolidate ranges
    for range in &ranges
    {
        println!("New Range");
        for i in range.0..range.1+1{
            if !valid_nums.contains(&i)
            {
                valid_nums.push(i);
            }
        }
    }
    println!("Num: {}", valid_nums.len());



}

fn parse_ranges(range: &str) -> (usize, usize)
{
    let vals: Vec<&str> = range.split('-').collect();
    
    return (vals[0].trim().parse::<usize>().expect("asd"), vals[1].trim().parse::<usize>().expect("asd"));
}
