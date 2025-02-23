use crate::io::BufMut;
use crate::postgres::protocol::Encode;
use byteorder::NetworkEndian;

pub struct Sync;

impl Encode for Sync {
    #[inline]
    fn encode(&self, buf: &mut Vec<u8>) {
        buf.push(b'S');
        buf.put_i32::<NetworkEndian>(4);
    }
}
