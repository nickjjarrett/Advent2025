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

    let mut total: usize = 0;

    for row in data.clone()     // Yuck
    {
        total += check_below(&mut beams, row);
        // println!("Beams:");
        // for i in &beams
        // {
            // print!("{} ",i);
        // }
        // println!();
    }

    println!("Total: {}", total);



    let mut multi = part2(&data, start_index);

    println!("multi: {}", multi);

    // it is not (total - 1)*2

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
            // *multi_val += 1;
            new_beams.push(i + 1);
            new_beams.push(i - 1);
            old_beams.push(i);
            // Check beams for this index
            // println!("{}", multi_val);
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




fn part2(data: &Vec<Vec<char>>, start_data: usize) -> i64
{
    let mut timebeams = vec![0; data.len()];

    timebeams[start_data] += 1;

    for line in data
    {
        for (i, thing) in line.iter().enumerate()
        {
            if *thing == '^'
            {
                timebeams[i-1] += timebeams[i];
                timebeams[i+1] += timebeams[i];

                timebeams[i] -= timebeams[i];
            }
        }
    }

    let mut total = 0;
    for beam in timebeams{
        total += beam;
    }

    return total;
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
