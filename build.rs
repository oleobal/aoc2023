use std::fs;

use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

#[derive(Parser)]
#[grammar = "src/puzzles/day19grammar.pest"]
struct Day19Parser;

fn write_day19_programs() {
    fn parse_workflow(f_prefix: &str, p: Pair<'_, Rule>) -> String {
        let mut tokens = p.into_inner();
        let id = tokens.next().unwrap().as_str();

        let rules = tokens
            .map(|rule| {
                let buf: Vec<Pair<'_, Rule>> = rule.into_inner().collect();
                let mut cond = "true".to_string();
                let mut dest = "".to_string();
                if buf.len() == 2
                /* condition and destination */
                {
                    cond = buf[0].as_str().to_string();
                    dest = buf[1].as_str().to_string();
                } else
                /* just dest */
                {
                    dest = buf[0].as_str().to_string();
                }

                if dest == "A" {
                    dest = "true".to_string();
                } else if dest == "R" {
                    dest = "false".to_string();
                } else {
                    dest = format!("{f_prefix}_{dest}(_x, _m, _a, _s)");
                }

                if cond == "true" {
                    return format!("return {dest};");
                } else {
                    return format!("if _{cond} {{ return {dest}; }}");
                }
            })
            .collect::<Vec<String>>()
            .join(" ");

        return format!("fn {f_prefix}_{id}(_x: u32, _m: u32, _a: u32, _s: u32) -> bool {{ {rules} }}");
    }

    fn parse_part(p: Pair<'_, Rule>) -> String {
        format!("[{}]", p.into_inner().map(|it| it.into_inner().next().unwrap().as_str().to_string()).collect::<Vec<String>>().join(", "))
    }

    
    let contents = ["day19_ex", "day19"].map(|filename| {
        let input = fs::read_to_string(format!("input/{}", filename)).expect("Unable to read file");
        
        let mut hasher = DefaultHasher::new();
        input.hash(&mut hasher);
        let f_prefix = format!("_{:x}", hasher.finish());

        let mut program = Day19Parser::parse(Rule::Program, &input)
            .unwrap()
            .next()
            .unwrap()
            .into_inner();

        let workflows = program.next().unwrap().into_inner().map(|it| parse_workflow(&f_prefix, it)).collect::<Vec<String>>().join("\n");
        
        let parts = format!("fn {f_prefix}_get_parts() -> Vec<[u32; 4]> {{\n    Vec::from([\n        {}\n    ])\n}}", program.next().unwrap().into_inner().map(parse_part).collect::<Vec<String>>().join(",\n        "));
        
        
        return (
            format!("(\"{f_prefix}\".to_string(), ({f_prefix}_get_parts as (fn() -> Vec<[u32; 4]>), {f_prefix}_in as (fn(u32, u32, u32, u32) -> bool)))"),
            format!("/* auto-generated from: {filename} */\n{workflows}\n\n{parts}\n")
        );
    }).into_iter().collect::<Vec<(String, String)>>();
    
    let index = format!("use std::collections::HashMap;\npub fn get_index() -> HashMap<String, (fn() -> Vec<[u32; 4]>, fn(u32, u32, u32, u32) -> bool)> {{ return HashMap::from([{}])}}", contents.iter().map(|it| (&it.0).to_string()).into_iter().collect::<Vec<String>>().join(", "));
    
    let dest = "src/puzzles/day19/day19_workflows.rs";
    fs::write(&dest, format!("{index}\n\n{}", contents.into_iter().map(|it| (&it.1).to_string()).into_iter().collect::<Vec<String>>().join("\n\n"))).unwrap();
}

fn main() {
    write_day19_programs();
}