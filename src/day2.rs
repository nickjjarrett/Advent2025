use std::fs;

pub fn part1() {
    let file = fs::read_to_string("src/data2.txt")
        .expect("Should have been able to read the file");

    let mut data: Vec<&str> = file.split(',').collect();
    let mut case_a: bool = false;
    let mut case_b: bool = false;

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
            if vals[0].len() %2 == 0
            {
                nice_data.push((new_a, new_b));
            }
            else if vals[1].len() %2 == 0
            {
                nice_data.push((new_c, new_d));
            }
        }  
        else {

            if vals[0].len() % 2 == 0{
                nice_data.push((vals[0].to_string(), vals[1].to_string()));
            }
        }
    }

    let mut total: i64 = 0;

    for pairs in nice_data{

        let mut num_ = 0;
        let start = pairs.0.split_at(pairs.0.len()/2);
        let end = pairs.1.split_at(pairs.1.len()/2);

        let start_a = start.0.parse::<i64>().unwrap();
        let start_b = start.1.parse::<i64>().unwrap();
        let end_a = end.0.parse::<i64>().unwrap();
        let end_b = end.1.parse::<i64>().unwrap();
        let mut first = 0;

        if start_a >= start_b

        {
            // first instance = start_a start_a e.g. 21 gives 22
            first = start_a;
            num_ += 1;
        }
        else {
            // first instance  = start_a + 1  start_a + 1 e.g. 23 gives 33
            first = start_a as i64 + 1;
        }
        if end_a <= end_b
        {
            num_ += 1;
        }
        num_ += end_a - start_a - 1;

        // print!("{} {} - ", start_a, start_b);
        // println!("{} {}", end_a, end_b);
        // println!("Num invalid IDs: {}", num_);

        // let base: i64 = 10;

        let exp =  pairs.0.len() as u32;
        // let min: i64 = 10i64.pow(exp-1) + 10i64.pow((exp/2) - 1);
        // let increment: i64 = base.pow(exp/2) + 1;
        let increment: i64 = 10i64.pow(exp/2) + 1;
        // println!("min {}  increment {} first {}", min, increment, first);
        // println!("sum {}", summate(min, first, increment, num_));

        total += summate(first, increment, num_);

    }

    println!("Total: {}", total);

}


// /// Split the numbers into x parts
// /// 
// fn splitNums(div: i8)
// {
//     for pairs in nice_data{

//         let mut num_ = 0;
//         let start = pairs.0.split_at(pairs.0.len()/2);
//         let end = pairs.1.split_at(pairs.1.len()/2);

//         let start_a = start.0.parse::<i64>().unwrap();
//         let start_b = start.1.parse::<i64>().unwrap();
//         let end_a = end.0.parse::<i64>().unwrap();
//         let end_b = end.1.parse::<i64>().unwrap();
//         let mut first = 0;

//         if start_a >= start_b
//         {
//             // first instance = start_a start_a e.g. 21 gives 22
//             first = start_a;
//             num_ += 1;
//         }
//         else {
//             // first instance  = start_a + 1  start_a + 1 e.g. 23 gives 33
//             first = start_a as i64 + 1;
//         }
//         if end_a <= end_b
//         {
//             num_ += 1;
//         }
//         num_ += end_a - start_a - 1;

//         // print!("{} {} - ", start_a, start_b);
//         // println!("{} {}", end_a, end_b);
//         // println!("Num invalid IDs: {}", num_);
//     }
// }


fn summate(first: i64, increment: i64, num: i64) -> i64
{
    let mut val: i64 = 0;
    for i in 0..num
    {
        val += increment + increment * (first - 1 + i);
        // println!("{}", increment + increment * (first - 1 + i))

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