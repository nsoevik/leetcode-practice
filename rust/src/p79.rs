use std::collections::HashSet;

type Point = (usize, usize);
const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        fn recurse(
            board: &Vec<Vec<char>>,
            curr_index: usize,
            word: &Vec<char>,
            curr_point: &Point,
            h: &HashSet<Point>,
        ) -> bool {
            if h.contains(&curr_point) {
                return false;
            }

            let char_in_grid = board[curr_point.0][curr_point.1];
            if char_in_grid != word[curr_index] {
                return false;
            }

            if curr_index == word.len() - 1 {
                return true;
            }

            let mut found = false;
            for (x, y) in DIRECTIONS {
                let new_x: Result<usize, _> = (curr_point.0 as i32 + x).try_into();
                let new_y: Result<usize, _> = (curr_point.1 as i32 + y).try_into();
                if new_x.is_ok_and(|x| x < board.len()) {
                    let x_val = new_x.unwrap();
                    if new_y.is_ok_and(|y| y < board[x_val].len()) {
                        let y_val = new_y.unwrap();
                        let point = (x_val, y_val);
                        let mut h = h.clone();
                        h.insert(*curr_point);
                        found |= recurse(&board, curr_index + 1, &word, &point, &h);
                    }
                }
            }

            found
        }

        let chars: Vec<char> = word.chars().collect();
        let h = HashSet::<Point>::new();
        for i in 0..board.len() {
            for j in 0..board[i].len() {
                if chars[0] == board[i][j] {
                    if recurse(&board, 0, &chars, &(i, j), &h) {
                        return true;
                    }
                }
            }
        }

        false
    }
}

struct Solution;
