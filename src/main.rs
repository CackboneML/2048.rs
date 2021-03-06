extern crate rand;
use rand::distributions::{IndependentSample, Range};
use rand::Rng;
use std::io;
use std::convert::AsRef;
use std::process::Command;

enum Direction {
	Up, Right, Down, Left
}

fn main()
{
    // Grid 4*4
    let mut grid  = [[0i8; 4];4];
    let mut key;
    let mut valid = true;
    let mut grid_bis = [[0i8; 4];4];

    // Initialization
    init_game(&mut grid);

    // Main game-loop
    loop
    {
        grid_bis = grid;
        key = input_key();
        match key.as_ref()
        {
            "z\n" => move_grid(Direction::Up, &mut grid),
            "q\n" => move_grid(Direction::Left, &mut grid),
            "d\n" => move_grid(Direction::Right, &mut grid),
            "s\n" => move_grid(Direction::Down, &mut grid),
            _ => valid = false,
        }

        if valid && !same_grid(grid, grid_bis)
        {
            add_tile(&mut grid);
            display_grid(grid);
        }
        else
        {
            valid = true;

            if grid_full(grid)
            {
                println!("Game over!");
                break;
            }
        }
            Command::new("clear").status();
            println!("\t\t\t-- 2048 --");
            display_grid(grid);
        }
}

fn grid_full(grid: [[i8; 4]; 4]) -> bool
{
    for y in 0..4
    {
        for x in 0..4
        {
            if grid[x][y] == 0
            {
                return false;
            }
        }
    }
    return true;
}

fn init_game(mut grid: &mut [[i8; 4]; 4])
{
    Command::new("clear").status();
    println!("\t\t\t-- 2048 --");
    add_tile(&mut grid);
    display_grid(*grid);
}

fn same_grid(grid: [[i8; 4]; 4], grid_bis: [[i8; 4]; 4]) -> bool
{
    for y in 0..4
    {
        for x in 0..4
        {
            if grid[x][y] != grid_bis[x][y]
            {
                return false;
            }
        }
    }
    return true;
}


fn move_grid(d: Direction, mut grid: &mut [[i8; 4]; 4])
{
	let mut x_delta:i8=0;
	let mut y_delta:i8=0;

	match d {
		Direction::Up => 	x_delta = 1,
		Direction::Down => 	x_delta = -1,
		Direction::Right => y_delta = -1,
		Direction::Left => 	y_delta = 1
	}
	for _ in 0..4
	{
		for x in (if x_delta == 1 {1} else {0})..(if x_delta == -1 {grid.len() - 1} else {grid.len()})
		{
			for y in (if y_delta == 1 {1} else {0})..(if y_delta == -1 {grid.len() - 1} else {grid.len()})
			{
				if grid[((x as i8) - x_delta) as usize][((y as i8) - y_delta) as usize] == 0
				{
					grid[((x as i8) - x_delta) as usize][((y as i8) - y_delta) as usize] = grid[x][y];
					grid[x][y] = 0;
				}
				else if grid[((x as i8) - x_delta) as usize][((y as i8) - y_delta) as usize] == grid[x][y]
				{
					grid[((x as i8) - x_delta) as usize][((y as i8) - y_delta) as usize] += grid[x][y];
					grid[x][y] = 0;
				}
			}
		}
	}
}

fn input_key() -> String
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    return input;
}


fn add_tile(mut grid: &mut [[i8; 4]; 4])
{
    let free_tiles = get_free_tiles(*grid);
    let range = Range::new(0, free_tiles.len());
    let mut rand = rand::thread_rng();
    let rand_tile;
    let values = [2, 2, 2, 4]; // Tile value 2 or 4
    let mut rand_value = rand::thread_rng();
    let value;
    let index;


    rand_tile = range.ind_sample(&mut rand);
    value = match rand_value.choose(&values)
    {
        Some(i) => *i,
        None => 0,
    };
    index = free_tiles[rand_tile];
    grid[index.0][index.1] = value;
}

fn get_free_tiles(grid: [[i8; 4]; 4]) -> Vec<(usize, usize)>
{
    let mut free_tiles = Vec::new();
    let mut x_index = 0;
    let mut y_index = 0;

    for x in grid.iter()
    {
        for y in x.iter()
        {
            if *y == 0
            {
                free_tiles.push((x_index, y_index));
            }
            y_index += 1;
        }
        x_index += 1;
        y_index = 0;
    }

    return free_tiles;
}

/** Diplay functions **/

fn display_grid(grid: [[i8; 4]; 4])
{
    for x in grid.iter()
    {
        print!("\t");
        print_line('-');;
        print_blocks();
        print!("\t|");
        for y in x.iter()
        {
            match *y
            {
                y if y == 0 => print!("         |"),
                y if y < 10 => print!("    {}    |", y),
                y if y >= 10 => print!("   {}    |", y),
                y if y >= 100 => print!("  {}    |", y),
                _ => print!("  {}    ", y),
            }

        }
        print!("\n");
        print_blocks();
    }
    print!("\t");
    print_line('-');
}

fn print_line(ln_char: char)
{
    for _ in 0..41
    {
        print!("{}", ln_char);
    }
    print!("\n");
}

fn print_blocks()
{
    print!("\t");
    for _ in 0..4
    {
        print!("|         ")
    }
    print!("|\n");
}
