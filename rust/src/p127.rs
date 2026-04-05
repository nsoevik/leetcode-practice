use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        if !word_list.contains(&end_word) {
            return 0;
        }

        let mut m: HashMap<String, Vec<String>> = HashMap::new();

        fn get_wildcards(word: &str) -> Vec<String> {
            let mut ret = Vec::new();
            for i in 0..word.len() {
                let start = &word[0..i];
                let end = &word[i + 1..];
                ret.push(format!("{}%{}", start, end));
            }
            ret
        }

        for word in word_list.iter().chain(std::iter::once(&begin_word)) {
            for w in get_wildcards(word) {
                m.entry(w).or_insert(Vec::new()).push(word.clone());
            }
        }

        let mut dq = VecDeque::new();
        let mut visited = HashSet::new();

        dq.push_back(begin_word.clone());
        visited.insert(begin_word);

        let mut depth = 1;

        while !dq.is_empty() {
            for _ in 0..dq.len() {
                let curr = dq.pop_front().unwrap();

                if curr == end_word {
                    return depth;
                }

                for wildcard in get_wildcards(&curr) {
                    if let Some(next_words) = m.get_mut(&wildcard) {
                        for next in next_words.iter() {
                            if !visited.contains(next) {
                                visited.insert(next.clone());
                                dq.push_back(next.clone());
                            }
                        }
                        next_words.clear();
                    }
                }
            }
            depth += 1;
        }

        0
    }
}

struct Solution;
