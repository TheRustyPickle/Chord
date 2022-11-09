pub mod parser;

use parser::*;

fn main() {
    let data = "-cat category name -p -r admin, team member -ch channel-1 channel-2 channel-3".to_string();
    println!("{:?}", parse_input(data));
}
