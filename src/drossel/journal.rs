use leveldb::database::Database;
use leveldb::database::binary::Interface;
use leveldb::options::{Options,WriteOptions,ReadOptions};

#[deriving(Show,PartialEq,Eq,PartialOrd,Ord)]
#[repr(u64)]
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

  pub fn compare(&self, other: &Key) -> i32 {
    if self.keytype < other.keytype {
      return -1
    }
    if self.keytype > other.keytype {
      return 1
    }
    if self.id < other.id {
      return -1
    }
    if self.id > other.id {
      return 1
    }
    0
  }
}

pub struct Journal {
  db: Database,
  queue_head: Key,
  queue_tail: Key,
}

impl Journal {
  pub fn new(path: Path) -> Journal {
    let mut options = Options::new();
    options.create_if_missing(true);
    let db = Database::open(path, options).unwrap();
    let queue_head = Key { keytype: Queue, id: 0 };
    let queue_tail = Key { keytype: Queue, id: 0 };
    Journal { db: db, queue_head: queue_head, queue_tail: queue_tail }
  }

  pub fn push(&mut self, data: &[u8]) {
    self.queue_head.as_slice(|key| {
      let mut write_options = WriteOptions::new();
      write_options.sync(true);
      self.db.put(write_options, key, data).unwrap_or_else(|err| {
        fail!("error writing to journal: {}", err)
      })
    });

    self.queue_head.id = self.queue_head.id + 1;
  }

  pub fn pop(&mut self) -> Option<Vec<u8>> {
    if self.queue_head.id > self.queue_tail.id {
      let res = self.queue_tail.as_slice(|key| {
        let read_options = ReadOptions::new();
        self.db.get(read_options, key).unwrap_or_else(|err| {
          fail!("error reading from journal: {}", err)
        })
      });
      self.queue_tail.id = self.queue_tail.id + 1;
      return res;
    } else {
      None
    }
  }

  pub fn len(&self) -> u64 {
    self.queue_head.id - self.queue_tail.id
  }
}

#[cfg(test)]
mod tests {
  use drossel::journal::{Key,Queue,Chunk,Journal};
  use std::io::TempDir;

  #[test]
  fn test_roundtrip() {
    let key = Key { keytype: Queue, id: 123 };
    key.as_slice(|bin| {
      let roundtrip = Key::from_u8(bin);
      assert_eq!(roundtrip.id, 123);
      assert_eq!(roundtrip.keytype, Queue);
    });
  }

  #[test]
  fn test_compare() {
    let key = Key { keytype: Queue, id: 123 };
    let key2 = Key { keytype: Chunk, id: 123 };
    let key3 = Key { keytype: Queue, id: 124 };
    assert_eq!(-1, key.compare(&key2));
    assert_eq!(1, key2.compare(&key));
    assert_eq!(-1, key.compare(&key3));
    assert_eq!(1, key3.compare(&key));
    assert_eq!(0, key.compare(&key));
  }

  #[test]
  fn test_journal() {
    let dir = TempDir::new("journal_test").unwrap();
    let mut journal = Journal::new(dir.path().join("journal_test"));
    let res = journal.pop();
    assert!(res.is_none());
    journal.push(&[1]);
    journal.push(&[2]);
    let res2 = journal.pop();
    assert!(res2.is_some());
    assert_eq!(Some(vec![1 as u8]), res2);
    let res3 = journal.pop();
    assert!(res3.is_some());
    assert_eq!(Some(vec![2 as u8]), res3);
    let res4 = journal.pop();
    assert!(res4.is_none());
  }
}
