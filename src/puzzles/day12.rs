use std::{
    collections::VecDeque,
    fmt::{Display, Formatter},
    iter,
    str::Chars,
};

use petgraph::{dot::*, visit::{NodeCount, IntoNodeReferences}};
use petgraph::{graph::Graph, stable_graph::NodeIndex, Directed};
use std::collections::{HashMap, HashSet};

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "src/puzzles/day12grammar.pest"]
struct Day12Parser;

#[derive(Debug, Hash, Clone)]
struct Record {
    map: String,
    gsi: VecDeque<u32>, //henceforth christening this "group size info"
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum TreeNodeType {
    CANDIDATE,
    LEAF,
    PRUNED,
}

#[derive(Debug)]
struct TreeNode {
    map: String,
    status: TreeNodeType,
}

impl Display for Record {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "({}, {:?})", self.map, self.gsi)
    }
}

fn parse_input(input: &str) -> Vec<Record> {
    let tokens = Day12Parser::parse(Rule::Records, input).unwrap();

    let mut result = Vec::new();
    for record in tokens {
        let mut inner = record.into_inner();
        let spring_map = inner.next().unwrap().as_str();

        let dcount: VecDeque<u32> = inner
            .next()
            .unwrap()
            .into_inner()
            .map(|it| it.as_str().parse::<u32>().unwrap())
            .collect();

        result.push(Record {
            map: spring_map.to_string(),
            gsi: dcount,
        });
    }
    return result;
}

/// Returns whether a block (a string of #, ? and .) could answer the given GSI
fn block_is_candidate(block: &str, gsi: &VecDeque<u32>) -> bool {
    //let _full_str = block;
    //let mut _index = 0;
    let mut block = block.chars();
    let mut gsi = gsi.clone();
    let mut seclen = 0;
    let mut seclen_starting_question_marks = 0;
    while let Some(c) = block.next() {
        //_index+=1;
        //println!("{:>2} {} {:>15} | seclen: {:>2} (sqm: {:>2}) | gsi: {:?}", _index, c, &_full_str[_index..], seclen, seclen_starting_question_marks, gsi);
        if c == '#' || c == '?' {
            if c == '?' && seclen == seclen_starting_question_marks {
                seclen_starting_question_marks+=1;
            }
            if c == '#' && gsi.len() == 0 {
                return false;
            }
            
            if gsi.len() == 0
            {
                continue;
            }
            seclen += 1;
            if seclen == gsi[0] {
                // we must check that it is exactly the right size
                // -> it is followed by nothing
                // -> it is followed by .
                // -> it is followed by ?
                // -> it starts with ?
                let endc = block.next();
                //_index+=1;
                if endc.is_none() {
                    gsi.pop_front();
                    seclen = 0;
                    seclen_starting_question_marks=0;
                } else if endc.is_some_and(|c| c == '?' || c == '.') {
                    gsi.pop_front();
                    seclen = 0;
                    seclen_starting_question_marks = 0;
                } else if seclen_starting_question_marks > 0 {
                    // I have a feeling this only covers one character and multiple are needed
                    gsi.pop_front();
                    seclen = 0;
                    seclen_starting_question_marks = 0;
                    //seclen -= 1;
                    //seclen_starting_question_marks-=1;
                } else {
                    return false;
                }
            }
        } else {
            if seclen - seclen_starting_question_marks > 0 {
                return false;
            }
            
            seclen = 0;
            seclen_starting_question_marks = 0;
        }
    }
    return gsi.len() == 0;
}

fn build_tree(r: &Record) -> Graph<TreeNode, ()> {
    let mut tree = Graph::<TreeNode, (), Directed>::new();

    fn _determine_map_type(map: &str, gsi: &VecDeque<u32>) -> TreeNodeType {
        if !block_is_candidate(map, &gsi) {
            return TreeNodeType::PRUNED;
        }
        if map.contains("?") {
            return TreeNodeType::CANDIDATE;
        }
        return TreeNodeType::LEAF;
    }

    fn _expand_tree(
        tree: &mut Graph<TreeNode, ()>,
        parent: NodeIndex,
        gsi: &VecDeque<u32>,
        target_char: &str,
    ) {
        let new_map = tree
            .node_weight(parent)
            .unwrap()
            .map
            .replacen("?", target_char, 1);

        let new_map_type = _determine_map_type(&new_map, gsi);

        let new_node = tree.add_node(TreeNode {
            map: new_map,
            status: new_map_type,
        });

        tree.add_edge(parent, new_node, ());

        if new_map_type == TreeNodeType::CANDIDATE {
            _expand_tree(tree, new_node, gsi, "#");
            _expand_tree(tree, new_node, gsi, ".");
        }
    }
    

    let root_node_type = _determine_map_type(&r.map, &r.gsi);
    let root_node = tree.add_node(TreeNode {
        map: r.map.clone(),
        status: root_node_type,
    });
    if root_node_type == TreeNodeType::CANDIDATE {
        _expand_tree(&mut tree, root_node, &r.gsi, "#");
        _expand_tree(&mut tree, root_node, &r.gsi, ".");
    }
    
    return tree;
}

pub fn p1(input: String) {
    let records = parse_input(&input);

    
    // let tree = build_tree(&records[5]);
    // println!("{:?}", Dot::with_config(&tree, &[Config::EdgeNoLabel]));
    // let s = tree.node_references().filter(|(_, node)| node.status==TreeNodeType::LEAF).count();
    // println!("{}", s);
    
    // let c = block_is_candidate("#.##", &VecDeque::from([1]));
    // println!("{}", c);
    
    
    let sum = records.into_iter().map(|r| {
        let tree = build_tree(&r);
        //println!("{:?}", Dot::with_config(&tree, &[Config::EdgeNoLabel]));
        
        let s = tree.node_references().filter(|(_, node)| node.status==TreeNodeType::LEAF).count();
        //println!("{}", s);
        return s;
    }).sum::<usize>();
    println!("{}", sum);
    
    
    
    //println!("{:?}", tree);
}
