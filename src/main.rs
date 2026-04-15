/*
This code is subject to the BSD-3-Clause license, 
if a copy of the license was not included with the software,
you may obtain one here: https://opensource.org/license/bsd-3-clause
*/


// I've decided to include a guide to the error codes, so you know exactly what it means :) - G0o53
/*
01 means "something broke internally"
02 means "unrecognised command"
09 means "not enough args"
*/

use std::env;
use std::io::Write;
use std::io;

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
            std::process::exit(01);
        }
    } else if command == "reads" {
        
    } else {
        eprint!("\x1b[31m[ERROR] 02\x1b[0m");
        std::process::exit(02);
    }
}
