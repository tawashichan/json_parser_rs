use std::collections::HashMap;

#[derive(Debug,Clone)]
pub enum Json {
    Int(i64),
    Float(f64),
    String(String),
    List(Vec<Json>),
    Object(HashMap<String,Json>),
    Null,
    Boolean(bool),
}
