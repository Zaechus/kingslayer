pub trait Closeable {
    fn open(&mut self);

    fn close(&mut self);
}
