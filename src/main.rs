#![feature(slice_patterns)]
#![feature(test)]

extern crate test;
//extern crate json_macro;
//#[macro_use]
//extern crate json_macro_derive;

mod ast;
mod lexer;
mod parser;


//use json_macro::JsonMacro;

fn main() {

    let str = "{\"hoge\":{\"aaa\":12},\"jiro\": [1,2,3,4]}";
    let tokens = lexer::str_to_tokens(str);
    println!("{:?}",tokens);
    let json = parser::parse_tokens(tokens);
    println!("{:?}",json);
}
