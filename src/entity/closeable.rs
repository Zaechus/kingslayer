pub trait Closeable {
    fn open(&mut self);

    fn close(&mut self);

    fn is_closed(&self) -> bool;
}
