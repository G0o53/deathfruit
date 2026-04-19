// TODO
/*
add more useful tools
*/

/*
This code is subject to the BSD-3-Clause license, 
if a copy of the license was not included with the software,
you may obtain one here: https://opensource.org/license/bsd-3-clause
*/

// I've decided to include a guide to the error and warn codes, so you know exactly what it means :) - G0o53
/* ERRORS
00 means "everything worked correctly, exiting now"
01 means "something broke internally"
02 means "unrecognised command"
09 means "not enough args"
*/

use std::env;
use std::io::Write;
use std::io;
use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();

    /* stdout lock */
    let stdout = std::io::stdout();
    let mut out = stdout.lock();    
    /* stdout lock */

    if args.len() < 2 {
        eprint!("\x1b[31m[ERROR] 09\x1b[0m\n");
        std::process::exit(09);
    }
    let command = &args[1];

    if command == "puts" {
        if args.len() > 2 {
            let str = &args[2];
            write!(out, "{str}").unwrap();
            
            std::process::exit(0);
        } else {
            eprint!("\x1b[31m[ERROR] 09\x1b[0m\n");
            std::process::exit(09);
            }
    } else if command == "&" {
        std::process::exit(0);
    } else if command == "gets" {
        if args.len() > 2 {
            let prmpt = &args[2];
            
            write!(out, "{prmpt}").unwrap();
            out.flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            
            print!("{input}");

            std::process::exit(0);
        } else {
            eprint!("\x1b[31m[ERROR] 09\x1b[0m\n");
            std::process::exit(09);
        }

    } else if command == "read" {
        if args.len() > 3 {
            let file = &args[2];
            let wlinen: &i64 = &args[3]
                .parse()
                .unwrap();
            let line = freadl(file, *wlinen);
            print!("{line}");
            std::process::exit(0);
        } else {
            print!("\x1b[31m[ERROR] 09\n");
            std::process::exit(09);
        }
    } else if command == "find" {
        if args.len() > 3 {
            let path = &args[2];
            let str: &String = &args[3]
                .parse()
                .unwrap();
            let res = readline(path, str.to_string());
            print!("{res}");
            std::process::exit(0);
        } else {
            print!("\x1b[31m[ERROR] 09\x1b[0m\n");
            std::process::exit(09);
        }
    } else if command == "version" {
        write!(out, "\x1b[32m[INFO] v0.0.8\x1b[0m\n").unwrap();
        std::process::exit(0);
    } else if command == "eval" {
        if args.len() > 3 {
            let x: &i64 = &args[2].parse().unwrap();
            let y: &i64 = &args[3].parse().unwrap();
            
            if x > y {
                write!(out, "{x}").unwrap();
                std::process::exit(0);
            } else if x < y {
                write!(out, "{y}").unwrap();
                std::process::exit(0);
            } else {
                write!(out, "=").unwrap();
                std::process::exit(0);
            }
        } else {
            eprint!("\x1b[31m[ERROR] 09\x1b[0m\n");
            std::process::exit(09);
        }
    } else if command == "cmp" {
        if args.len() > 3 {
            let x: &i64 = &args[2].parse().unwrap();
            let y: &i64 = &args[3].parse().unwrap();

            if x > y {
                write!(out, ">").unwrap();
                std::process::exit(0);
            } else if x < y {
                write!(out, "<").unwrap();
                std::process::exit(0);
            } else {
                write!(out, "=").unwrap();
                std::process::exit(0);
            }
        } else {
            eprint!("\x1b[31m[ERROR] 09\x1b[0m\n");
            std::process::exit(09);
        }
    } else if command == "min" {
        if args.len() > 3 {
            let x: &i64 = &args[2].parse().unwrap();
            let y: &i64 = &args[3].parse().unwrap();

            if x > y {
                write!(out, "{y}").unwrap();
                std::process::exit(0);
            } else if x < y {
                write!(out, "{x}").unwrap();
                std::process::exit(0);
            } else {
                write!(out, "=").unwrap();
                std::process::exit(0);
            }

        } else {
            eprint!("\x1b[31m[ERROR] 09\x1b[0m\n");
            std::process::exit(09);
        }
    } else if command == "chalk" {
        if args.len() > 4 {
            let x: &i64 = &args[2].parse().unwrap();
            let op = &args[3];
            let y: &i64 = &args[4].parse().unwrap();

            if op == "+" {
                write!(out, "{}\n", x+y).unwrap();
            } else if op == "-" {
                write!(out, "{}\n", x-y).unwrap();
            } else if op == "*" {
                write!(out, "{}\n", x*y).unwrap();
            } else if op == "/" {
                write!(out, "{}\n", x/y).unwrap();
            } else {
                write!(out, "\x1b[31m[ERROR] 02\x1b[0m").unwrap();
                std::process::exit(02);
            }
        }
    } else {
        eprint!("\x1b[31m[ERROR] 02\x1b[0m");
        std::process::exit(02);
    }
}

#[inline]
fn freadl(pathfile: &str, line: i64) -> String {
    let mut linen: i64 = 1;
    let mut lline = String::new();
    let file = File::open(pathfile).unwrap();
    let mut reader = BufReader::new(file);

    while reader.read_line(&mut lline).unwrap() > 0 {
        if linen < line {
            linen += 1;
            lline.clear();
        } else {
            return lline;
        }
    }
    let exit = String::new();
    exit
}

#[inline]
fn readline(pathfile: &str, line: String) -> String {
    let mut lline = String::new();
    let file = File::open(pathfile).unwrap();
    let mut reader = BufReader::new(file);

    while reader.read_line(&mut lline).unwrap() > 0 {
        if lline.contains(&line) {
            return lline;
        }
        lline.clear();
    }
    let exit = String::new();
    exit
}

