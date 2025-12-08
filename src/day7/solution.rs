use std::{fs, ops::Index, vec};

pub fn part1(path: &str)
{
    let file = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    let split = file.split('\n').collect::<Vec<_>>();
    
    let mut data = vec![];

    for i in split{
        data.push(i.chars().collect::<Vec<char>>());
    }
    
    let mut start_index = 0;
    for (i, letter) in data[0].iter().enumerate(){
        if *letter == 'S'
        {
            start_index = i;
        }
    }

    println!("{}", file);
    println!("S index: {}", start_index);

    // Need to do it row by row
    // Keep track of the number of beams
    // Then if any of the beams hit a splitter, double and set their index
    // Part 1

    let mut beams: Vec<usize> = vec![start_index];

    let mut total = 0;

    for row in data
    {
        total += check_below(&mut beams, row);
        // println!("Beams:");
        for i in &beams
        {
            // print!("{} ",i);
        }
        // println!();
    }

    println!("Total: {}", total);

    // println!("Silly: {}", silly_test(start_index, 0, &data));


}



fn check_below( beams: &mut Vec<usize>, data: Vec<char>) -> usize
{
    let mut val = 0;
    let mut new_beams: Vec<usize> = vec![];
    let mut old_beams: Vec<usize> = vec![];

    for (i, thing) in data.iter().enumerate()
    {
        if *thing == '^' && beams.contains(&i)
        {
            // let instances = beams.iter().filter(|x| *x == &i).count();
            val += 1;
            new_beams.push(i + 1);
            new_beams.push(i - 1);
            old_beams.push(i);
            // Check beams for this index
            // println!("{}", i);
        }
    }

    // println!("New Beams: ");
    // for i in &new_beams{
    //     print!("{},", i);
    // }
    
    // println!("\nOld Beams: ");
    for old in &old_beams
    {
        // print!("{} ", old);
        beams.retain(|x| *x != *old);
    }
    // println!();

    beams.extend(new_beams);

    // for i in old_beams{
    //     print!("{},", i);
    // }

    // check_below(index, row+1, data);

    return val;
}



fn silly_test(index: usize, row: usize, data: &Vec<Vec<char>>) -> usize
{
    let mut val = 0;
    if row == data.len()
    {
        return 1;
    }

    if data[row][index] == '^'
    {
        val += silly_test(index - 1, row + 1, &data);
        val += silly_test(index + 1, row + 1, &data);
    }
    else 
    {
        val += silly_test(index, row + 1, &data);
        
    }
    return val;
}
