use std::{collections::HashMap, error::Error};

mod content;
mod group;

pub fn evaluate(content: &str, _args: &HashMap<String, String>) -> Result<String, Box<Error>> {
    let group_pairs = group::parse(content);
    println!("{:?}", group_pairs);
    Ok(String::new())
}
