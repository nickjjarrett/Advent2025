
use std::fs;
use std::collections::HashSet;

pub fn part1() {
    // let data_path = "src/testdata2.txt";
    let data_path = "src/data2.txt";
    let file = fs::read_to_string(data_path)
        .expect("Should have been able to read the file");

    let data: Vec<&str> = file.split(',').collect();

    // split the ranges that straddle different orders of magnitude
    // 95 - 115
    // becomes
    // 95 - 99
    // 100 - 115
    // Any range with odd orders of magnitude can be ignored
    let mut nice_data = Vec::new();

    for range in data{
        let vals: Vec<&str> = range.split('-').collect();
        // println!("lens {} {}", vals[0].len(), vals[1].len());
        if vals[0].len() != vals[1].len()
        {
            let new_a = vals[0].to_string();
            let new_b: String = "9".repeat(vals[0].len());
            let mut new_c: String = "0".repeat(vals[1].len());
            new_c = new_c.replacen("0", "1", 1);
            let new_d = vals[1].to_string();
            // println!("{} {}", new_a, new_b);
            // println!("{} {}", new_c, new_d);
            nice_data.push((new_a, new_b));
            nice_data.push((new_c, new_d));
        }  
        else {
            nice_data.push((vals[0].to_string(), vals[1].to_string()));
        }
    }

    let mut total: i64 = 0;

    // let bad_ids: &mut  Vec<i64> =  &mut vec![];
    let bad_ids: &mut  HashSet<i64> = &mut Default::default();

    for pairs in nice_data{
        println!("\n{}-{}", pairs.0, pairs.1);

        let exp =  pairs.0.len() as u32;
        // let increment: i64 = 10i64.pow(exp/2) + 1;
        let first: &mut i64 = &mut 0;



        for i in 2..pairs.0.len()+1
        {
            *first = 0;
            if pairs.0.len() % i == 0
            {
                // println!("\nDivisor: {i}");
                let str_len = pairs.0.len() / i;
                let mut min_val: String = "0".repeat(str_len);
                min_val = min_val.replacen("0", "1", 1);
                min_val = min_val.repeat(i);
                let mut increment = min_val.parse::<i64>().expect("val");

                // println!("min {}", increment);
                increment = increment / (10i64.pow(str_len as u32) / 10);
                // println!("increment {}", increment);

                let num = split_nums(i, first, pairs.clone());
                println!("num: {num}");
                total += summate(*first, increment, num, bad_ids);
            }
        }        
        // println!(" increment {} first {}", increment, first);

    }

    // println!("Total: {}", total);
    for id in bad_ids.iter()
    {
        total += id;
    }
    println!("Total: {}", total);
    // 23589704719 is too low
    // 23588159155
    // 23589704719
    // 24843528659
    // 44143124633
}


fn split_into(number: String, sections: usize) -> Vec<String>
{
    let mut new_string: Vec<String> = vec![];

    let mut temp: String = number.clone();

    // Split into sections, section length = len string / sections
    let section_len = number.len()/sections;
    // println!("section Len {section_len}");

    for _ in 0..sections - 1
    {
        let blah = temp.split_at(section_len);
        new_string.push(blah.0.to_string());
        
        temp = blah.1.to_string();
    }
    new_string.push(temp.to_string());
    
    return new_string;
}



/// Split the numbers into x parts
/// 
fn split_nums(div: usize, first: &mut i64, pairs: (String, String)) -> i64
{
    let mut num_ = 0;
    // for pairs in nice_data{
    let splits_1: Vec<String> = split_into(pairs.0.clone(), div);
    
    // for i in splits_1{
    //     print!("{i} ");
    // }
    // println!("");
    
    let splits_2 = split_into(pairs.1.clone(), div);
    // for i in splits_2{
    //     print!("{i} ");
    // }
    println!("");

    let start = splits_1[0].parse::<i64>().unwrap();
    let end = splits_2[0].parse::<i64>().unwrap();

    *first = start as i64;
    num_ = 2;
    for i in 1..splits_1.len()
    {
        println!("start i {}", splits_1[i]);
        if start > splits_1[i].parse::<i64>().unwrap()
        {
            // *first = start as i64;
            break;
        }
        else if start < splits_1[i].parse::<i64>().unwrap()
        {
            *first = start as i64 + 1;
            num_ -= 1;
            break;
        }
    }

    for i in 1..splits_2.len()
    {
        println!("end i {}", splits_2[i]);
        if end < splits_2[i].parse::<i64>().unwrap()
        {
            break;
        }
        else if end > splits_2[i].parse::<i64>().unwrap() 
        {
            num_ -= 1;
            break;    
        }
    }

    num_ += end - start - 1;
    if num_ < 0{
        num_ = 0;
    }




    // let start = splits_1[0].parse::<i64>().unwrap();
    // let end = splits_2[0].parse::<i64>().unwrap();
    // let start_a = splits_1[0].parse::<i64>().unwrap();
    // let start_b = splits_1[1].parse::<i64>().unwrap();
    // let end_a = splits_2[0].parse::<i64>().unwrap();
    // let end_b = splits_2[1].parse::<i64>().unwrap();

    // print!("{} {} - ", start_a, start_b);
    // println!("{} {}", end_a, end_b);
    // // let mut first = 0;
    
    // if start_a >= start_b{
    //     // first instance = start_a start_a e.g. 21 gives 22
    //     *first = start_a;
    //     num_ += 1;
    // }
    // else {
    //     // first instance  = start_a + 1  start_a + 1 e.g. 23 gives 33
    //     *first = start_a as i64 + 1;
    // }

    // if end_a <= end_b{
    //     num_ += 1;
    // }

    // num_ += end_a - start_a - 1;


    return num_;
}


fn summate(first: i64, increment: i64, num: i64, bad_ids: &mut HashSet<i64>) -> i64
{
    let mut val: i64 = 0;
    for i in 0..num
    {
        bad_ids.insert(increment + increment * (first - 1 + i));
        // val += increment + increment * (first - 1 + i);
        println!("{}", increment + increment * (first - 1 + i));
    }
    // println!("Val: {}", val);
    return val;
}

/*

part 2
Need to divide the numbers up more than once
e.g. 123456 -> 1 2 3 4 5 6 AND 12 34 56 AND 123 456
Split into constituent parts to see if fully repeated number is included
If it is included, need to know so we can negate it after the fact
As the other methods will pick that up as an invalid ID as well

increments will be (for above case)
- 111111
- 10101
- 1001
use a set to bring together all invalid IDs, so duplicates won't matter

*/