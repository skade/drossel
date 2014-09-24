use leveldb::database::Database;
use leveldb::database::binary::Interface;
use leveldb::database::error::Error;
use leveldb::database::comparator::Comparator;
use leveldb::database::iterator::Iterable;
use leveldb::options::{Options,WriteOptions,ReadOptions};

static comparator: KeyComparator<'static> = KeyComparator { name: "drossel_key_comparator" };

#[deriving(Show,PartialEq,Eq,PartialOrd,Ord,Clone)]
#[repr(u64)]
pub enum KeyType {
  Queue,
  Chunk
}

#[deriving(Show,PartialEq)]
pub type Id = u64;

#[deriving(Show,PartialEq,Clone)]
pub struct Key {
  id: Id,
  keytype: KeyType,
}

struct KeyComparator<'a> {
  name: &'a str
}

//impl<'a> KeyComparator<'a> {
//  fn new(name: &'a str) -> KeyComparator<'a> {
//    KeyComparator { name: name }
//  }
//}

impl<'a> Comparator for KeyComparator<'a> {
  fn name(&self) -> *const u8 { self.name.as_ptr() }

  fn compare(&self, a: &[u8], b: &[u8]) -> Ordering {
    Key::from_u8(a).compare(Key::from_u8(b))
  }
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

  pub fn compare(&self, other: &Key) -> Ordering {
    if self.keytype < other.keytype {
      return Less
    }
    if self.keytype > other.keytype {
      return Greater
    }
    if self.id < other.id {
      return Less
    }
    if self.id > other.id {
      return Greater
    }
    Equal
  }
}

pub struct Journal {
  db: Database,
  head: Key, // The key that points to the last value written
  tail: Key, // The key that points to the earliest value written, but not read
  reserved_tail: Key // The key that points to the beginning of the reserved block
}

impl Journal {
  fn new(path: Path) -> Result<Journal, Error> {
    let mut options = Options::new();
    options.create_if_missing(true);
    options.set_comparator(box comparator);
    let db = Database::open(path, options);
    let head = Key { keytype: Queue, id: 0 };
    let tail = Key { keytype: Queue, id: 0 };
    let reserved_tail = Key { keytype: Queue, id: 0 };
    match db {
      Ok(new) => Ok(Journal { db: new, head: head, tail: tail, reserved_tail: reserved_tail }),
      Err(e) => Err(e)
    }
  }

  fn open_existing(path: Path) -> Result<Journal,Error> {
    let mut options = Options::new();
    options.create_if_missing(false);
    options.set_comparator(box comparator);
    let db = Database::open(path, options);
    match db {
      Ok(mut existing) => {
        let (head, tail, reserved_tail) = Journal::read_keys(&mut existing);
        Ok(Journal { db: existing, head: head, tail: tail, reserved_tail: reserved_tail })
      },
      Err(e) => Err(e)
    }
  }

  fn read_keys(db: &mut Database) -> (Key, Key, Key) {
    let read_options = ReadOptions::new();
    let mut iter = db.iter(read_options);
    let reserved_tail = Key { keytype: Queue, id: 0 };
    if !iter.valid() {
      // we have a db, but no keys in it
      let queue_head = Key { keytype: Queue, id: 0 };
      let queue_tail = Key { keytype: Queue, id: 0 };
      return (queue_head, queue_tail, reserved_tail)
    }
    let first = iter.next().unwrap().key();
    let tail = Key::from_u8(first.as_slice());
    if !iter.valid() {
      // we have a db, with only one key. That key is head and tail.
      return (tail.clone(), tail.clone(), reserved_tail)
    }
    let last = iter.last().unwrap().key();
    let head = Key::from_u8(last.as_slice());
    (head.clone(), tail.clone(), reserved_tail)
  }

  pub fn open(path: Path) -> Result<Journal,Error> {
    let res = Journal::open_existing(path.clone());
    match res {
      Ok(j) => Ok(j),
      Err(_) => {
        Journal::new(path)
      }
    }
  }

  pub fn push(&mut self, data: &[u8]) {
    self.head.as_slice(|key| {
      let mut write_options = WriteOptions::new();
      write_options.sync(true);
      self.db.put(write_options, key, data).unwrap_or_else(|err| {
        fail!("error writing to journal: {}", err)
      })
    });

    self.head.id = self.head.id + 1;
  }

  pub fn pop(&mut self, remove: bool) -> Option<Vec<u8>> {
    if self.head.id >= self.tail.id {
      let res = self.tail.as_slice(|key| {
        let read_options = ReadOptions::new();
        let result = self.db.get(read_options, key).unwrap_or_else(|err| {
          fail!("error reading from journal: {}", err)
        });
        result
      });
      if remove {
        self.remove(false);
      }
      if res.is_some() {
        self.tail.id = self.tail.id + 1;
      }
      return res;
    } else {
      None
    }
  }

  fn remove(&mut self, reserved: bool) {
    let key = if reserved {
                self.tail
              } else {
                self.reserved_tail
              };
    key.as_slice(|raw| {
      let mut write_options = WriteOptions::new();
      write_options.sync(true);
      self.db.delete(write_options, raw).unwrap_or_else(|err| {
        fail!("error reading from journal: {}", err)
      })
    });
    if reserved {
      self.advance_to_next_reserved();
    }
  }

  fn advance_to_next_reserved(&mut self) {
    let read_options = ReadOptions::new();
    let mut iter = self.db.iter(read_options);
    let next_item = iter.next();

    match next_item {
      Some(entry) => {
        let binary_key = entry.key();
        let next_key = Key::from_u8(binary_key.as_slice());
        self.reserved_tail = next_key.clone();
      },
      None => {}
    }
  }

  pub fn len(&self) -> u64 {
    self.head.id - self.tail.id
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
    assert_eq!(Less, key.compare(&key2));
    assert_eq!(Greater, key2.compare(&key));
    assert_eq!(Less, key.compare(&key3));
    assert_eq!(Greater, key3.compare(&key));
    assert_eq!(Equal, key.compare(&key));
  }

  #[test]
  fn test_journal() {
    let dir = TempDir::new("journal_test").unwrap();
    let mut journal = Journal::open(dir.path().join("journal_test")).unwrap();
    let res = journal.pop(true);
    assert!(res.is_none());
    journal.push(&[1u8]);
    journal.push(&[2u8]);
    let res2 = journal.pop(true);
    assert!(res2.is_some());
    assert_eq!(Some(vec![1 as u8]), res2);
    let res3 = journal.pop(true);
    assert!(res3.is_some());
    assert_eq!(Some(vec![2 as u8]), res3);
    let res4 = journal.pop(true);
    assert!(res4.is_none());
    assert_eq!(0, journal.len());
  }
}
