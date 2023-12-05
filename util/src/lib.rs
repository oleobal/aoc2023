use proc_macro::TokenStream;

// code inspired by https://stackoverflow.com/a/54351072

/// register_puzzle!(4,1) for the first puzzle of day 4
#[proc_macro]
pub fn register_puzzle(input: TokenStream) -> TokenStream {
    let tokens = input.into_iter().collect::<Vec<_>>();

    let day = tokens[0].to_string().parse::<usize>().unwrap();
    let star = tokens[2].to_string().parse::<usize>().unwrap();

    return format!("puzzles.insert(\"{day}-{star}\".to_string(), day{day}::p{star});")
        .parse::<TokenStream>()
        .unwrap();
}

/// register_puzzles_for_days(5): register all puzzles up to day 5
#[proc_macro]
pub fn register_puzzles_for_days(input: TokenStream) -> TokenStream {
    let tokens = input.into_iter().collect::<Vec<_>>();

    let count = tokens[0].to_string().parse::<usize>().unwrap();

    return (1..count+1)
    .map(|day| {
        format!("puzzles.insert(\"{day}-1\".to_string(), day{day}::p1); puzzles.insert(\"{day}-2\".to_string(), day{day}::p2);").parse::<TokenStream>().unwrap()
    })
    .collect();
}

#[proc_macro]
pub fn publish_puzzles_for_days(input: TokenStream) -> TokenStream {
    let tokens = input.into_iter().collect::<Vec<_>>();

    let count = tokens[0].to_string().parse::<usize>().unwrap();

    return (1..count+1)
    .map(|day| {
        format!("pub mod day{day};").parse::<TokenStream>().unwrap()
    })
    .collect();
}