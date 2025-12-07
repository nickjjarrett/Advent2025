use std::fs;


pub fn part1(path: &str)
{
    let file = fs::read_to_string(path)
        .expect("Should have been able to read the file");
    let split: std::str::Split<'_, char> = file.split('\n');

}