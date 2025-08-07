extern crate the_ray_tracer_challenge_rust as tracer;
use tracer::{canvas::Canvas, ppm};

pub mod ch10;
pub mod ch11;
pub mod ch4;
pub mod ch5;
pub mod ch6;
pub mod ch8;
pub mod ch9;
use crate::ch4::ch4;
use crate::ch5::ch5;
use crate::ch6::ch6;
use crate::ch8::ch8;
use crate::ch9::ch9;
use crate::ch10::ch10;
use crate::ch11::ch11;

use std::{collections::HashMap, env, fs, process};

fn print_usage(chapters: &[i32]) {
    eprintln!("Output a PPM image from chapters of the Ray Tracer Challenge\n");
    eprintln!("Usage: main [--output <file>] [--ch <chapter>]");
    eprintln!("  --output <file>   Write output to file instead of stdout");
    eprintln!(
        "  --ch <chapter>    Run a specific chapter ({})",
        chapters
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );
    eprintln!("                    Default: 10");
}

fn main() {
    let mut chapters: HashMap<_, fn() -> Canvas> = HashMap::new();
    chapters.insert(4, ch4);
    chapters.insert(5, ch5);
    chapters.insert(6, ch6);
    chapters.insert(8, ch8);
    chapters.insert(9, ch9);
    chapters.insert(10, ch10);
    chapters.insert(11, ch11);
    let mut valid_chapters = chapters.keys().cloned().collect::<Vec<i32>>();
    valid_chapters.sort();

    let mut output_file: Option<String> = None;
    let mut chapter = 11;

    let args: Vec<String> = env::args().collect();
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "--output" => {
                if i + 1 < args.len() {
                    output_file = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("Error: Missing output file name");
                    print_usage(&valid_chapters);
                    process::exit(1);
                }
            }
            "--ch" => {
                if i + 1 < args.len() {
                    chapter = args[i + 1].clone().parse::<i32>().unwrap_or_else(|_| {
                        eprintln!("Error: Invalid chapter number");
                        print_usage(&valid_chapters);
                        process::exit(1);
                    });
                    if !chapters.contains_key(&chapter) {
                        eprintln!("Error: Invalid chapter {chapter}");
                        print_usage(&valid_chapters);
                        process::exit(1);
                    }
                    i += 2;
                } else {
                    eprintln!("Error: Missing chapter number");
                    print_usage(&valid_chapters);
                    process::exit(1);
                }
            }
            "-h" | "--help" => {
                print_usage(&valid_chapters);
                process::exit(0);
            }
            _ => {
                eprintln!("Unknown argument: {}", args[i]);
                print_usage(&valid_chapters);
                process::exit(1);
            }
        }
    }

    let ch_func = chapters.get(&chapter).unwrap();
    let canvas = ch_func();
    let ppm = ppm::canvas_to_ppm(canvas);

    if output_file.is_some() {
        let file_name = output_file.unwrap();
        fs::write(&file_name, ppm).unwrap_or_else(|err| {
            eprintln!("Error writing to file {file_name}: {err}");
            process::exit(1);
        });
    } else {
        println!("{ppm}");
    }
}
