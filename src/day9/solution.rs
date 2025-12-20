use std::{fs};


#[derive(Clone)]
struct Rectangle{
    area: i64,
    point_a: (i64, i64),
    point_b: (i64, i64)
}

// impl<T: Clone> Clone for Rectangle<T>{

// }

impl Rectangle{
    fn calc_area(&mut self)
    {
        self.area = ((self.point_a.0 - self.point_b.0).abs() + 1) * ((self.point_a.1 - self.point_b.1).abs() + 1)
    }
}


struct CrossSections{
    range: (u16,u16),
    internal: Vec<(u16, u16)>,
    external: Vec<(u16, u16)>
}

// Horizontal cross sections for test data
// First
// range (0,0)
// internal []
// external [(0,13)]
//
// second
// range (1,2)
// internal [(7,11)]
// external [(0,6), (12,13)]
//
// third
// range (3,5)
// internal [(2,11)]
// external [(0,1), (12,13)]
//
// fourth
// range (6,7)
// internal [(9,11)]
// external [(0,8), (12,13)]
//
// fifth
// range (8,8)
// internal []
// external [(0,13)]




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

    let mut rectangles: Vec<Rectangle> = vec![];

    for (i, point) in data.iter().enumerate()
    {
        for j in i + 1..data.len()
        {   
            let mut rect = Rectangle{
                area: 0,
                point_a: *point,
                point_b: data[j]
            };

            rect.calc_area();
            rectangles.push(rect);
        }
    }

    rectangles.sort_by_key(|r| r.area);

    let largest: Rectangle = rectangles.pop().unwrap();

    println!("Max area: {} using {:?}", largest.area, (largest.point_a, largest.point_b));
    println!("Rectangles: {}", rectangles.len());

    
    // data.sort();
    // println!("{:?}", data);
    
    // let max_x = data[data.len()-1].1 as u16;
    
    // let first_range = CrossSections{
    //     range: (0,(data[0].0 - 1) as u16),
    //     internal: vec![],
    //     external: vec![(0,max_x + 1)]
    // };
    
    // let mut verticals: Vec<CrossSections> = vec![first_range];
    
    // for i in (1..data.len()-1).step_by(2)
    // {
    //     println!("{:?}", data[i]);
    //     let range = CrossSections{
    //         range: (verticals[i/2].range.1 + 1, (data[i+1].0) as u16),
    //         internal: vec![],
    //         external: vec![(0,max_x + 1)]
    //     };
    //     verticals.push(range);
    // }
    
    // for ranges in verticals{
    //     println!("{:?}", ranges.range);
    //     println!("{:?}", ranges.external);

    // }



    let max_x = data[data.len()-1].0 as u16;

    data.sort_by_key(|a| a.1);
    println!("{:?}", data);
    
    let first_range = CrossSections{
        range: (0,(data[0].1 - 1) as u16),
        internal: vec![],
        external: vec![(0,max_x + 1)]
    };

    let mut horizontals: Vec<CrossSections> = vec![first_range];

    // let prev_range = horizontals[0].external.clone();

    let range = CrossSections{
        range: (1,2),
        internal: vec![(7,11)],
        external: vec![(0,6), (12,12)]
    };
    horizontals.push(range);
    
    let range = CrossSections{
        range: (3,5),
        internal: vec![(2,11)],
        external: vec![(0,1), (12,12)]
    };
    horizontals.push(range);
    
    let range = CrossSections{
        range: (6,7),
        internal: vec![(9,11)],
        external: vec![(0,8), (12,12)]
    };
    horizontals.push(range);
    
    let range = CrossSections{
        range: (8,8),
        internal: vec![(9,11)],
        external: vec![(0,8), (12,12)]
    };
    horizontals.push(range);
    





    for i in (1..data.len()).step_by(2)
    {
        // println!("data {:?}", data[i]);
        println!("range {:?}", (data[i-1].0, data[i].0));
        
        // Find out if more or less internals than before
        // Replace the val of range with new one

        // let range = CrossSections{
        //     range: (horizontals[i/2].range.1 + 1, (data[i+1].1 -1) as u16),
        //     internal: vec![],
        //     external: vec![(0,max_x + 1)]
        // };
        // horizontals.push(range);
    }

    for ranges in horizontals{
        println!("{:?}", ranges.range);
        println!("{:?}", ranges.external);

    }

}





// takes a rectangle, and checks if it is internal to the overall shape
// 
// 
// 
fn is_internal(rect: Rectangle)
{
    
}


// Part 2
//
// Tiles that are adjacent in your list will always be on either the same row or the same column.
//
//
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


// .............. 0
// .......#XXX#.. 1
// .......XXXXX.. 2
// ..#XXXX#XXXX.. 3
// ..XXXXXXXXXX.. 4
// ..#XXXXXX#XX.. 5
// .........XXX.. 6
// .........#X#.. 7
// .............. 8
// 0123456789abcd

// can good area be drawn as this?
// ....... 1
// ..#X#.. 2
// .##XX.. 4
// .#X#X.. 6
// ...##.. 8
// ....... 9
// 238acde

// ..............
// .......#...#..
// ..............
// ..#....#......
// ..............
// ..#......#....
// ..............
// .........#.#..
// ..............


// Get all areas
// Sort largest to smallest (will need to know the coords of the rectangles)
// Use ranges to determine internal/external
// e.g. min -> max is external
//  check next column/row
//  when encounter red tile row, ranges are adjusted
//  check that the ranges overlap when hitting the current rectangle
//