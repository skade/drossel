use strand::mutable::Event;
use strand::errors::{Errors};

pub trait AsEvent<V,S> {
  fn as_event<'a>(self, fun: |a: &Event<V,S>| -> Result<S, Errors>) -> Result<S, Errors>;
}