extern crate phf_codegen;

use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

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
        let bytes = fs::read(path).unwrap();
        let content = String::from_utf8_lossy(&bytes);
        print!(
            "static DIVISIONS_{}: phf::Map<&'static str, &'static str> = ",
            code
        );
        let mut map = phf_codegen::Map::new();
        for line in content.lines().skip(1) {
            let parts: Vec<&str> = line.split('\t').collect();
            let code = parts[2];
            let division = parts[3];
            map.entry(code, &format!("\"{}\"", division));
        }
        print!("{}", map.build());
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
    let mca_codes =
        generate_by_dir(path.join("mca").as_path()).expect("generate from mca failed");
    let contrib_codes =
        generate_by_dir(path.join("contrib").as_path()).expect("generate from contrib failed");
    print!("pub static DIVISIONS: phf::Map<&'static str, &'static phf::Map<&'static str, &'static str>> = ");
    let mut map = phf_codegen::Map::new();
    for code in &mca_codes {
        let code_name = code.replace("-", "_");
        map.entry(&code[..], &format!("&DIVISIONS_{}", code_name));
    }
    for code in &contrib_codes {
        let code_name = code.replace("-", "_");
        map.entry(&code[..], &format!("&DIVISIONS_{}", code_name));
    }
    print!("{}", map.build());
    print!(";\n");
}
