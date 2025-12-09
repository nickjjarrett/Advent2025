use std::{fs};


// 1. Create coordinate tuples (162,817,812)

// 2. Calculate distances sqrt((a1-b1)^2+(a2-b2)^2+(a3-b3)^2) 
// 3. Store distances and pair indexes Vec[(dist, (a,b))]
// 4. Sort by distance
// 5. Build circuits from points of closest points (1,3,12),(9,10)
//   a. pull coordinate pairs
//   b. check for existing loops with current points (could be multiple)
//   c. add points to new or existing loops and combine (vec.extend??)
// 6. Sort circuits by length


pub fn part1(path: &str)
{
    let file = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    let split = file.split('\n').collect::<Vec<_>>();
    
    // let mut test_vec = vec![(3,(4,0)), (4,(2,1)), (1,(2,3))];
    // println!("{:?}", test_vec);
    // test_vec.sort();
    // println!("{:?}", test_vec);


    let mut data: Vec<(i64, i64, i64)> = vec![];
    // println!("{:?}", split[0].trim());

    for i in split{
        let coords = i.trim().split(',').collect::<Vec<&str>>();
        let mut tuple_coord = (0,0,0);
        tuple_coord.0 = coords[0].parse::<i64>().expect("Nums");
        tuple_coord.1 = coords[1].parse::<i64>().expect("Nums");
        tuple_coord.2 = coords[2].parse::<i64>().expect("Nums");
        data.push(tuple_coord);
    }
    
    println!("{:?}", data);

    let mut distances: Vec<(i64, (usize, usize))> = vec![];

    for (i, coord_a) in data.iter().enumerate()
    {
        for j in i..data.len()
        {    
            // println!("{:?}",calc_distances(*coord_a, i, data[j], j));
            distances.push(calc_distances(*coord_a, i, data[j], j));
        }
    }

    distances.sort();
    println!("{:?}", distances);

}


fn calc_distances(a_coord: (i64, i64, i64), a_index: usize, b_coord: (i64, i64, i64), b_index: usize) -> (i64, (usize, usize))
{
    // println!("A: {:?}",a_coord);
    // println!("B: {:?}",b_coord);

    let a = a_coord.0 - b_coord.0;
    let dist_1 = a*a;
    let dist_2 = (a_coord.1 - b_coord.1)*(a_coord.1 - b_coord.1);
    let dist_3 = (a_coord.2 - b_coord.2)*(a_coord.2 - b_coord.2);

    let calc = (dist_1 + dist_2 + dist_3).isqrt();

    (calc, (a_index,b_index))
}
