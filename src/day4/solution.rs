use std::fs;


pub fn part1()
{
    let file = fs::read_to_string("src/day4/data.txt")
        .expect("Should have been able to read the file");

    print!("{}", file);
    println!();
    
    let split = file.split('\n');

    let mut data: Vec<Vec<char>> = vec![];
    
    for line in split{
        data.push(line.trim().chars().collect());
    }

    // Part 1
    let mut num_papers = remove_paper(&data);
    let mut num_removals = num_papers.len();

    println!("Final num: {}", num_removals);

    // Part 2
    while num_papers.len() > 0
    {
        update_data(&mut data, num_papers);
        num_papers = remove_paper(&data);
        num_removals +=num_papers.len();
    }
    println!("Final num 2: {}", num_removals);

    print_paper_stack(&data);
}

fn print_paper_stack(data: &Vec<Vec<char>>)
{
    for line in data.iter()
    {
        for x in line.iter()
        {
            print!("{}", x);
        }
        println!();
    }
}

fn remove_paper(data: &Vec<Vec<char>>) -> Vec<(usize, usize)>
{
    let mut accessible_papers: Vec<(usize, usize)> = vec![];

    for (row, line) in data.iter().enumerate()
    {
        for (column, x) in line.iter().enumerate()
        {
            if *x == '@'
            {
                let row_lims = get_max_min(row, data.len() - 1);
                let col_lims: (usize, usize) = get_max_min(column, line.len() - 1);
    
                let mut sub_matrix: Vec<Vec<char>> = vec![]; 
                
                for i in row_lims.0..row_lims.1+1{
                    sub_matrix.push(data[i][col_lims.0..col_lims.1+1].to_vec());
                }

                if check_surround(sub_matrix)
                {
                    accessible_papers.push((row, column));
                }
            }
        }
    }

    return accessible_papers;
}


fn update_data(data: &mut Vec<Vec<char>>, removed: Vec<(usize, usize)>)
{
    for paper in removed{
        data[paper.0][paper.1] = '.';
    }
}


fn get_max_min(index: usize, max_val: usize) -> (usize, usize)
{
    let mut max = max_val;
    let mut min = 0;

    if index != 0{
        min = index - 1;
    }

    if index !=max_val{
        max = index + 1;
    }

    return(min,max);
}

// Pass a slice of 3x3 section
// Check row - 1, col - 1, col, col + 1
// Check row, col - 1, col +1
// Check row + 1, col - 1, col, col + 1
fn check_surround(sub: Vec<Vec<char>>) -> bool
{
    let mut paper_count = 0;
    for  line in sub
    {
        for  x in line{
            if x == '@'{
                paper_count+=1;
            }
        }
    }
    if paper_count < 5{
        return true;
    }
    return false;
}
