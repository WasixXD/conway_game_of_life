use rand::Rng;
use std::{thread, time::Duration};
use std::io::Write;


fn count_neighbour(grid: &Vec<Vec<u8>>, x: i32, y: i32, height: i32, width: i32) -> i32 {
    
    let mut sum = 0;
    for i in -1..2 {
        for j in -1..2 {
            let col = ((y + i + height) % height) as i32;
            let row = ((x + j + width) % width) as i32;
            
            sum += grid[col as usize][row as usize];

        }
    }

    sum -= grid[y as usize][x as usize];

    return sum.into();
}


fn create_empty_2darray(width: usize, height: usize) -> Vec<Vec<u8>> {
    let mut array = vec![vec![0; width]; height];
    let mut rng = rand::thread_rng();
        
    for i in 0..height {
        for j in 0..width {
            let n1 = rng.gen_range(0..2);
            array[i][j] = n1;
        }
    }

    return array;
}


fn print_game(grid: &Vec<Vec<u8>>, w: i32, h: i32) {
    thread::sleep(Duration::from_millis(200));

    for i in 0..h {
    
        for j in 0..w {
            print!("{:?}", if grid[i as usize][j as usize] == 1 { "@" } else { " " });
            
        }
        print!("\n");
        std::io::stdout().flush();
     }

}


fn main() {
    const WIDTH: usize = 63;
    const HEIGHT: usize = 45;
    
    let mut grid:Vec<Vec<u8>> = create_empty_2darray(WIDTH, HEIGHT);





    //glider code
    //let mut grid:Vec<Vec<u8>> = vec![vec![0; WIDTH]; HEIGHT];

    // grid[1][0] = 1;
    // grid[2][1] = 1;
    // grid[2][2] = 1;
    // grid[1][2] = 1;
    // grid[0][2] = 1;


    let mut next = grid.clone();
    let mut g = 0;
    let mut p = 0;
    loop {
        //game logic 
        for y in 0..HEIGHT as i32 {
            for x in 0..WIDTH as i32 {
                let neighbors = count_neighbour(&grid, x, y, HEIGHT as i32, WIDTH as i32);
                let xu = x as usize;
                let yu =  y as usize;


                


                if grid[yu][xu] == 1 && neighbors < 2 {
                    next[yu][xu] = 0;
                } else if grid[yu][xu] == 1 && neighbors > 3 {
                    next[yu][xu] = 0;
                } else if grid[yu][xu] == 0 && neighbors == 3 {
                    next[yu][xu] = 1;
                    
                } else {
                    next[yu][xu] = grid[yu][xu];

                }
                
                if next[yu][xu] == 1 {
                    p += 1;
                }
                               
            }
        }  
        if p == 0 {
            break
        }
        print_game(&next, WIDTH as i32, HEIGHT as i32);
        println!("Generation: {} | Population: {}", g, p);
        
        grid = next.clone();
        g+= 1; 
        p = 0;
    }
    

    
}