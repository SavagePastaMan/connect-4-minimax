use connect4::solver::Solver;
use connect4::Position;

use std::io;
use std::thread::sleep;
use std::time;

const HUMAN_FIRST: bool = true;
const DEPTH: i32 = 10;
const ITERATIVE_DEEPENING: bool = true;

fn main() {
    //human_vs_ai(HUMAN_FIRST);
    ai_vs_ai(22, 11);
}

fn ai_vs_ai(d1: i32, d2: i32) {
    let mut p = Position::new(7, 6);
    let mut b = vec![vec!["~"; p.width as usize]; p.height as usize];

    let result = loop {
        let (r, summary) = computer_turn(&mut p, &mut b, d1, "o", false);
        println!(
            "Computer1 played {}, searched {} nodes in {} seconds; evaluation: {}",
            summary.0, summary.1, summary.2, summary.3,
        );
        print_board(&b);
        if r {
            break "Computer 1 Wins";
        }

        let (r, summary) = computer_turn(&mut p, &mut b, d2, "x", false);
        println!(
            "Computer2 played {}, searched {} nodes in {} seconds; evaluation: {}",
            summary.0, summary.1, summary.2, summary.3,
        );
        print_board(&b);
        if r {
            break "Computer 2 Wins";
        }

        if p.moves == p.width * p.height {
            break "Draw";
        }
        sleep(time::Duration::from_secs(1));
    };

    println!("{}", result);
}

fn human_vs_ai(human_first: bool) {
    let mut p = Position::new(7, 6);
    let mut board = vec![vec!["~"; p.width as usize]; p.height as usize];

    let result = loop {
        print_board(&board);
        if human_first {
            let r = human_turn(&mut p, &mut board);
            if r {
                break "Human wins!";
            }

            print_board(&board);

            let (r, summary) = computer_turn(&mut p, &mut board, DEPTH, "x", ITERATIVE_DEEPENING);
            println!(
                "computer played {}, searched {} nodes in {} seconds",
                summary.0, summary.1, summary.2,
            );
            println!("Computer Evaluation: {}", summary.3);
            if r {
                break "Computer wins!";
            }
        } else {
            let (r, summary) = computer_turn(&mut p, &mut board, DEPTH, "x", ITERATIVE_DEEPENING);
            println!(
                "computer played {}, searched {} nodes in {} seconds",
                summary.0, summary.1, summary.2,
            );
            println!("Computer Evaluation: {}", summary.3);
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

fn computer_turn(
    p: &mut Position,
    b: &mut Vec<Vec<&'static str>>,
    depth: i32,
    piece: &'static str,
    id: bool,
) -> (bool, (i32, u128, f64, i32)) {
    let ab = (p.width * p.height) / 2;
    let start = time::Instant::now();
    let (eval, best, n_nodes) = Solver::solve(p, depth, ab, id);
    let end = start.elapsed().as_secs_f64();

    let summary = (best + 1, n_nodes, end, eval);

    add_to_board(b, best as usize, piece);

    if p.is_winning_move(best as usize) {
        p.play(best as usize);
        return (true, summary);
    }

    p.play(best as usize);

    (false, summary)
}

fn human_turn(p: &mut Position, b: &mut Vec<Vec<&'static str>>) -> bool {
    println!("enter column");
    let mut col = String::new();
    io::stdin().read_line(&mut col).unwrap();
    let col = col.strip_suffix("\n").unwrap().parse::<usize>().unwrap() - 1;

    add_to_board(b, col, "o");

    if p.is_winning_move(col) {
        p.play(col);
        return true;
    }

    p.play(col);

    false
}

fn print_board(b: &Vec<Vec<&'static str>>) {
    println!("_____________________");
    for row in b.iter() {
        let out = row.join("  ");
        println!("|{}|", out);
    }
    println!(" 1  2  3  4  5  6  7");
}

fn add_to_board(b: &mut Vec<Vec<&'static str>>, col: usize, piece: &'static str) {
    for row in b.iter_mut().rev() {
        if row[col] == "~" {
            row[col] = piece;
            break;
        }
    }
}
