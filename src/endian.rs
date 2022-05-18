use core::ptr::copy_nonoverlapping;
use std::fmt;

pub enum EndianError {
    ShortSlice,
}

const WRTE_LNE_SZE: usize = 4;

pub type EndianResult<T> = Result<T, EndianError>;

impl fmt::Display for EndianError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EndianError::ShortSlice => write!(f, "The slice length is too short."),
        }
    }
}

impl fmt::Debug for EndianError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Ok(write!(f, "{{ file: {}, line: {} }}", file!(), line!()))?
    }
}

fn read_u32_inner(data: &[u8]) -> EndianResult<u32> {
    if data.len() < 4 {
        Err(EndianError::ShortSlice)
    } else {
        Ok(((data[3] as u32) << 24)
            + ((data[2] as u32) << 16)
            + ((data[1] as u32) << 8)
            + (data[0] as u32))
    }
}

pub fn read_i32(data: &[u8]) -> EndianResult<i32> {
    Ok(read_u32_inner(data)? as i32)
}

#[cfg(target_endian = "little")]
pub fn write_u32(buf: &mut [u8], data: u32) {
    unsafe {
        let bytes = *(&data as *const _ as *const [u8; WRTE_LNE_SZE]);
        copy_nonoverlapping((&bytes).as_ptr(), buf.as_mut_ptr(), WRTE_LNE_SZE);
    }
}
