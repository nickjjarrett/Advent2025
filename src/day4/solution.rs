use std::fs;

pub fn part1()
{
    let file = fs::read_to_string("src/day4/testdata.txt")
        .expect("Should have been able to read the file");

    // print!("{}", file);
    
    let split = file.split('\n');

    let mut data: Vec<Vec<char>> = vec![];
    
    for line in split{
        
        data.push(line.chars().collect());
    }

    

    for (row, line) in data.iter().enumerate()
    {
        // line[4] = 'X';
        for (column, x) in line.iter().enumerate(){
            print!("{}", x);
            check_surround(row, column);
        }
        println!();
    }

    

    // print!("Final num: {}", code);
}

// Check row - 1, col - 1, col, col + 1
// Check row, col - 1, col +1
// Check row + 1, col - 1, col, col + 1
fn check_surround(row: usize, column: usize) -> i32
{

    return 0;
}

