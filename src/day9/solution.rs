use std::{fs};

pub fn part1(path: &str)
{
    let file = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    let split = file.split('\n').collect::<Vec<_>>();
    
    let mut data: Vec<(i64, i64)> = vec![];

    for i in split{
        let coords = i.trim().split(',').collect::<Vec<&str>>();
        let mut tuple_coord = (0,0);
        tuple_coord.0 = coords[0].parse::<i64>().expect("Nums");
        tuple_coord.1 = coords[1].parse::<i64>().expect("Nums");
        data.push(tuple_coord);
    }
    
    // println!("{:?}", data);

    let mut max_area = 0;
    let mut max_points = (0,0);

    for (i, point) in data.iter().enumerate()
    {
        for j in i + 1..data.len()
        {    
            let area = calc_area(*point, data[j]);
            if area > max_area
            {
                max_area = area;
                max_points.0 = i;
                max_points.1 = j;
            }
        }
    }

    println!("Max area: {} using {:?}", max_area, max_points);

}


fn calc_area(a_coord: (i64, i64), b_coord: (i64, i64)) -> i64
{
    ((a_coord.0 - b_coord.0).abs() + 1) * ((a_coord.1 - b_coord.1).abs() + 1)
}


// Part 2
//
// Tiles that are adjacent in your list will always be on either the same row or the same column.
//
//
// .............. 1
// .......#XXX#.. 2 
// .......XXXXX.. 3
// ..#XXXX#XXXX.. 4
// ..XXXXXXXXXX.. 5
// ..#XXXXXX#XX.. 6
// .........XXX.. 7
// .........#X#.. 8
// .............. 9
// 123456789abcde
//
// Find the largest internal rectangle
// the rectangle can't have and red squares internally
// the rectangle can have red squares on the perimeter
// an example of bad perimeter squares is (3,6) to (c,8)
// 
// When hitting a red square on the perimeter
//   if the connecting square takes the direction within/across rectangle, then it's bad
//   e.g. (3,6) to (c,8) at (a,8)
//   if the connecting square takes direction away from rectangle, then it's okay 
//   e.g. (3,4) to (a,6) at (8,4)

 
// 
