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
    
    let mut data: Vec<(i64, i64, i64)> = vec![];

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
        for j in i + 1..data.len()
        {    
            // println!("{:?}",calc_distances(*coord_a, i, data[j], j));
            distances.push(calc_distances(*coord_a, i, data[j], j));
        }
    }

    distances.sort();
    println!("Length: {}", distances.len());
    // for i in &distances[0..1005]
    // {
    //     println!("{}", i.0)
    // }

    let mut circuits: Vec<Vec<usize>> = vec![];

    // let num = 1000;
    // let num= 10;
    // let mut firs_set = distances[0..num].iter().collect::<Vec<_>>();

    let mut firs_set = distances;

    firs_set.reverse();

    println!("{:?}", firs_set);

    while firs_set.len() > 0
    {
        
        let pair = firs_set.pop().unwrap();
        // println!("New point: {:?}", pair);
        let point_1 = pair.1.0;
        let point_2 = pair.1.1;

        let mut new = true;

        let mut num = 0;
        let mut circuit_index: (usize, usize) = (0,0);

        for (index, circuit) in circuits.iter_mut().enumerate()
        // for i in 0..circuits.len()
        {
            if circuit.contains(&point_1) && circuit.contains(&point_2)
            {
                new = false;
                break;
            }
            else if circuit.contains(&point_1)
            {   
                new = false;
                circuit.push(point_2);
                num += 1;
                if num == 1
                {
                    circuit_index.0 = index;
                }
                else if num == 2
                {
                    circuit_index.1 = index;
                }
            }
            else if circuit.contains(&point_2)
            {   
                new = false;
                circuit.push(point_1);
                num += 1;
                if num == 1
                {
                    circuit_index.0 = index;
                }
                else if num == 2
                {
                    circuit_index.1 = index;
                }
            }

        }
        
        if num == 2
        {
            let temp = &circuits[circuit_index.1].clone();

            circuits[circuit_index.0].extend(temp);
            circuits.remove(circuit_index.1);

            let index_a = circuits[circuit_index.0].iter().position(|&r| r == point_1);
            
            // println!("{:?}", circuits[circuit_index.0].iter().position(|&r| r == point_1));
            circuits[circuit_index.0].remove(index_a.unwrap());
            
            let index_b = circuits[circuit_index.0].iter().position(|&r| r == point_2);
            circuits[circuit_index.0].remove(index_b.unwrap());

            // println!("Circuits to be joined using point {:?}", pair);
            // println!("{:?}", circuits[circuit_index.0].iter().position(|&r| r == point_2));
            // println!("{:?}", circuits[circuit_index.1]);
        }

        if new
        {
            circuits.push(vec![point_1, point_2]);
        }

        if circuits[0].len() == data.len()
        {
            println!("We got them all");

            println!("Total {}", data[pair.1.0].0*data[pair.1.1].0);
            // println!("Total: {}", point_1*point_2);
            break;
        }


    }

    // circuits.sort_by_key(|v| v.len() );
    // // print!("\n\n\n\n\n");
    // // circuits[0].sort();
    // // println!("{:?}", circuits);

    // let a = circuits.pop().unwrap().len();
    // let b = circuits.pop().unwrap().len();
    // let c = circuits.pop().unwrap().len();

    // println!("Total: {}", a*b*c);
}


fn calc_distances(a_coord: (i64, i64, i64), a_index: usize, b_coord: (i64, i64, i64), b_index: usize) -> (i64, (usize, usize))
{
    let a = a_coord.0 - b_coord.0;
    let dist_1 = a*a;
    let dist_2 = (a_coord.1 - b_coord.1)*(a_coord.1 - b_coord.1);
    let dist_3 = (a_coord.2 - b_coord.2)*(a_coord.2 - b_coord.2);

    let calc = (dist_1 + dist_2 + dist_3).isqrt();

    (calc, (a_index,b_index))
}
