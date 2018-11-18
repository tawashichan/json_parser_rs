use lexer::Token;
use ast::Json;
use std::collections::HashMap;


pub fn parse_tokens(tokens: Vec<Token>) -> Json {
    parse_json(&tokens)
}

fn parse_json(tokens: &[Token]) -> Json {
    let (rest,json) =  match tokens {
        [Token::LBRACE,_rest..] => parse_value(tokens),
        [Token::LBRACKET,_rest..] => parse_value(tokens),
        _ => panic!("invalid tokens")
    };
    if rest.len() > 0 {
        panic!("invalid tokens {:?}",rest)
    }
    json
}

fn parse_value<'a>(tokens: &'a[Token]) -> (&'a[Token],Json) {
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
            let (res,json) = parse_value(rest);
            acm.insert(s.clone(),json);
            parse_object(res,acm)
        }
        [Token::COMMA,rest..] => {
            parse_object(rest,acm)
        }
        _ =>  (tokens,Json::Object(acm.clone()))
    }
}

fn parse_array<'a>(tokens: &'a[Token],acm: &mut Vec<Json>) -> (&'a[Token],Json) {
    match tokens {
        [Token::RBRACKET,_rest..] => (tokens,Json::Array(acm.clone())),
        _ => {
            let (rest,json) = parse_value(tokens);
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

#[test]
fn parse_obj7(){
    let tokens = vec![Token::LBRACE,Token::STRING("hoge".to_string()),Token::COLON,Token::LBRACKET,Token::BOOLEAN(false),Token::COMMA,Token::BOOLEAN(true),Token::RBRACKET,Token::RBRACE];
    let json = parse_tokens(tokens);
    let map = &mut HashMap::new();
    map.insert("hoge".to_string(),Json::Array(vec![Json::Boolean(false),Json::Boolean(true)]));
    assert_eq!(json,Json::Object(map.clone()))
}

#[test]
fn parse_obj8(){
    let tokens = vec![Token::LBRACE,Token::STRING("hoge".to_string()),Token::COLON,Token::LBRACE,Token::STRING("string".to_string()),Token::COLON,Token::BOOLEAN(true),Token::RBRACE,Token::RBRACE];
    let json = parse_tokens(tokens);
    let map = &mut HashMap::new();
    let map2 = &mut HashMap::new();
    map2.insert("string".to_string(),Json::Boolean(true)); 
    map.insert("hoge".to_string(),Json::Object(map2.clone()));
    assert_eq!(json,Json::Object(map.clone()))
}

#[test]
fn parse_obj9(){
    let tokens = vec![Token::LBRACE,Token::STRING("hoge".to_string()),Token::COLON,Token::STRING("string".to_string()),Token::COMMA,Token::STRING("jiro".to_string()),Token::COLON,Token::BOOLEAN(true),Token::RBRACE];
    let json = parse_tokens(tokens);
    let map = &mut HashMap::new();
    map.insert("jiro".to_string(),Json::Boolean(true));
    map.insert("hoge".to_string(),Json::String("string".to_string()));
    assert_eq!(json,Json::Object(map.clone()))
}

