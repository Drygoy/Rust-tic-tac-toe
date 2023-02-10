use std::io;

fn main() {
    let mut grid = [[' '; 3]; 3];
    let mut current_player = 'X';

    loop {
        print_grid(&grid);

        let mut input = String::new();
        println!("{}'s turn. Enter row and column:", current_player);

        io::stdin().read_line(&mut input)
            .expect("Failed to read input");

        let inputs: Vec<&str> = input.trim().split(' ').collect();
        let row: usize = inputs[0].parse().expect("Invalid row");
        let col: usize = inputs[1].parse().expect("Invalid column");

        if grid[row][col] == ' ' {
            grid[row][col] = current_player;
        } else {
            println!("Space already occupied, try again.");
            continue;
        }

        if has_won(current_player, &grid) {
            println!("{} wins!", current_player);
            break;
        }

        if is_draw(&grid) {
            println!("It's a draw.");
            break;
        }

        current_player = if current_player == 'X' { 'O' } else { 'X' };
    }
}

fn print_grid(grid: &[[char; 3]; 3]) {
    println!(" {} | {} | {} ", grid[0][0], grid[0][1], grid[0][2]);
    println!("---+---+---");
    println!(" {} | {} | {} ", grid[1][0], grid[1][1], grid[1][2]);
    println!("---+---+---");
    println!(" {} | {} | {} ", grid[2][0], grid[2][1], grid[2][2]);
}

fn has_won(player: char, grid: &[[char; 3]; 3]) -> bool {
    for i in 0..3 {
        if grid[i][0] == player && grid[i][1] == player && grid[i][2] == player {
            return true;
        }
        if grid[0][i] == player && grid[1][i] == player && grid[2][i] == player {
            return true;
        }
    }

    if grid[0][0] == player && grid[1][1] == player && grid[2][2] == player {
        return true;
    }
    if grid[0][2] == player && grid[1][1] == player && grid[2][0] == player {
        return true;
    }

    false
}

fn is_draw(grid: &[[char; 3]; 3]) -> bool {
    for row in grid.iter() {
        for space in row.iter() {
            if *space == ' ' {
                return false;
            }
        }
    }

    true
}
