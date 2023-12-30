use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

mod day19_workflows;

use day19_workflows::get_index;


pub fn p1(input: String) {
    // the workflows are generated at build time (not very pretty impl but conceptually I think it's fine)
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    let f_prefix = format!("_{:x}", hasher.finish());
    
    let index = get_index();
    
    let (get_parts, f_in) = index[&f_prefix];
    
    println!("{}", get_parts().into_iter().map(|part| {
        if f_in(part[0], part[1], part[2], part[3]) {
            (part[0] + part[1] + part[2] + part[3]) as u64
        }
        else { 0 }
    }).sum::<u64>());
}