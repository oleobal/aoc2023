use petgraph::{graph::Graph, Undirected, stable_graph::NodeIndex};
use petgraph::dot::{Dot, Config};

pub fn p1(input: String) {
    let mut graph = Graph::<char, (), Undirected>::new_undirected();
    
    let lines = input.trim().split("\n");
    let mut prev_line: Vec<Option<NodeIndex>> = Vec::from_iter([None , None]);
    for line in lines {
        let mut cur_line: Vec<Option<NodeIndex>> = Vec::new();
        for (index, c) in line.trim().chars().enumerate()
        {
            if c == '.'
            {
                cur_line.push(None);
            }
            else if c.is_digit(10) || c.is_ascii_punctuation()
            {
                cur_line.push(Some(graph.add_node(c)));
                if index>0 && index-1<prev_line.len() && !prev_line[index-1].is_none()
                {
                    graph.add_edge(cur_line.last().unwrap().unwrap(), prev_line[index-1].unwrap(), ());
                }
                if index<prev_line.len() && !prev_line[index].is_none()
                {
                    graph.add_edge(cur_line.last().unwrap().unwrap(), prev_line[index].unwrap(), ());
                }
                if index+1<prev_line.len() && !prev_line[index+1].is_none()
                {
                    graph.add_edge(cur_line.last().unwrap().unwrap(), prev_line[index+1].unwrap(), ());
                }
                if index>0 && !cur_line[index-1].is_none()
                {
                    graph.add_edge(cur_line[index-1].unwrap(), cur_line[index].unwrap(), ());
                }
            }
            else
            {
                panic!("unknown case for c: {}", c);
            }
        }
        
        prev_line = cur_line;
    }
    
    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
}
