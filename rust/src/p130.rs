use std::collections::{HashSet, VecDeque};

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];
type Point = (usize, usize);

fn prnt<T>(label: &str, val: &T)
where
    T: std::fmt::Debug,
{
    println!("{:?}: {:?}", label, val);
}

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        fn out_of_bounds(row: i32, col: i32, board_rows: i32, board_cols: i32) -> bool {
            return row < 0 || col < 0 || row >= board_rows || col >= board_cols;
        }

        for i in 0..board.len() {
            for j in 0..board[i].len() {
                if board[i][j] == 'O' {
                    let mut dq: VecDeque<Point> = VecDeque::new();
                    let mut visited: HashSet<Point> = HashSet::new();
                    let mut seen_outside = false;
                    dq.push_back((i, j));

                    while !dq.is_empty() {
                        let (curr_i, curr_j) = dq.pop_front().unwrap();
                        let new_point = (curr_i, curr_j);
                        if visited.contains(&new_point) {
                            continue;
                        }
                        visited.insert(new_point);
                        if board[curr_i][curr_j] == 'O' {
                            for (dir_row, dir_col) in DIRECTIONS {
                                let new_row = (curr_i as i32) + dir_row;
                                let new_col = (curr_j as i32) + dir_col;
                                if out_of_bounds(
                                    new_row,
                                    new_col,
                                    board.len() as i32,
                                    board[0].len() as i32,
                                ) {
                                    seen_outside = true;
                                    break;
                                }

                                dq.push_back((new_row as usize, new_col as usize));
                            }
                        }
                    }

                    if !seen_outside {
                        for p in visited {
                            board[p.0][p.1] = 'X';
                        }
                    }
                }
            }
        }
    }
}

struct Solution;
