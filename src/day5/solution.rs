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
    // No brute forcing
    let mut consolidated = consolidate_ranges(ranges);
    
    let mut prev = 1000;
    // consolidated = consolidate_ranges(consolidated);
    // consolidated = consolidate_ranges(consolidated);

    loop
    {
        consolidated = consolidate_ranges(consolidated);
        if prev - consolidated.len() == 0
        {
            break;
        }
        prev = consolidated.len();
    }

    
    let mut total = 0;
    
    for i in consolidated
    {
        total += i.1 - i.0 + 1;
    }
    
    println!("Total {}",total);

    // 403440257222619 is too high 
    // 350939902751909

}

fn parse_ranges(range: &str) -> (usize, usize)
{
    let vals: Vec<&str> = range.split('-').collect();
    
    return (vals[0].trim().parse::<usize>().expect("asd"), vals[1].trim().parse::<usize>().expect("asd"));
}


fn consolidate_ranges(ranges: Vec<(usize, usize)>) -> Vec<(usize, usize)>
{
    let mut consolidated: Vec<(usize, usize)> = vec![ranges[0]];

    for range in &ranges[1..]
    {
        let mut found = false;
        // Check in the consolidated ranges where this one fits in
        for con_range in &mut consolidated
        {
            // Range is fully encapsulated
            if range.0 >= con_range.0 && range.1 <= con_range.1
            {
                found = true;
                break;
            }
            // Range overlaps above
            else if range.0 >= con_range.0 && range.0 <= con_range.1 && range.1 > con_range.1
            {
                found = true;
                con_range.1 = range.1;
                break;
            }
            // Range overlaps below
            else if range.0 < con_range.0 && range.1 <= con_range.1 && range.1 >= con_range.0
            {
                found = true;
                con_range.0 = range.0;
                break;
            }
            // Range fully encapsulates
            else if range.0 <= con_range.0 && range.1 >= con_range.1
            {
                found = true;
                con_range.1 = range.1;
                con_range.0 = range.0;
                break;
            }
            // Range doesn't overlap, check the next one
        }
        if !found
        {
            // Need new range added
            consolidated.push(*range);
        }
    }

    // consolidated.sort();

    return consolidated;
}