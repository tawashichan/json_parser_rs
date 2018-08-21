
#[derive(Debug,Clone)]
pub enum Json {
    Assoc(Vec<(String,Json)>),
    Int(i64),
    Float(f64),
    String(String),
    List(Vec<Json>),
    Null
}
