use super::Position;
use std::collections::HashMap;

pub struct Solver {
    pub node_count: u128,
    column_order: Vec<usize>,
    pub trans_table: HashMap<u64, i32>,
}

impl Solver {
    pub fn new(width: i32, height: i32) -> Self {
        let trans_table = HashMap::new();
        let mut column_order = Vec::new();
        for i in 0..width {
            column_order.push((width / 2 + (1 - 2 * (i % 2)) * (i + 1) / 2) as usize);
        }

        Solver {
            node_count: 0,
            column_order,
            trans_table,
        }
    }

    pub fn solve(p: &mut Position, depth: i32, ab: i32, id: bool) -> (i32, i32, u128) {
        if !id {
            let mut s = Solver::new(p.width, p.height);
            let (eval, best) = s.negamax(p, -ab, ab, depth);
            return (eval, best, s.node_count);
        }

        let mut min = -ab;
        let mut max = ab;

        let mut s = Solver::new(p.width, p.height);

        let mut bm = 0;

        while min < max {
            let mut med = min + (max - min) / 2;

            if med <= 0 && min / 2 < med {
                med = min / 2;
            } else if med >= 0 && max / 2 > med {
                med = max / 2;
            }

            let (eval, best) = s.negamax(p, med, med + 1, depth);

            if eval <= med {
                max = eval;
            } else {
                min = eval;
                bm = best;
            }
        }

        (min, bm, s.node_count)
    }

    fn negamax(
        &mut self,
        position: &mut Position,
        mut alpha: i32,
        mut beta: i32,
        depth: i32,
    ) -> (i32, i32) {
        self.node_count += 1;
        if depth == 0 {
            for i in 0..position.width {
                if position.can_play(self.column_order[i as usize]) {
                    return (0, self.column_order[i as usize] as i32);
                }
            }
        }
        if position.moves == (position.width * position.height) {
            return (0, 0);
        }

        for i in 0..position.width {
            if position.can_play(i as usize) && position.is_winning_move(i as usize) {
                return (
                    (position.width * position.height + 1 - position.moves) / 2,
                    i,
                );
            }
        }

        let mut max = (position.width * position.height - 1 - position.moves) / 2;

        let k = position.key();
        if self.trans_table.contains_key(&k) {
            max = 1 + position.min_score + self.trans_table[&k];
        }

        if beta > max {
            beta = max;
            if alpha >= beta {
                return (beta, 0);
            }
        }

        let mut best = 3;

        for i in 0..position.width {
            if position.can_play(self.column_order[i as usize]) {
                let mut p2 = position.clone();
                p2.play(self.column_order[i as usize]);

                let (score, _) = self.negamax(&mut p2, -beta, -alpha, depth - 1);
                let score = -score;
                if score >= beta {
                    return (score, self.column_order[i as usize] as i32);
                }
                if score > alpha {
                    alpha = score;
                    best = self.column_order[i as usize] as i32;
                }
            }
        }

        self.trans_table
            .insert(position.key(), alpha - position.min_score + 1);

        (alpha, best)
    }
}
