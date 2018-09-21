#![feature(slice_patterns)]
#![feature(test)]

extern crate test;

mod ast;
mod lexer;
mod parser;
mod data;

use ast::Json;
use test::Bencher;

#[bench]
fn test_json(b: &mut Bencher) {
    b.iter(||
        {
            let long_str = data::test_long_str();
            let tokens = lexer::str_to_tokens(long_str);
            let json = parser::parse_tokens(tokens);
        }
    );
}


fn main() {


    let str = "{\"hoge\": \"\",\"tawashi\": {\"aaa\":144.5,\"poyoyo\": [[[[6483]]],[42]]}}".to_string();
    //let super_long_str = data::test_long_str();
    //let tokens = lexer::str_to_tokens(data::test_long_str());
    let tokens = lexer::str_to_tokens(str);
    let json = parser::parse_tokens(tokens);
    println!("{:?}", json);

    //let hoge : Box<[u32]> = Box::new([1,2,3]);
    //println!("{:?}",hoge.into_vec());
}
