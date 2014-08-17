#[deriving(PartialEq,Show,Send)]
pub enum DBResult {
  Pong,
  Inserted(String),
  Removed(String, Option<Vec<u8>>)
}