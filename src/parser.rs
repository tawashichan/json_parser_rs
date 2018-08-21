use lexer::Token;
use ast::Json;

//クソ遅いので、極力vecではなくslice,Stringではなく&strを使うように書き換える

fn parse_list(tokens: Vec<Token>) -> (Vec<Token>,Json) {
    parse_list_sub(tokens,vec![])
}

fn parse_list_sub(tokens: Vec<Token>,acm: Vec<(String,Json)>) -> (Vec<Token>,Json) {
    match &tokens[..] {
        [Token::STRING(s),Token::COLON,rest..] => {
            let (res,ac) = parse_list_sub(rest.to_vec(),vec![]);
            parse_list_sub(res,[acm,vec![(s.clone(),ac)]].concat().to_vec())
        }
        [Token::RBRACE,rest..] => (rest.to_vec(),Json::Assoc(acm)),
        [Token::COMMA, rest..] => parse_list_sub(rest.to_vec(),acm),
        [Token::INT(i),rest..] => (rest.to_vec(),Json::Int(*i)),
        [Token::STRING(s),rest..] => (rest.to_vec(),Json::String(s.clone())),
        [Token::FLOAT(f),rest..] => (rest.to_vec(),Json::Float(*f)),
        [Token::LBRACE,rest..] => parse_list_sub(rest.to_vec(),acm),
        [Token::LBRACKET,rest..] => parse_bracket(rest.to_vec(),vec![]),
        _ => (vec![],Json::Assoc(acm))
    }
}

fn parse_bracket(tokens: Vec<Token>,acm: Vec<(Json)>) -> (Vec<Token>,Json) {
    match &tokens[..] {
        [Token::RBRACKET,rest..] => (rest.to_vec(),Json::List(acm)),
        _ => {
            let (res,ac) = parse_list_sub(tokens.clone(),vec![]);
            parse_bracket(res,[vec![ac],acm].concat().to_vec())
        }
    }
}

pub fn parse_tokens(tokens: Vec<Token>) -> Json {
    let (re,acm) = parse_list(tokens);
    acm
}