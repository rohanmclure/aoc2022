use lazy_static::lazy_static;

use aoc::parser::parse_non_empty_line;

struct Sensor {
    pos: (usize, usize),
    beacon_pos: (usize, usize)
}

lazy_static! {
    static ref SENSOR_REGEX: Regex = r"Sensor at x=([0-9]+), y=([0-9]+): closest beacon is at x=([0-9]+), y=([0-9]+)".unwrap();
}

fn parse_sensor_data(line: &str) -> Sensor {
    
}

fn main() {

    let mut r = {
        let f = fs::File::open(&args[1])?;
        BufReader::new(f)
    };
    
    while let Some(line) = parse_non_empty_line(&mut r) {
        
    }
}
