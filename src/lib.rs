use std::io;

pub fn ioerr(text : &str) -> io::Error {
    io::Error::new(io::ErrorKind::Other, text)
}


