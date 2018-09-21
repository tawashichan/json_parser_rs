#![feature(slice_patterns)]
#![feature(test)]

extern crate test;

mod ast;
mod lexer;
mod parser;
#[macro_use]
mod json_macro;




fn main() {


    let str = "{\"aaaa\": true,\"hoge\": null,\"tawashi\": {\"aaa\":144.5,\"poyoyo\": [[[[6483]]],[42]]}}".to_string();
    //let super_long_str = data::test_long_str();
    //let tokens = lexer::str_to_tokens(data::test_long_str());
    let tokens = lexer::str_to_tokens(str);
    println!("{:?}",tokens);
    let json = parser::parse_tokens(tokens);
    println!("{:?}", json);

    my_macro! {
        struct S {
            hoge: String,
            b: String,
        }
    };
    println!("{:?}",S::get_field_names());

    //let hoge : Box<[u32]> = Box::new([1,2,3]);
    //println!("{:?}",hoge.into_vec());
}
