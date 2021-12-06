use connect4::solver::Solver;
use connect4::Position;

use std::io;
use std::time;

const HUMAN_FIRST: bool = false;
const DEPTH: i32 = 20;

fn main() {
    play(HUMAN_FIRST);
}

fn play(human_first: bool) {
    let mut p = Position::new(7, 6);
    let mut board = vec![Vec::new(); p.width as usize];

    let result = loop {
        if human_first {
            let r = human_turn(&mut p, &mut board);
            if r {
                break "Human wins!";
            }

            print_board(&board);

            let r = computer_turn(&mut p, &mut board);
            if r {
                break "Computer wins!";
            }
        } else {
            let r = computer_turn(&mut p, &mut board);
            if r {
                break "Computer wins!";
            }
            print_board(&board);

            let r = human_turn(&mut p, &mut board);
            if r {
                break "Human wins!";
            }
            print_board(&board);
        }
    };

    print_board(&board);
    println!("{}", result);
}

fn computer_turn(p: &mut Position, b: &mut Vec<Vec<i32>>) -> bool {
    let ab = (p.width * p.height) / 2;
    let start = time::Instant::now();
    let (_, best, n_nodes) = Solver::solve(p, DEPTH, ab);
    let end = start.elapsed().as_secs_f64();
    println!(
        "computer played {}, searched {} nodes in {} seconds",
        best + 1,
        n_nodes,
        end
    );
    b[best as usize].push(2);
    if p.is_winning_move(best as usize) {
        p.play(best as usize);
        return true;
    }

    p.play(best as usize);

    false
}

fn human_turn(p: &mut Position, b: &mut Vec<Vec<i32>>) -> bool {
    println!("enter column");
    let mut col = String::new();
    io::stdin().read_line(&mut col).unwrap();
    let col = col.strip_suffix("\n").unwrap().parse::<usize>().unwrap() - 1;

    b[col].push(1);
    if p.is_winning_move(col) {
        p.play(col);
        return true;
    }

    p.play(col);

    false
}

fn print_board(b: &Vec<Vec<i32>>) {
    for (i, col) in b.iter().enumerate() {
        println!("{} {:?}", i + 1, col);
    }
    println!()
}
