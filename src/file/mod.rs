use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_data(path: &str) -> Vec<(String, String)> {
    let mut vec: Vec<(String, String)> = Vec::new();
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(l) = line {
                if l.contains("##") {
                    continue;
                }
                if l.is_empty() {
                    continue;
                }

                if l.contains("_") {
                    let date: Vec<&str> = l.split("_").collect();
                    if date.len() > 2 {
                        let id = date.get(1).unwrap();
                        let name = date.get(2).unwrap();
                        vec.push((id.to_string(), name.to_string()));
                    } else {
                        let id = date.get(0).unwrap();
                        let name = date.get(1).unwrap();
                        vec.push((id.to_string(), name.to_string()));
                    }
                    continue;
                }

                if l.contains(" ") {
                    let date: Vec<&str> = l.split(" ").collect();
                    if date.len() == 2 {
                        let id = date.get(0).unwrap();
                        let name = date.get(1).unwrap();
                        vec.push((id.to_string(), name.to_string()));
                    } else if date.len() > 2 {
                        if date.first().unwrap().contains("-") {
                            let id = date.get(0).unwrap();
                            let count = id.len() + 1;
                            let name = &l[count..];
                            vec.push((id.to_string(), name.to_string()));
                        } else {
                            let actor = date.get(0).unwrap();
                            let id = date.get(1).unwrap();
                            let count = id.len() + actor.len() + 2;
                            let name = &l[count..];
                            vec.push((id.to_string(), name.to_string()));
                        }
                    }
                    continue;
                }
                vec.push((l, String::new()));
                // for s in l.split(" ") {
                //     if s.contains("-") {
                //         println!("{}", s);
                //     }
                // }
            }
        }
    }
    return vec;
}

pub fn read_data_v2(path: &str) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(l) = line {
                let date: Vec<&str> = l.split(" ").collect();
                vec.push(date.first().unwrap().to_string());
                println!("{}", date.first().unwrap());
            }
        }
    }
    return vec;
}

pub fn read_asmr_data(path: &str) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(l) = line {
                if l.contains("##") {
                    continue;
                }
                if l.is_empty() {
                    continue;
                }

                if l.contains("_") {
                    let date: Vec<&str> = l.split("_").collect();
                    for s in date {
                        if s.contains("RJ"){
                            vec.push(s.to_string());
                        }
                    }
                    continue;
                }

                
            }
        }
    }
    return vec;
}
