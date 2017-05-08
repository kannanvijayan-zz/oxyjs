
use parser::char_utils::{AsciiChar, NonAsciiChar};
use std::slice;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StreamPosition(u32);
impl StreamPosition {
    pub fn default() -> StreamPosition {
        StreamPosition(u32::max_value())
    }

    pub fn value(self) -> u32 {
        self.0
    }

    pub fn length_from(self, other: StreamPosition) -> u32 {
        assert!(self.0 >= other.0);
        self.0 - other.0
    }

    fn offset_forward(self, bytes: u32) -> StreamPosition {
        StreamPosition(self.0 + bytes)
    }
}

pub trait InputStream {
    // Read/unread the next 8-bit character.
    fn read_ascii(&mut self) -> AsciiChar;
    fn unread_ascii(&mut self, ch: AsciiChar);

    // Read/unread the next unicode char.
    fn read_non_ascii(&mut self) -> NonAsciiChar;
    fn unread_non_ascii(&mut self, ch: NonAsciiChar);

    // Mark or rewind to a position in the stream.
    fn mark(&self) -> StreamPosition;
    fn rewind(&mut self, posn: StreamPosition);

    // Check for the given ascii text at the given position.
    fn check_ascii_text(&self, text: &[char], posn: StreamPosition) -> bool;
}

pub struct VecInputStream {
    data_cur: *const u8,
    data_end: *const u8,
    data_start: *const u8,
    data: Vec<u8>
}

impl VecInputStream {
    pub fn new(mut data: Vec<u8>) -> VecInputStream {
        let data_start = data.as_mut_slice().as_ptr();
        let data_end = unsafe { data_start.offset(data.len() as isize) };
        assert!(((data_end as usize) - (data_start as usize)) <= (u32::max_value() as usize));
        let data_cur = data_start;
        VecInputStream { data_cur, data_end, data_start, data }
    }

    pub fn current_offset(&self) -> u32 {
        ((self.data_cur as usize) - (self.data_start as usize)) as u32
    }
    pub fn end_offset(&self) -> u32 {
        ((self.data_end as usize) - (self.data_start as usize)) as u32
    }
    pub fn is_valid_position(&self, posn: StreamPosition) -> bool {
        posn.0 <= self.end_offset()
    }
    pub fn is_rewind_position(&self, posn: StreamPosition) -> bool {
        posn.0 <= self.current_offset()
    }

    unsafe fn ptr_at(&self, posn: StreamPosition) -> *const u8 {
        assert!(self.is_valid_position(posn));
        unsafe { self.data_start.offset(posn.0 as isize) }
    }
}
impl InputStream for VecInputStream {
    fn read_ascii(&mut self) -> AsciiChar {
        if self.data_cur < self.data_end {
            let ch = unsafe { *(self.data_cur) };
            self.data_cur = unsafe { self.data_cur.offset(1) };
            AsciiChar::new(ch)
        } else {
            AsciiChar::end()
        }
    }

    fn unread_ascii(&mut self, ch: AsciiChar) {
        assert!(ch.is_valid());
        if ! ch.is_end() {
            assert!(self.data_start < self.data_cur);
            self.data_cur = unsafe { self.data_cur.offset(-1) };
        }
    }

    fn read_non_ascii(&mut self) -> NonAsciiChar {
        panic!("Reading non-ascii unicode chars not yet supported!");
    }

    fn unread_non_ascii(&mut self, ch: NonAsciiChar) {
        panic!("Reading non-ascii unicode chars not yet supported!");
    }

    fn mark(&self) -> StreamPosition {
        StreamPosition(self.current_offset())
    }
    fn rewind(&mut self, posn: StreamPosition) {
        assert!(self.is_rewind_position(posn));
        self.data_cur = unsafe { self.ptr_at(posn) };
    }

    #[inline(always)]
    fn check_ascii_text(&self, text: &[char], posn: StreamPosition) -> bool {
        assert!(self.is_valid_position(posn.offset_forward(text.len() as u32)));
        unsafe {
            let mut curs = self.ptr_at(posn);
            for c in text {
                assert!(*curs < 0x80);
                if *curs != (*c as u8) {
                    return false;
                }
                curs = curs.offset(1);
            }
        }
        true
    }
}
