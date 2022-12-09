#![feature(let_chains)]
use std::env;
use std::io::{self, BufReader};
use std::fs;
use std::ptr::NonNull;

use lazy_static::lazy_static;
use regex::Regex;

use aoc::parser::parse_non_empty_line;

lazy_static! {
    static ref CMD_REGEX: Regex
         = Regex::new(r"\$ (?:cd (.*)|ls())|([0-9]+) (.*)|dir (.*)")
                 .unwrap();

    static ref MAX_DIR_SIZE: usize = 100_000;
    static ref TOTAL_SPACE: usize = 70_000_000 - 30_000_000;
}

enum LineInfo {
    Cd(String),
    Ls,
    Stat(usize, String),
    DirEntry(String)
}

#[derive(Debug)]
struct Directory {
    dirname: String,
    dentries: Vec<*mut Inode>,
    back: Option<NonNull<Inode>>
}

impl Directory {
    fn new(s: String) -> Self {
        Directory { dirname: s,
                    dentries: vec![],
                    back: None }
    }

}

#[derive(Debug)]
struct File {
    size: usize,
    name: String
}

impl File {
    fn new(sz: usize, name: String) -> Self {
        File { size: sz, name: name}
    }
}

#[derive(Debug)]
enum Inode {
    Dir(Directory),
    File(File)
}

impl Inode {
    fn root() -> Self {
        Inode::Dir(Directory::new("/".to_string()))
    }

    fn get_size(&self) -> usize {
        match self {
            Inode::Dir(dir) => {
                dir.dentries
                   .iter()
                   .map(|d| unsafe { d.as_ref().unwrap() }.get_size())
                   .sum::<usize>()
            },
            Inode::File(file) => {
                file.size
            }
        }
    }

    fn add(&mut self, n: Inode) {
        if let Inode::Dir(dir) = self {
            let ptr = Box::into_raw(Box::new(n));
            dir.dentries.push(ptr);
            if let Inode::Dir(d) = unsafe { ptr.as_mut().unwrap() } {
                unsafe {
                    d.back = Some(NonNull::new_unchecked(self as *mut Inode));
                }
            }
        } else {
            panic!("Add called on file inode")
        }
    }

    fn get_subdir(&mut self, selector: &String) -> &mut Self {
        if let Inode::Dir(dir) = self {
            for e in dir.dentries.iter().map(|p| unsafe{ (*p).as_mut().unwrap() }) {
                if let Inode::Dir(d) = e {
                    if &d.dirname == selector {
                        return e;
                    }
                }
            }
            panic!("Subdirectory not found!")
        } else {
            panic!("Cannot cd into file inode")
        }
    }

    fn get_enclosing_mut(&mut self) -> &mut Self {
        if let Inode::Dir(dir) = self {
            if let Some(b) = &mut dir.back {
                let back = unsafe { b.as_mut() };
                if matches!(back, Inode::Dir(_)) {
                    back
                } else {
                    unreachable!()
                }
            } else {
                panic!("No enclosing directory to root!")
            }
        } else {
            unimplemented!()
        }
    }
}

fn main() -> io::Result<()> {

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Incorrect number of args!")
    }

    let mut r = {
        let f = fs::File::open(&args[1])?;
        BufReader::new(f)
    };

    let mut v = vec![];

    /* parse symbols */
    while let Some(line) = parse_non_empty_line(&mut r) {
        let captures = CMD_REGEX.captures(&line).unwrap();

        let li = if let Some(cd) = captures.get(1) {
            LineInfo::Cd(cd.as_str().to_string())
        } else if let Some(_) = captures.get(2) {
            LineInfo::Ls
        } else if let Some(size) = captures.get(3)
               && let Some(file) = captures.get(4) {
            LineInfo::Stat(size.as_str().parse::<usize>().unwrap(),
                           file.as_str().to_string())
        } else if let Some(dir) = captures.get(5) {
            LineInfo::DirEntry(dir.as_str().to_string())
        } else {
            panic!("File didn't parse")
        };

        v.push(li);
    }

    let mut root = Inode::root();
    let mut cwd = &mut root;
    /* Create directories in discovery order */
    {
        // let mut wd = &mut t;
        for info in v {
            match info {
                LineInfo::Cd(dir) => {
                    println!("$ cd {dir}");
                    match dir.as_str() {
                        ".." => {
                            cwd = cwd.get_enclosing_mut();
                        },
                        "/"  => {},
                         _   => {
                            cwd = cwd.get_subdir(&dir);
                        }
                    }
                },
                LineInfo::Ls => {},
                LineInfo::Stat(sz, file) => {
                    cwd.add(Inode::File(File::new(sz, file)));
                },
                LineInfo::DirEntry(dir) => {
                    cwd.add(Inode::Dir(Directory::new(dir)));
                }
            }
        }
    }

    /* Finally, dfs the tree to find small directories */
    fn p1(s: &mut usize, n: &Inode) {
        let sz = n.get_size();
        println!("{:?}", n);
        println!("Got size: {sz}");
        if  matches!(n, Inode::Dir(_))
         && sz <= *MAX_DIR_SIZE {
            *s += sz;
        }
        if let Inode::Dir(dir) = n {
            for d in dir.dentries.iter() {
                let d_ref = unsafe { d.as_ref() }.unwrap();
                p1(s, d_ref);
            }
        }
    }

    let dir_sum = {
        let mut s = 0;
        p1(&mut s, &root);
        s
    };
    println!("Part one directory sum: {dir_sum}");

    fn p2(m: &mut Option<usize>, debt: usize, n: &Inode) {
        let sz = n.get_size();
        if  matches!(n, Inode::Dir(_))
         && sz < (*m).unwrap_or(usize::max_value())
         && sz >= debt {
             *m = Some(sz);
        }
        if let Inode::Dir(dir) = n {
            for d in dir.dentries.iter() {
                let d_ref = unsafe { d.as_ref() }.unwrap();
                p2(m, debt, d_ref);
            }
        }
    }

    let dir_size = {
        let mut min = None;
        println!("Need to source {} bytes:", root.get_size() - *TOTAL_SPACE);
        p2(&mut min, root.get_size() - *TOTAL_SPACE, &root);
        min.unwrap()
    };
    println!("Part two size of directory to delete: {dir_size}");

    Ok(())
}