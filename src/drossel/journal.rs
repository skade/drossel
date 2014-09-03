#[deriving(Show,PartialEq)]
pub enum KeyType {
  Queue,
  Chunk
}

#[deriving(Show,PartialEq)]
pub type Id = u64;

#[deriving(Show,PartialEq)]
pub struct Key {
  id: Id,
  keytype: KeyType,
}

impl Key {
  pub fn empty() -> Key {
    Key { keytype: Queue, id: 0 }
  }

  pub fn new(keytype: KeyType, id: Id) -> Key {
    Key { keytype: keytype, id: id }
  }

  pub fn from_u8(key: &[u8]) -> &Key {
    use std::mem::transmute;

    assert!(key.len() == 16)

    unsafe { transmute(key.as_ptr()) }
  }

  pub fn as_slice<T>(self, f: |v: &[u8]| -> T) -> T {
    use std::mem::transmute;

    unsafe { f(transmute::<_, [u8, ..16]>(self)) }
  }
}

#[cfg(test)]
mod tests {
  use drossel::journal::{Key,Queue};

  #[test]
  fn test_roundtrip() {
    let key = Key { keytype: Queue, id: 123 };
    key.as_slice(|bin| {
      let roundtrip = Key::from_u8(bin);
      assert_eq!(roundtrip.id, 123);
      assert_eq!(roundtrip.keytype, Queue);
    });
  }
}
