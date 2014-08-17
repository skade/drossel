use strand::mutable::Event;
use strand::errors::{Errors};

pub trait AsEvent<V,S> {
  fn as_event<R>(self, fun: |a: Box<Event<V,S>>| -> R) -> R;
  fn as_sendable_event<R: Send>(self, fun: |a: Box<Event<V,S>+Send>| -> R) -> R;
}
