pub mod method;
pub mod params;
pub mod request;
pub mod response;
pub mod types;

#[derive(Debug)]
pub enum Error {
    InvalidParam,
    MethodNotFound,
    ResponseParsing,
    RawResponseParsing(String),
    ResponseId(usize),
    BatchParsing,
    WrongMethod,
}
