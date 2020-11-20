use crate::utils::to_word;
use crate::utils::ALPH;
use indicatif::ProgressBar;
use indicatif::ProgressIterator;
use petgraph::graph::{Edges, NodeIndex};
use petgraph::visit::EdgeRef;
use petgraph::Direction;
use petgraph::{Directed, Graph}; // todo use daggy?
use rayon::prelude::*;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

fn load_from_file<T: DeserializeOwned + Serialize>(file: &str, callback: fn() -> T) -> T {
    match fs::read(file) {
        Ok(b) => {
            println!("Loaded from file {}", file);
            bincode::deserialize(&b).unwrap()
        }
        Err(_) => {
            let t = callback();
            let serialized = bincode::serialize(&t).unwrap();
            match fs::write(file, &serialized) {
                Ok(_) => {
                    println!("Saving successful");
                }
                Err(e) => {
                    println!("error {}", e);
                }
            };
            t
        }
    }
}
pub trait IDictionary {
    fn default() -> Self where Self: Sized;
    fn evaluate(&self, rack: &Vec<usize>) -> Option<&f32>;
}
#[derive(Deserialize, Serialize)]
pub struct Dictionary {
    leaves: HashMap<Vec<usize>, f32>,
}

impl IDictionary for Dictionary {
    fn default() -> Dictionary {
        load_from_file("dict.ser", || {
            let mut dict = Dictionary {
                leaves: HashMap::new(),
            };
            let bar = ProgressBar::new(40);

            dict.leaves = fs::read_to_string("resources/leaves.txt")
                .expect("No leaves file")
                .lines()
                .map(String::from)
                .collect::<Vec<String>>()
                .par_iter()
                .map(|line| {
                    let s: Vec<&str> = line.split(" ").collect();
                    let word = to_word(&s[0].chars().collect());
                    let eval = s[1].parse::<f32>().unwrap();
                    (word, eval)
                })
                .collect();

            dict.leaves.insert(
                vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
                0.0,
            );
            bar.finish();

            dict
        })
    }

    fn evaluate(&self, rack: &Vec<usize>) -> Option<&f32> {
        self.leaves.get(rack)
    }
}

#[derive(Deserialize, Serialize)]
pub struct Trie {
    pub graph: Graph<char, char>,
    pub current: NodeIndex<u32>,
}

pub trait ITrie<NodeIndexType> {
    fn default() -> Self where Self: Sized;
    fn root(&self) -> NodeIndexType;
    fn seed(&self, initial: &Vec<char>) -> NodeIndexType;
    fn can_next(&self, current: NodeIndexType, next: char) -> Option<NodeIndexType>;
    fn nexts(&self, current: NodeIndexType) -> Vec<(char, NodeIndexType)>;
    fn check_word(&self, word: &String) -> bool;
}

impl Trie {
    fn hashroot(&self) -> NodeIndex {
        self.can_next(self.root(), '#').unwrap()
    }

    // -> [Option<(char, NodeIndex)>; 26]
    fn _nexts(&self, current: NodeIndex) -> Edges<char, Directed, u32> {
        self.graph.edges_directed(current, Direction::Outgoing)
    }
}

impl ITrie<NodeIndex> for Trie {
    fn default() -> Trie {
        load_from_file("trie.ser", || {
            let mut graph = Graph::new();
            let current = graph.add_node(' ');
            let mut trie = Trie { graph, current };

            let mut last_node;

            let extend = |t: &mut Trie, ln, c| {
                if let Some(new) = t.can_next(ln, c) {
                    return new;
                } else {
                    let next_node = t.graph.add_node(c);
                    t.graph.add_edge(ln, next_node, c.clone());
                    return next_node;
                }
            };

            let dummy = extend(&mut trie, current, '#');

            for i in ALPH.chars().progress() {
                if i == '?' {
                    continue;
                }
                let i_node = extend(&mut trie, dummy, i);

                for j in ALPH.chars() {
                    if j == '?' {
                        continue;
                    }
                    let j_node = extend(&mut trie, i_node, j);

                    let dipth: String = i.to_string() + &j.to_string();
                    let filepath = format!("resources/{}.txt", dipth);

                    let words: Vec<String> = fs::read_to_string(filepath)
                        .expect(&dipth)
                        .lines()
                        .map(String::from)
                        .collect();

                    for word in words {
                        last_node = j_node;

                        for c in word.chars().skip(2) {
                            last_node = extend(&mut trie, last_node, c);
                        }

                        extend(&mut trie, last_node, '@'); // EOW

                        for l in 1..word.len() {
                            last_node = current;
                            let v: Vec<char> = word.chars().take(l).collect();
                            for c in v.iter().rev() {
                                last_node = extend(&mut trie, last_node, *c);
                            }

                            last_node = extend(&mut trie, last_node, '#');

                            for c in word.chars().skip(l) {
                                last_node = extend(&mut trie, last_node, c);
                            }

                            extend(&mut trie, last_node, '@');
                        }
                    }
                }
            }

            trie
        })
    }

    fn root(&self) -> NodeIndex {
        self.graph.node_indices().next().unwrap()
    }

    fn seed(&self, initial: &Vec<char>) -> NodeIndex {
        let edges = self.graph.raw_edges(); // todo: optimize away
        let mut current = self.hashroot();

        for c in initial {
            for a in self.graph.edges_directed(current, Direction::Outgoing) {
                let e = &edges[a.id().index()];
                if e.weight == *c {
                    current = e.target();
                    break;
                }
            }
        }

        current
    }

    fn can_next(&self, current: NodeIndex, next: char) -> Option<NodeIndex> {
        let edges = self.graph.raw_edges();
        for a in self.graph.edges_directed(current, Direction::Outgoing) {
            let e = &edges[a.id().index()];
            if e.weight == next {
                return Some(e.target());
            }
        }

        None
    }

    fn nexts(&self, current: NodeIndex) -> Vec<(char, NodeIndex)> {
        let edges = self.graph.raw_edges();
        self._nexts(current)
            .map(|a| {
                let e = &edges[a.id().index()];
                (e.weight, e.target())
            })
            .collect()
    }

    fn check_word(&self, word: &String) -> bool {
        let mut current_node = self.hashroot();
        for current_char in word.chars() {
            if let Some(next_node) = self.can_next(current_node, current_char) {
                current_node = next_node;
            } else {
                return false;
            }
        }
        self.can_next(current_node, '@').is_some()
    }
}