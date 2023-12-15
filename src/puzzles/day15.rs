use regex::Regex;
use std::{collections::VecDeque, iter};

fn calculate_hash(input: &str) -> u32 {
    let mut val = 0;
    for c in input.chars() {
        val = (val + (c as u32)) * 17 % 256;
    }
    return val;
}

pub fn p1(input: String) {
    println!(
        "{}",
        input.trim().split(",").map(calculate_hash).sum::<u32>()
    );
}

struct Op {
    label: String,
    op: char,
    length: Option<u8>,
}

fn parse_ops(input: String) -> Vec<Op> {
    let re = Regex::new(r"(?<label>[a-z]+)(?<op>[=-])(?<length>\d?)").unwrap();

    input
        .trim()
        .split(",")
        .map(|s| {
            let c = re.captures(s).unwrap();
            if &c["op"] == "-" {
                return Op {
                    label: c["label"].to_string(),
                    op: '-',
                    length: None,
                };
            } else {
                return Op {
                    label: c["label"].to_string(),
                    op: '=',
                    length: Some(c["length"].parse::<u8>().unwrap()),
                };
            }
        })
        .collect()
}

fn _represent_boxes(boxes: &Vec<VecDeque<(String, u8)>>) -> String {
    boxes
        .into_iter()
        .enumerate()
        .filter(|(_i, b)| b.len() > 0)
        .map(|(i, b)| {
            format!(
                "Box {:>3}: {}\n",
                i,
                b.into_iter()
                    .map(|(label, length)| format!("[{} {}] ", label, length))
                    .collect::<String>()
            )
        })
        .collect::<String>()
}

fn compute_focusing_power(boxes: &Vec<VecDeque<(String, u8)>>) -> u32 {
    boxes
        .iter()
        .enumerate()
        .filter(|(_b_i, b)| b.len() > 0)
        .map(|(b_i, b)| {
            b.iter()
                .enumerate()
                .map(|(l_i, (_label, length))| (b_i + 1) * (l_i + 1) * *length as usize)
                .sum::<usize>()
        })
        .sum::<usize>() as u32
}

pub fn p2(input: String) {
    let ops = parse_ops(input);

    let mut boxes = iter::repeat(VecDeque::<(String, u8)>::new())
        .take(256)
        .collect::<Vec<VecDeque<(String, u8)>>>();

    for op in ops {
        let target_box = &mut boxes[calculate_hash(&op.label) as usize];
        let existing_i = target_box
            .iter()
            .position(|(label, _length)| *label == op.label);
        if op.op == '=' {
            if existing_i.is_some() {
                target_box[existing_i.unwrap()].1 = op.length.unwrap();
            } else {
                target_box.push_back((op.label, op.length.unwrap()))
            }
        } else {
            if existing_i.is_some() {
                target_box.remove(existing_i.unwrap());
            }
        }
    }

    //println!("{}", _represent_boxes(&boxes));
    println!("{}", compute_focusing_power(&boxes));
}
