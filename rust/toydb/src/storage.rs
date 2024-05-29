use std::io::{self};

// key/value engine for Storage engine
pub trait Engine: std::fmt::Display + Send + Sync {
    /// The iterator returned by scan(). Traits can't return "impl Trait". and we don't want to
    /// use trait objects, so the type must be specified.
    type ScanIterator<'a>: DoubleEndedIterator<Item = io::Result<(Vec<u8>, Vec<u8>)>> + 'a
    where
        Self: 'a;

    /// Deletes a key, or does nothing if it does not exist.
    fn delete(&mut self, key: &[u8]) -> io::Result<()>;

    /// Flushes any buffered data to the underlying storage medium.
    /// Write to storage via the `fsync`
    fn flush(&mut self) -> io::Result<()>;

    /// Gets a value for a key, if it exists.
    fn get(&mut self, key: &[u8]) -> io::Result<Option<Vec<u8>>>;

    /// Iterates over an ordered range of key/value pairs.
    fn scan<R: std::ops::RangeBounds<Vec<u8>>>(&mut self, range: R) -> Self::ScanIterator<'_>;

    /// Sets a value for a key, replacing the existing value if any.
    fn set(&mut self, key: &[u8], value: Vec<u8>) -> io::Result<()>;
}
