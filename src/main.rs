/*
This code is subject to the BSD-3-Clause license, 
if a copy of the license was not included with the software,
you may obtain one here: https://opensource.org/license/bsd-3-clause
*/


// I've decided to include a guide to the error codes, so you know exactly what it means :) - G0o53
/* ERRORS
01 means "something broke internally"
02 means "unrecognised command"
09 means "not enough args"
*/

/* WARNS
01 means "not found, exiting gracefully"
*/

use std::env;
use std::io::Write;
use std::io;
use std::fs;
use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();

    /* stdout lock */
    let stdout = std::io::stdout();
    let mut out = stdout.lock();    
    /* stdout lock */

    if args.len() < 2 {
        eprint!("\x1b[31m[ERROR] 09\x1b[0m");
        std::process::exit(09);
    }
    let command = &args[1];

    if command == "puts" {
        if args.len() > 2 {
            let str = &args[2];
            write!(out, "{str}").unwrap();    
        } else {
            eprint!("\x1b[31m[ERROR] 09\x1b[0m");
            std::process::exit(09);
            }
    } else if command == "&" {
        return;
    } else if command == "gets" {
        if args.len() > 2 {
            let prmpt = &args[2];

            write!(out, "{prmpt}").unwrap();
            out.flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            
            print!("{input}");
        } else {
            eprint!("\x1b[31m[ERROR] 09\x1b[0m");
            std::process::exit(09);
        }
    } else if command == "write" {
        if args.len() > 3 {
            let file = &args[2];
            let str = &args[3];
            fwrite(file, str).unwrap();
            std::process::exit(01);
        } else {
            eprint!("\x1b[31m[ERROR] 09\x1b[0m");
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
            std::process::exit(01);
        } else {
            print!("\x1b[31m[ERROR] 09");
            std::process::exit(09);
        }
    } else {
        eprint!("\x1b[31m[ERROR] 02\x1b[0m");
        std::process::exit(02);
    }
}

fn fwrite(file: &str, str: &str) -> std::io::Result<()> {
    fs::write(file, str)?;
    Ok(())
}

fn freadl(pathfile: &str, line: i64) -> String {
    let mut linen: i64 = 1;
    let mut lline = String::new();
    let file = File::open(pathfile).unwrap();
    let mut  reader = BufReader::new(file);

    while reader.read_line(&mut lline).unwrap() > 0 {
        if linen < line {
            linen += 1;
            lline.clear();
        } else {
            return lline;
        }
    }
    let exit = String::from("\x1b[33m[WARN] 01\x1b[0m");
    exit
}
