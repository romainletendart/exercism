use std::{borrow::Borrow, ops::BitXor};

#[cfg(feature = "io")]
use std::io::{Read, Write};

/// A munger which XORs a key with some data
#[derive(Clone)]
pub struct Xorcism<'a> {
    key: &'a [u8],
    offset: usize,
}

impl<'a> Xorcism<'a> {
    /// Create a new Xorcism munger from a key
    ///
    /// Should accept anything which has a cheap conversion to a byte slice.
    pub fn new<Key: AsRef<[u8]> + ?Sized>(key: &'a Key) -> Xorcism<'a> {
        Self {
            key: key.as_ref(),
            offset: 0,
        }
    }

    /// XOR each byte of the input buffer with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        data.iter_mut()
            .zip(self.get_key_iterator())
            .for_each(|(c, key_c)| *c ^= key_c);
    }

    /// XOR each byte of the data with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    ///
    /// Should accept anything which has a cheap conversion to a byte iterator.
    /// Shouldn't matter whether the byte iterator's values are owned or borrowed.
    pub fn munge<
        'b,
        Data: IntoIterator<Item = impl Borrow<u8> + BitXor<&'b u8, Output = u8>> + 'b,
    >(
        &'b mut self,
        data: Data,
    ) -> impl Iterator<Item = u8> + 'b {
        data.into_iter()
            .zip(self.get_key_iterator())
            .map(|(c, key_c)| c ^ key_c)
    }

    fn get_key_iterator(&mut self) -> impl Iterator<Item = &u8> {
        self.key
            .iter()
            .cycle()
            .skip(self.offset)
            .inspect(|_| self.offset = ((self.offset) + 1) % self.key.len())
    }

    #[cfg(feature = "io")]
    pub fn reader(self, reader: impl Read) -> impl Read {
        struct Reader<'a, R> {
            xorcism: Xorcism<'a>,
            reader: R,
        }

        impl<'a, R: Read> Read for Reader<'a, R> {
            fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
                let read_count = self.reader.read(buf)?;
                self.xorcism.munge_in_place(&mut buf[..read_count]);
                Ok(read_count)
            }
        }

        Reader {
            xorcism: self,
            reader,
        }
    }

    #[cfg(feature = "io")]
    pub fn writer(self, writer: impl Write) -> impl Write {
        struct Writer<'a, W> {
            xorcism: Xorcism<'a>,
            writer: W,
        }

        impl<'a, W: Write> Write for Writer<'a, W> {
            fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
                const BUF_LEN: usize = 64;
                let mut munge_buf: [u8; BUF_LEN] = [0; BUF_LEN];
                let munge_count = {
                    if buf.len() > BUF_LEN {
                        BUF_LEN
                    } else {
                        buf.len()
                    }
                };
                munge_buf[..munge_count].copy_from_slice(&buf[..munge_count]);
                self.xorcism.munge_in_place(&mut munge_buf);
                self.writer.write(&munge_buf[..munge_count])
            }

            fn flush(&mut self) -> std::io::Result<()> {
                self.writer.flush()
            }
        }

        Writer {
            xorcism: self,
            writer,
        }
    }
}
