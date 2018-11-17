use std::collections::HashMap;

#[derive(Debug,Clone,PartialEq)]
pub enum Json {
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(HashMap<String,Json>),
    Null,
    Boolean(bool),
}
