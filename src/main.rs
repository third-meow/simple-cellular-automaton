
use std::{thread, time};

const ENV_HEIGHT: usize = 30;
const ENV_WIDTH: usize = 30;

fn print_2d_vec(v: &Vec<Vec<u32>>) {
    for i in v.iter() {
        for j in i.iter() {
            if *j > 0 {
                print!("{}[41m{}{}[40m ",27 as char, j, 27 as char);
            } else {
                print!("{} ", j);
            }
        }
        println!("");
    }
}

fn clear_tui() {
    print!("{}[2J", 27 as char);
}

fn get_surrounding_free_cells(x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut v: Vec<(usize, usize)> = Vec::new();

    if y >= 1 {
        v.push((x, y-1));
        if x >= 1 {
            v.push((x-1, y-1));
        }
        if x < (ENV_HEIGHT - 1) {
            v.push((x+1, y-1));
        }
    }

    if y < (ENV_HEIGHT - 1) {
        v.push((x, y+1));
        if x >= 1 {
            v.push((x-1, y+1));
        }
        if x < (ENV_HEIGHT - 1) {
            v.push((x+1, y+1));
        }
    }

    if x < (ENV_HEIGHT - 1) {
        v.push((x+1, y));
    }
    if x >= 1 {
        v.push((x-1, y));
    }
    v
}

fn kill_surrounding_cells(env: &mut Vec<Vec<u32>>, x: usize, y: usize){
    if y >= 1 {
        env[x][y-1] = 0;
        if x >= 1 {
            env[x-1][y-1] = 0;
        }
        if x < (ENV_HEIGHT - 1) {
            env[x+1][y-1] = 0;
        }
    }

    if y < (ENV_HEIGHT - 1) {
        env[x][y+1] = 0;
        if x >= 1 {
            env[x-1][y+1] = 0;
        }
        if x < (ENV_HEIGHT - 1) {
            env[x+1][y+1] = 0;
        }
    }

    if x < (ENV_HEIGHT - 1) {
        env[x+1][y] = 0;
    }
    if x >= 1 {
        env[x-1][y] = 0;
    }
}

fn take_step(env: &mut Vec<Vec<u32>>){
    for i in 0..env.len() {
        for j in 0..env[i].len() {
            if env[i][j] > 1 {
                env[i][j] -= 1;
            } else if env[i][j] == 1 as u32 {
                let free_cells = get_surrounding_free_cells(i, j);

                if free_cells.len() < 2 {
                    kill_surrounding_cells(env, i, j);
                } else {
                    for (x, y) in free_cells.iter() {
                        env[*x][*y] = 4;
                    }
                }
                env[i][j] = 0;
            }
        }
    }
}

fn main() {
    let mut env: Vec<Vec<u32>> = Vec::new();
    for i in 0..ENV_HEIGHT {
        env.push(Vec::new());
        for _ in 0..ENV_WIDTH {
            env[i].push(0);
        }
    }

    env[0][0] = 4;

    loop {
        clear_tui();
        print_2d_vec(&env);
        thread::sleep(time::Duration::from_millis(100));
        take_step(&mut env);
    }

}
