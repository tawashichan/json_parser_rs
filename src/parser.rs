use lexer::Token;
use ast::Json;
use std::collections::HashMap;


fn parse_list(tokens: &[Token]) -> Json {
    let (_,json) = parse_list_sub(tokens,&mut HashMap::new());
    json
}

fn parse_list_sub<'a>(tokens: &'a[Token],acm: &mut HashMap<String,Json>) -> (&'a[Token],Json) {
    match tokens {
        [Token::STRING(s),Token::COLON,rest..] => {
            let (res,ac) = parse_list_sub(rest,&mut HashMap::new());
            acm.insert(s.clone(),ac);
            parse_list_sub(res,acm)
        }
        [Token::RBRACE,rest..] => (rest,Json::Map(acm.to_owned())),
        [Token::COMMA, rest..] => parse_list_sub(rest,acm),
        [Token::INT(i),rest..] => (rest,Json::Int(*i)),
        [Token::STRING(s),rest..] => (rest,Json::String(s.to_owned())),
        [Token::FLOAT(f),rest..] => (rest,Json::Float(*f)),
        [Token::LBRACE,rest..] => parse_list_sub(rest,acm),
        [Token::NULL,rest..] => (rest,Json::Null),
        [Token::TRUE,rest..] => (rest,Json::TRUE),
        [Token::FALSE,rest..] => (rest,Json::FALSE),
        [Token::LBRACKET,rest..] => parse_bracket(rest,&mut vec![]),
        _ => (&[],Json::Map(acm.to_owned()))
    }
}

fn parse_bracket<'a>(tokens: &'a[Token],acm: & mut Vec<(Json)>) -> (&'a[Token],Json) {
    match tokens {
        [Token::RBRACKET,rest..] => (rest,Json::List(acm.to_owned())),
        _ => {
            let (res,ac) = parse_list_sub(tokens,&mut HashMap::new());
            acm.push(ac);
            parse_bracket(res,acm)
        }
    }
}


pub fn parse_tokens(tokens: Vec<Token>) -> Json {
    parse_list(&tokens)
}