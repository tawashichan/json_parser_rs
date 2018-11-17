use lexer::Token;
use ast::Json;
use std::collections::HashMap;


pub fn parse_tokens(tokens: Vec<Token>) -> Json {
    parse_json(&tokens)
}

fn parse_json(tokens: &[Token]) -> Json {
    let (_,json) =  match tokens {
        [Token::LBRACE,_rest..] => parse_list(tokens),
        [Token::LBRACKET,_rest..] => parse_list(tokens),
        _ => panic!("invalid tokens")
    };
    json
}

fn parse_list<'a>(tokens: &'a[Token]) -> (&'a[Token],Json) {
    match tokens {
        [Token::LBRACE,rest..] => {
            let (res,json) = parse_object(rest,&mut HashMap::new());
            match res {
                [Token::RBRACE,re..] => {
                    (re,json)
                }
                _ => panic!("invalid token {:?}",res)
            }
        }
        [Token::LBRACKET,rest..] => {
            let (res,json) = parse_array(rest,&mut vec![]);
            match res {
                [Token::RBRACKET,re..] => {
                    (re,json)
                }
                _ => panic!("invalid token {:?}",res)
            }
        }
        [Token::NUMBER(n),rest..] => {
            (rest,Json::Number(*n))
        }
        [Token::STRING(s),rest..] => {
            (rest,Json::String(s.clone()))
        }
        [Token::NULL,rest..] => {
            (rest,Json::Null)
        }
        [Token::BOOLEAN(b),rest..] => {
            (rest,Json::Boolean(*b))
        }
        _ => panic!("invalid token")
    }
}

fn parse_object<'a>(tokens: &'a[Token],acm: &mut HashMap<String,Json>) -> (&'a[Token],Json) {
    match tokens {
        [Token::STRING(s),Token::COLON,rest..] => {
            let (res,json) = parse_list(rest);
            acm.insert(s.clone(),json);
            parse_object(res,acm)
        }
        _ =>  (tokens,Json::Object(acm.clone()))
    }
}

fn parse_array<'a>(tokens: &'a[Token],acm: &mut Vec<Json>) -> (&'a[Token],Json) {
    match tokens {
        [Token::RBRACKET,_rest..] => (tokens,Json::Array(acm.clone())),
        _ => {
            let (rest,json) = parse_list(tokens);
            acm.push(json);
            match rest {
                [Token::COMMA,res..] =>  parse_array(res,acm),
                _ => (rest,Json::Array(acm.clone()))
            }
        }
    }
}


#[test]
fn parse_obj(){
    let tokens = vec![Token::LBRACE,Token::STRING("hoge".to_string()),Token::COLON,Token::NUMBER(43.2),Token::RBRACE];
    let json = parse_tokens(tokens);
    let map = &mut HashMap::new();
    map.insert("hoge".to_string(),Json::Number(43.2));
    assert_eq!(json,Json::Object(map.clone()))
}

#[test]
fn parse_obj2(){
    let tokens = vec![Token::LBRACE,Token::STRING("hoge".to_string()),Token::COLON,Token::STRING("hoge".to_string()),Token::RBRACE];
    let json = parse_tokens(tokens);
    let map = &mut HashMap::new();
    map.insert("hoge".to_string(),Json::String("hoge".to_string()));
    assert_eq!(json,Json::Object(map.clone()))
}

#[test]
fn parse_obj3(){
    let tokens = vec![Token::LBRACE,Token::STRING("hoge".to_string()),Token::COLON,Token::NULL,Token::RBRACE];
    let json = parse_tokens(tokens);
    let map = &mut HashMap::new();
    map.insert("hoge".to_string(),Json::Null);
    assert_eq!(json,Json::Object(map.clone()))
}

#[test]
fn parse_obj4(){
    let tokens = vec![Token::LBRACE,Token::STRING("hoge".to_string()),Token::COLON,Token::BOOLEAN(true),Token::RBRACE];
    let json = parse_tokens(tokens);
    let map = &mut HashMap::new();
    map.insert("hoge".to_string(),Json::Boolean(true));
    assert_eq!(json,Json::Object(map.clone()))
}

#[test]
fn parse_obj5(){
    let tokens = vec![Token::LBRACE,Token::STRING("hoge".to_string()),Token::COLON,Token::BOOLEAN(false),Token::RBRACE];
    let json = parse_tokens(tokens);
    let map = &mut HashMap::new();
    map.insert("hoge".to_string(),Json::Boolean(false));
    assert_eq!(json,Json::Object(map.clone()))
}

#[test]
fn parse_obj6(){
    let tokens = vec![Token::LBRACE,Token::STRING("hoge".to_string()),Token::COLON,Token::LBRACKET,Token::BOOLEAN(false),Token::RBRACKET,Token::RBRACE];
    let json = parse_tokens(tokens);
    let map = &mut HashMap::new();
    map.insert("hoge".to_string(),Json::Array(vec![Json::Boolean(false)]));
    assert_eq!(json,Json::Object(map.clone()))
}