use std::{
    collections::HashMap,
    env,
    error::Error,
    fs::File,
    io::{self, Read},
};
use string_template;

fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() -> Result<(), Box<Error>> {
    let filename = env::args().nth(1).unwrap();
    let content = read_file(&filename)?;

    let args = HashMap::new();
    let pairs = string_template::evaluate(&content, &args)?;
    println!("{:?}", pairs);

    Ok(())
}
