use std::io::{BufReader, BufRead};
use std::fs::File;
use std::str::FromStr;

pub fn parse_non_empty_line(r: &mut BufReader<File>) -> Option<String> {
    let line;

    /* Get non-empty payload line */
    loop {
        line = {
            let mut l = String::new();
            let bytes = r.read_line(&mut l).unwrap();
            let l_prefix = l.trim_end();
            if bytes == 0 {
                return None;
            }
            if l_prefix.is_empty() {
                continue;
            }
            String::from_str(l_prefix).unwrap()
        };
        break;
    }

    Some(line)
}