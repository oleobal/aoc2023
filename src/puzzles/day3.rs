use petgraph::{graph::Graph, Undirected, stable_graph::NodeIndex};

fn build_graph(input: String) -> Graph<String, (), Undirected> {
    let mut graph = Graph::<String, (), Undirected>::new_undirected();
    
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
                // same line
                if index>0 && !cur_line.last().unwrap().is_none() {
                    let prev = cur_line.last().unwrap().unwrap();
                    
                    // merge with the previous number if needed
                    if c.is_digit(10) && graph[prev].chars().all(|c| c.is_numeric()) {
                        graph[prev].push(c);
                        cur_line.push(Some(prev));
                    }
                    else {
                        // same-line adjacency
                        cur_line.push(Some(graph.add_node(c.to_string())));
                        graph.update_edge(cur_line.last().unwrap().unwrap(), prev, ());
                    }
                }
                else {
                    cur_line.push(Some(graph.add_node(c.to_string())));
                }
                
                // adjacency with previous line
                if index>0 && index-1<prev_line.len() && !prev_line[index-1].is_none() {
                    graph.update_edge(cur_line.last().unwrap().unwrap(), prev_line[index-1].unwrap(), ());
                }
                if index<prev_line.len() && !prev_line[index].is_none() {
                    graph.update_edge(cur_line.last().unwrap().unwrap(), prev_line[index].unwrap(), ());
                }
                if index+1<prev_line.len() && !prev_line[index+1].is_none() {
                    graph.update_edge(cur_line.last().unwrap().unwrap(), prev_line[index+1].unwrap(), ());
                }
                
            }
            else
            {
                panic!("unknown case for c: {}", c);
            }
        }
        
        prev_line = cur_line;
    }
    return graph;
}

pub fn p1(input: String) {
    let graph = build_graph(input);
    
    let mut sum = 0;
    for index in graph.node_indices().filter(|i| graph[*i].chars().next().unwrap().is_ascii_punctuation()) {
        for n in graph.neighbors(index) {
            //println!("={}", graph[n]);
            sum += graph[n].parse::<i32>().unwrap();
        }
    }
    
    //println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    
    println!("{}", sum);
}

pub fn p2(input: String) {
    let graph = build_graph(input);
    
    let mut sum = 0;
    for index in graph.node_indices().filter(|i| graph[*i].chars().next().unwrap() == '*' && graph.neighbors(*i).count() == 2) {
        sum += graph.neighbors(index).fold(1, |acc, e| acc * graph[e].parse::<i32>().unwrap());
    }
    
    println!("{}", sum);
}
