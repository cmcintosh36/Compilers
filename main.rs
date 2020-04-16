// This file serves as a driver for your compiler
// It should read a file (*.gpy) as a string, parse it into
// an AST, then compile it to assembly
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(unused_macros)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]
#![allow(non_snake_case)]

extern crate regex;
use std::fs;
use std::env;

#[allow(dead_code)]
mod lexer;
use lexer::{LexerState,Tok};

#[allow(dead_code)]
mod types;
use types::Instr::*;
use types::Val::*;

#[allow(dead_code)]
mod parser;
use parser::{parse};

#[allow(dead_code)]
mod compile;
use compile::{compile};

// This use can be removed when your parser is working
// main does not need to call it, it is just here for debug purposes
//use self::lexer::*;

fn main() -> Result<(), String> {
    let file = env::args().last().expect("cargo run file");
    let buf = fs::read_to_string(&file).expect(&format!("main: couldn't read {}", file));
    println!("tokens are:");
    let mut l = LexerState::new(&buf);
    loop {
        if let Some(tok) = l.next() {
            println!("{:?}", tok);            
            match tok {
                Tok::DOLLAR => break,
                _ => {}
            }
        } else { break }
    }

    println!("{}", "");
    let e = parse(&buf);
    let mut is: Vec<types::Instr> = Vec::new();
    is.push(types::Instr::SetFrame(0));
    is.push(types::Instr::Push(types::Val::Vloc(4)));
    is.push(types::Instr::Call);
    is.push(types::Instr::Halt);
    let mut instrs = compile(&e, &mut is);
    instrs.push(types::Instr::Ret);
    println!("instructions are: {:?}", instrs);
    Ok(())

}
