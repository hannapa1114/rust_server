pub struct Request {
    path: String,
    query_string: Option<String>,  // No value 에 대한 안전장치
    method: super::method::Method, // super parent module 참조할때
}
