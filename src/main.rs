use clap::{crate_authors, crate_version, App, Arg};
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub trait MatcherTrait {
    fn execute(&self, line: &str) -> bool;
}

pub struct FixedStringsMatcher {
    pattern: String,
}
impl FixedStringsMatcher {
    pub fn new(pattern: String) -> FixedStringsMatcher {
        FixedStringsMatcher { pattern: pattern }
    }
}
impl MatcherTrait for FixedStringsMatcher {
    fn execute(&self, line: &str) -> bool {
        line.contains(&self.pattern)
    }
}

fn main() {
    let matches = App::new("grep")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Search for PATTERNS in each FILE")
        .arg(
            Arg::with_name("fixed-strings")
                .short("F")
                .long("fixed-strings")
                .help("PATTERNS are strings"),
        )
        .arg(
            Arg::with_name("PATTERNS")
                .help("use PATTERNS for matching")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("FILES")
                .help("take PATTERNS from FILES")
                .required(true)
                .multiple(true)
                .index(2),
        )
        .get_matches();

    let pattern = matches.value_of("PATTERNS").unwrap();
    let file_pathes = matches
        .values_of("FILES")
        .unwrap()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    for file_path in file_pathes {
        let path = Path::new(&file_path);
        let display = path.display();

        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}, {}", why, display.to_string()),
            Ok(file) => file,
        };

        let mut s = String::new();

        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {} {}", why, display.to_string()),
            Ok(_) => {
                for line in s.lines() {
                    println!("{}", line);
                }
            }
        }
    }

    println!("{:?}", pattern);
}
