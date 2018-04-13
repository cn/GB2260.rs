extern crate phf_codegen;

use std::env;
use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};

fn generate_by_dir(dir: &Path) -> io::Result<Vec<String>> {
    let mut codes = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            continue;
        }
        let code = path.file_stem().unwrap().to_string_lossy();
        codes.push(code.to_string());
        let code = code.replace("-", "_");
        let file = File::open(&path)?;
        let mut reader = BufReader::new(file);
        print!("static DIVISIONS_{}: phf::Map<&'static str, &'static str> = ", code);
        let mut map = phf_codegen::Map::new();
        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split('\t').collect();
            let code = parts[2];
            let division = parts[3];
            map.entry(code.to_string(), &format!("\"{}\"", division));
        }
        map.build(&mut io::stdout()).unwrap();
        println!(";\n");
    }
    Ok(codes)
}

fn main() {
    let data_dir = env::args().skip(1).next().expect("No data dir provided");
    println!("// Do not edit, this file is auto-generated.");
    println!("#![allow(non_upper_case_globals)]\n");
    println!("use phf;\n");
    let path: PathBuf = data_dir.into();
    let stats_codes = generate_by_dir(path.join("stats").as_path()).expect("generate from stats failed");
    let contrib_codes = generate_by_dir(path.join("contrib").as_path()).expect("generate from contrib failed");
    print!("static DIVISIONS: phf::Map<&'static str, &'static phf::Map<&'static str, &'static str>> = ");
    let mut map = phf_codegen::Map::new();
    for code in &stats_codes {
        let code_name = code.replace("-", "_");
        map.entry(&code[..], &format!("&DIVISIONS_{}", code_name));
    }
    for code in &contrib_codes {
        let code_name = code.replace("-", "_");
        map.entry(&code[..], &format!("&DIVISIONS_{}", code_name));
    }
    map.build(&mut io::stdout()).unwrap();
    print!(";\n");
}
