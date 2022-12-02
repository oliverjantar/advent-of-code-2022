use std::{
    fs::File,
    io::{self, prelude::*, BufReader},
    path::Path,
    rc::Rc,
};

pub struct FileReader {
    reader: io::BufReader<File>,
    buf: Rc<String>,
    buf_capacity: usize,
}

impl FileReader {
    pub fn open(path: impl AsRef<Path>, buf_capacity: usize) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let buf = Rc::new(String::with_capacity(buf_capacity));

        Ok(Self {
            reader,
            buf,
            buf_capacity,
        })
    }
}

impl Iterator for FileReader {
    type Item = io::Result<Rc<String>>;

    fn next(&mut self) -> Option<Self::Item> {
        let buf = match Rc::get_mut(&mut self.buf) {
            Some(buf) => {
                buf.clear();
                buf
            }
            None => {
                self.buf = Rc::new(String::with_capacity(self.buf_capacity));
                Rc::make_mut(&mut self.buf)
            }
        };

        self.reader
            .read_line(buf)
            .map(|u| {
                if u == 0 {
                    None
                } else {
                    Some(Rc::clone(&self.buf))
                }
            })
            .transpose()
    }
}
