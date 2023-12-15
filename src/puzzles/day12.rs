use std::{
    collections::VecDeque,
    fmt::{Display, Formatter},
};

use petgraph::visit::IntoNodeReferences;
use petgraph::{graph::Graph, stable_graph::NodeIndex, Directed};

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "src/puzzles/day12grammar.pest"]
struct Day12Parser;

#[derive(Debug, Hash, Clone)]
struct Record {
    map: String,
    bricks: Vec<String>,
    gsi: VecDeque<u32>, //henceforth christening this "group size info"
}

#[derive(Debug, Hash, Clone)]
struct Block {
    c: char,
    len: u8,
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
        let spring_map = inner.next().unwrap();

        let dcount: VecDeque<u32> = inner
            .next()
            .unwrap()
            .into_inner()
            .map(|it| it.as_str().parse::<u32>().unwrap())
            .collect();

        result.push(Record {
            map: spring_map.as_str().to_string(),
            bricks: spring_map
                .into_inner()
                .map(|it| it.as_str().to_string())
                .collect(),
            gsi: dcount,
        });
    }
    return result;
}

/// Returns whether a block (a string of #, ? and .) could answer the given GSI
/// this is for early pruning to avoid unnecessary computation
fn block_is_candidate(input: &str, gsi: &VecDeque<u32>) -> bool {
    if input.contains("?")
    {
        return true;
    }
    let blocks: Vec<_> = input.split(".").filter(|b| *b != "").collect();
    if blocks.len() != gsi.len() {
        return false;
    }
    for (i, b) in blocks.into_iter().enumerate() {
        if i >= gsi.len() {
            return false;
        }
        if b.len() != gsi[i].try_into().unwrap() {
            return false;
        }
    }
    return true;

    // let mut blocks = VecDeque::<Block>::new();
    // for c in input.chars() {
    //     if !blocks.is_empty() {
    //         let last_block = blocks.back_mut().unwrap();
    //         if last_block.c == c {
    //             last_block.len += 1;
    //         } else {
    //             blocks.push_back(Block { c: c, len: 1 });
    //         }
    //     } else {
    //         blocks.push_back(Block { c: c, len: 1 });
    //     }
    // }
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

    let sum = records
        .into_iter()
        .map(|r| {
            let tree = build_tree(&r);
            //println!("{:?}", Dot::with_config(&tree, &[Config::EdgeNoLabel]));

            let s = tree
                .node_references()
                .filter(|(_, node)| node.status == TreeNodeType::LEAF)
                .count();
            //println!("{}", s);

            return s;
        })
        .sum::<usize>();
    println!("{}", sum);
}

pub fn p2(input: String) {
    let records = parse_input(&input);

    let unfolded_records = records.into_iter().map(|r| {
        let blen = r.bricks.len() * 5;
        let gsilen = r.gsi.len() * 5;
        Record {
            map: r.map.repeat(5),
            bricks: r.bricks.into_iter().cycle().take(blen).collect(),
            gsi: r.gsi.into_iter().cycle().take(gsilen).collect(),
        }
    });

    let sum = unfolded_records
        .into_iter()
        .map(|r| {
            let tree = build_tree(&r);
            //println!("{:?}", Dot::with_config(&tree, &[Config::EdgeNoLabel]));

            let s = tree
                .node_references()
                .filter(|(_, node)| node.status == TreeNodeType::LEAF)
                .count();
            //println!("{}", s);

            return s;
        })
        .sum::<usize>();
    println!("{}", sum);
}
