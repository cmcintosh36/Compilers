// This file is used to implement a parser
//
// It should convert a given string into an AST calling the lexer
// for the next token along the way. Using the first and follow sets
// covered in class to construct a recursive-descent parser will be the
// easiest way to do this. Before writing code in this file, you may find
// it helpful to write out the first and follow set table.
//
// You should construct a function for each of the below rules

/* GRAMMAR:
<prog> ::= <funlist> % <exp>
<funlist> ::= <fun> <funlist> | epsilon
<fun> ::= (fun <id> <paramlist> -> <ty> <exp>)
<paramlist> ::= ( <id> <ty> ) <paramlist> | epsilon
<ty> ::= i32 | bool | unit | ( array <ty> )
<id> ::= [a-zA-Z]+
<i32> ::= [0-9]+
<bool> ::= true | false
<uop> ::= neg
<biop> ::= + | - | * | / | == | <
<explist> ::= <exp> <explist> | epsilon
<exp> ::= <i32>
        | <bool>
        | tt
        | <id>
        | ( <uop> <exp> )
        | ( <biop> <exp> <exp> )
        | ( let <id> <exp> <exp> )
        | ( seq <exp> <exp> )
        | ( alloc <exp> <exp> ) // size init
        | ( set <exp> <exp> <exp> ) // array idx
        | ( get <exp> <exp> ) // array idx
        | ( cond <exp> <exp> <exp> )
        | ( funptr <id> )
        | ( call <exp> <explist> )
        | ( <id> <explist> )
        | ( print <exp> )
        | ( spawn <exp> )
 */


use lexer::{LexerState};
use lexer::Tok::*;
use types::*;
use types::Exp::*;
use types::Binop::*;
use types::Unop::*;
use types::Let::*;
use types::Seq::*;

macro_rules! parse_err {
        ( $l:expr, $err:expr ) => {
            Err(format!("{} at {}:{} in '{}'",
                        $err, $l.info.line_no, $l.info.col_no, $l.rest))
        };
    }

fn parse_prog(l: &mut LexerState) -> Result<Exp, String> {
        match l.peek().expect("Error: Program expected a token") {
                OPENPAREN -> {
                        parse_funlist(l);
                        l.eat(%);
                        parse_exp(l);
                        Ok(Program(FunList, FunExp))=;
                }
        }
}

