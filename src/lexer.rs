


#[derive(Clone,Debug)]
pub enum Token {
    STRING(String),
    INT(i64),
    FLOAT(f64),
    LBRACE,
    RBRACE,
    LBRACKET,
    RBRACKET,
    COLON,
    COMMA,
    NULL,
    BOOLEAN(bool),
    EOF
}

pub fn split_string(s: String) -> Vec<char> {
    s.chars().collect()
}


fn get_str(str_vec: &[char]) -> (String, &[char]) {
    get_str_sub(str_vec, "".to_string())
}

fn get_str_sub(str_vec: &[char],acm: String) -> (String,&[char]) {
    match str_vec {
        [first,rest..] => match first {
            '\"' => (acm,rest),
            _c => get_str_sub(rest,format!("{}{}",acm,first))
        }
        &[] => (acm,&[]),
    }
}

fn get_keyword(str_vec: &[char]) -> (Token, &[char]){
    get_keyword_sub(str_vec,"".to_string())
}

fn get_keyword_sub(str_vec: &[char], acm: String) -> (Token, &[char]) {
    match str_vec {
        [first, rest..] => if first.is_alphabetic() {
            get_keyword_sub(rest, format!("{}{}", acm, first))
        } else {
            match &*acm {
                "null" => (Token::NULL, str_vec),
                "true" => (Token::BOOLEAN(true), str_vec),
                "false" => (Token::BOOLEAN(false), str_vec),
                _ => panic!("invalid keyword: {:?}",acm)
            }
        }
        &[] => panic!("invalid tokens")//(acm,&[]),
    }
}

fn get_num_str(str_vec: &[char]) -> (String, &[char],bool) {
    get_num_str_sub(str_vec,"".to_string(),false)
}

//jsonの仕様的にintとfloatの区別は存在しないっぽいので
fn get_num_str_sub(str_vec: &[char], acm: String,is_float: bool) -> (String, &[char],bool) {
    match &str_vec[..] {
        [first,rest..] => {
            if first.is_numeric() {
                get_num_str_sub(rest, format!("{}{}",acm,first),is_float)
            } else if *first == '.' {
                get_num_str_sub(rest,format!("{}{}",acm,first),true)
            } else {
                (acm,str_vec,is_float)
            }
        }
        &[] => (acm, &[],is_float)
    }
}

//nightlyじゃないとvectorを分解できない...
fn next_token(slice: &[char]) -> (Token, &[char]) {
    match slice {
        [first, rest..] => match first {
            ' ' => next_token(rest),
            '{' => (Token::LBRACE, rest),
            '}' => (Token::RBRACE, rest),
            '[' => (Token::LBRACKET, rest),
            ']' => (Token::RBRACKET, rest),
            ':' => (Token::COLON, rest),
            ',' => (Token::COMMA, rest),
            c =>
                if c.is_numeric() {
                    let (num_str, re,is_float) = get_num_str(slice); //moveもmutableな参照もしてないからここでslice使える
                    if is_float {
                        let num = num_str.parse::<f64>().unwrap();
                        (Token::FLOAT(num), re)
                    } else {
                        let num = num_str.parse::<i64>().unwrap();
                        (Token::INT(num), re)
                    }
                } else if *c == '\"' {
                    let (s, re) = get_str(rest);
                    (Token::STRING(s),re)
                } else {
                   get_keyword(slice)
                }
        },
        [] => (Token::EOF, &[])
    }
}

fn get_tokens<'a>(slice: &[char],acm: &'a mut Vec<Token>) -> &'a Vec<Token> {
    match next_token(slice) {
        (Token::EOF,_) => acm,
        (token,slice) => {
            acm.push(token);
            get_tokens(slice,acm)
        },
    }

    //stack over flow避けるなら下の書き方になるが...
    /*let mut s = slice;
    while s.len() > 0 {
        match next_token(s) {
            (Token::EOF,_) => (),
            (token,slice) => {
                s = slice;
                acm.push(token);
            },
            (token,_) => ()
        }
    }
    acm*/
}

pub fn str_to_tokens<'a>(str: String) -> Vec<Token> {
    let str_vec = split_string(str);
    get_tokens(&str_vec,&mut vec![]).to_owned()
}

