mod config;
use config::parse_to_json_format;

fn main() {
    test();
}

pub fn test() {
    let json_vec = parse_to_json_format();
    for i in json_vec {
        println!("{:#?}\n", i);
    }
}