fn parse_fun(l: &mut LexerState, res: &mut Exp) -> Exp {
        let tok = l.peek().expect("expected token");
        let mut res_final = res.clone();
        match tok {
            OPENPAREN => {
                l.next();
                res_final = parse_fun(l, res);
            },
            CLOSEPAREN => {
                l.next();
                res_final = parse_fun(l, res);
            },
            VAR_ID(s) => {
                l.next();
                res_final = parse_fun(l, res);
            },
            TYPE_I32 => {
                l.next();
                res_final = parse_fun(l, res);
            },
            RET_TYPE => {
                l.next();
                res_final = parse_fun(l, res);
            },
            _ => res_final = parse_exp(l, res),
        }
        res_final
    
    }
    
    fn parse_let(l: &mut LexerState, res: &mut Exp) -> Exp {
        let tok = l.peek().expect("expected tok");
        let mut res_final = res.clone();
        match tok {
            LET => {
                l.eat(tok);
                let tok = l.peek().expect("expected tok");
                match tok.clone() {
                    VAR_ID(s) => {
                        l.eat(tok);
                        let e1 = parse_exp(l, res);
                        let e2 = parse_exp(l, res);
                        res_final = ELet(Box::new(Letexp{op: Let_x, var: EVar(s.to_string()), e1: e1, e2: e2}))
                    },
                    _ => panic!("Error: Expected variable"),
                }
            },
            _ => panic!("Error: Unexpected token"),
        }
        res_final
    }
    
    fn parse_seq(l: &mut LexerState, res: &mut Exp) -> Exp {
        let tok = l.peek().expect("expected tok");
        let mut res_final = res.clone();
        match tok {
            SEQUENTIAL => {
                l.eat(tok);
                let e1 = parse_exp(l, res);
                let e2 = parse_exp(l, res);
                res_final = ESeq(Box::new(Seqexp{op: Seq_x, e1: e1, e2: e2}))
            },
            _ => panic!("Error: Unexpected token"),
        }
        res_final
    }
    
    fn parse_val(l: &mut LexerState, res: &mut Exp) -> Exp {
        let tok = l.peek().expect("expected tok");
        let mut res_final = res.clone();
        match tok.clone() {
            I32(i) => {
                l.eat(tok);
                res_final = EI32(i)
            },
            VAR_ID(s) => {
                l.eat(tok);
                res_final = EVar(s.to_string())
            }
            _ => panic!("Error: Unexpected token"),
        }
        res_final
    }
    
    fn parse_var(l: &mut LexerState, res: &mut Exp) -> Exp {
        let tok = l.peek().expect("expected tok");
        let mut res_final = res.clone();
        match tok.clone() {
            VAR_ID(s) => {
                l.eat(tok);
                res_final = EVar(s.to_string())
            }
            _ => panic!("Error: Unexpected token"),
        }
        res_final
    }
    
    fn parse_bool(l: &mut LexerState, res: &mut Exp) -> Exp {
        let tok = l.peek().expect("expected tok");
        let mut res_final = res.clone();
        match tok {
            TRUE => {
                l.eat(tok);
                res_final = EBool(true);
            },
            FALSE => {
                l.eat(tok);
                res_final = EBool(false);
            }
            _ => panic!("Error: Unexpected token"),
        }
        res_final
    }
    
    fn parse_unop(l: &mut LexerState, res: &mut Exp) -> Exp {
        let tok = l.peek().expect("expected token");
        let mut res_final = res.clone();
        match tok {
            NEGATIVE => {
                l.eat(tok);
                let e = parse_exp(l, res);
                res_final = EUnop(Box::new(Unexp{op: Neg, e: e}))
            },
            _ => panic!("Error: Unexpected token"),
        }
    
        res_final
    }
    
    
    fn parse_binop(l: &mut LexerState, res: &mut Exp) -> Exp {
        let tok = l.peek().expect("expected token");
        let mut res_final = res.clone();
        match tok {
            PLUS => {
                l.eat(tok);
                let e1 = parse_exp(l, res);
                let e2 = parse_exp(l, res);
                res_final = EBinop(Box::new(Binexp{op: BPlus, lhs: e1, rhs: e2}))
            }
            MINUS => {
                l.eat(tok);
                let e1 = parse_exp(l, res);
                let e2 = parse_exp(l, res);
                res_final = EBinop(Box::new(Binexp{op: BMinus, lhs: e1, rhs: e2}))
            }
            TIMES => {
                l.eat(tok);
                let e1 = parse_exp(l, res);
                let e2 = parse_exp(l, res);
                res_final = EBinop(Box::new(Binexp{op: BTimes, lhs: e1, rhs: e2}))
            }
            DIVIDE => {
                l.eat(tok);
                let e1 = parse_exp(l, res);
                let e2 = parse_exp(l, res);
                res_final = EBinop(Box::new(Binexp{op: BDivide, lhs: e1, rhs: e2}))
            }
            EQUAL => {
                l.eat(tok);
                let e1 = parse_exp(l, res);
                let e2 = parse_exp(l, res);
                res_final = EBinop(Box::new(Binexp{op: BEqual, lhs: e1, rhs: e2}))
            }
            LESSTHAN => {
                l.eat(tok);
                let e1 = parse_exp(l, res);
                let e2 = parse_exp(l, res);
                res_final = EBinop(Box::new(Binexp{op: BLess_Than, lhs: e1, rhs: e2}))
            },
            _ => panic!("Error: Unexpected binary operator"),
        }
        res_final
    }
    
    fn parse_exp(l: &mut LexerState, res: &mut Exp) -> Exp {
        let tok = l.peek().expect("expected token");
        let mut res_final = res.clone();
        match tok {
            OPENPAREN => {
                l.eat(tok);
                res_final = parse_exp(l, res);
            }
            CLOSEPAREN => {
                l.eat(tok);
                res_final = parse_exp(l, res);
            }
            VAR_ID(s) => {res_final = parse_var(l, res);},
            I32(n) => {res_final = parse_val(l, res);},
            PLUS => {res_final = parse_binop(l, res);},
            MINUS => {res_final = parse_binop(l, res);},
            TIMES => {res_final = parse_binop(l, res);},
            DIVIDE => {res_final = parse_binop(l, res);},
            NEGATIVE => {res_final = parse_unop(l, res);},
            TRUE => {res_final = parse_bool(l, res);},
            FALSE => {res_final = parse_bool(l, res);},
            LET => {res_final = parse_let(l, res);}
            SEQUENTIAL => {res_final = parse_seq(l, res);}
            _ => {}
        }
        res_final
    }
    
    //Programs
    //prog ::= fn1 fn2 ... fnM % e
    pub fn parse(s: &str) -> Exp {
        let mut res: Exp = EI32(0);
        let mut l = LexerState::new(s);
        loop{
            if let Some(tok) = l.next(){
                if tok == FUNCTION {res = parse_fun(&mut l, &mut res);}
                else if tok == PERCENT {break;}
            }
            else{break}
        }
        let res2 = parse_exp(&mut l, &mut res);
        res2
    }
