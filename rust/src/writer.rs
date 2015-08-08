use std::cmp::min;
use std::io;
use std::mem;
use std::ptr;
use std::raw::Repr;
use std::raw;
use std::slice;
use test;

const SRC_LEN: usize = 4;
const BATCHES: usize = 128;

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[inline]
fn do_std_writes<W: Writer>(dst: &mut W, src: &[u8], batches: usize) {
    for _ in range(0, batches) {
        dst.write_all(src).unwrap();
    }
}

#[inline(always)]
fn do_std_writes_inline_always<W: Writer>(dst: &mut W, src: &[u8], batches: usize) {
    for _ in range(0, batches) {
        dst.write_all(src).unwrap();
    }
}

#[inline(never)]
fn do_std_writes_inline_never<W: Writer>(dst: &mut W, src: &[u8], batches: usize) {
    for _ in range(0, batches) {
        dst.write_all(src).unwrap();
    }
}

//////////////////////////////////////////////////////////////////////////////

#[test]
fn test_std_vec_writer() {
    let mut dst = Vec::with_capacity(BATCHES * SRC_LEN);
    let src = &[1; SRC_LEN];

    do_std_writes(&mut dst, src, BATCHES);

    assert_eq!(dst.len(), BATCHES * SRC_LEN);
    assert!(dst.iter().all(|c| *c == 1));
}

#[bench]
fn bench_std_vec_writer(b: &mut test::Bencher) {
    let mut dst = Vec::with_capacity(BATCHES * SRC_LEN);
    let src = &[1; SRC_LEN];

    b.iter(|| {
        dst.clear();

        do_std_writes(&mut dst, src, BATCHES);
    })
}

#[bench]
fn bench_std_vec_writer_inline_always(b: &mut test::Bencher) {
    let mut dst = Vec::with_capacity(BATCHES * SRC_LEN);
    let src = &[1; SRC_LEN];

    b.iter(|| {
        dst.clear();

        do_std_writes_inline_always(&mut dst, src, BATCHES);
    })
}

#[bench]
fn bench_std_vec_writer_inline_never(b: &mut test::Bencher) {
    let mut dst = Vec::with_capacity(BATCHES * SRC_LEN);
    let src = &[1; SRC_LEN];

    b.iter(|| {
        dst.clear();

        do_std_writes_inline_never(&mut dst, src, BATCHES);
    })
}

//////////////////////////////////////////////////////////////////////////////

#[test]
fn test_std_buf_writer() {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    {
        let mut dst = io::BufWriter::new(dst);
        do_std_writes(&mut dst, src, BATCHES);
    }

    assert!(dst.iter().all(|c| *c == 1));
}

#[bench]
fn bench_std_buf_writer(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = io::BufWriter::new(dst);
        do_std_writes(&mut dst, src, BATCHES);
    })
}

#[bench]
fn bench_std_buf_writer_inline_always(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = io::BufWriter::new(dst);
        do_std_writes_inline_always(&mut dst, src, BATCHES);
    })
}

#[bench]
fn bench_std_buf_writer_inline_never(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = io::BufWriter::new(dst);
        do_std_writes_inline_never(&mut dst, src, BATCHES);
    })
}

//////////////////////////////////////////////////////////////////////////////

#[inline]
unsafe fn do_copy_nonoverlapping_memory(mut dst: *mut u8, src: *const u8, len: usize, batches: usize) {
    for _ in range(0, batches) {
        ptr::copy_nonoverlapping_memory(dst, src, len);
        dst = dst.offset(len as isize);
    }
}


#[inline(never)]
unsafe fn do_copy_nonoverlapping_memory_inline_never(mut dst: *mut u8, src: *const u8, len: usize, batches: usize) {
    for _ in range(0, batches) {
        ptr::copy_nonoverlapping_memory(dst, src, len);
        dst = dst.offset(len as isize);
    }
}


#[inline(always)]
unsafe fn do_copy_nonoverlapping_memory_inline_always(mut dst: *mut u8, src: *const u8, len: usize, batches: usize) {
    for _ in range(0, batches) {
        ptr::copy_nonoverlapping_memory(dst, src, len);
        dst = dst.offset(len as isize);
    }
}

#[test]
fn test_copy_nonoverlapping_memory() {
    let dst = &mut [0_u8; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    unsafe {
        do_copy_nonoverlapping_memory(dst.as_mut_ptr(), src.as_ptr(), src.len(), BATCHES);
    }
    assert!(dst.iter().all(|c| *c == 1));
}


#[bench]
fn bench_copy_nonoverlapping_memory(b: &mut test::Bencher) {
    let dst = &mut [0_u8; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        unsafe {
            do_copy_nonoverlapping_memory(dst.as_mut_ptr(), src.as_ptr(), src.len(), BATCHES);
        }
    })
}

#[bench]
fn bench_copy_nonoverlapping_memory_inline_always(b: &mut test::Bencher) {
    let dst = &mut [0_u8; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        unsafe {
            do_copy_nonoverlapping_memory_inline_always(dst.as_mut_ptr(), src.as_ptr(), src.len(), BATCHES);
        }
    })
}

#[bench]
fn bench_copy_nonoverlapping_memory_inline_never(b: &mut test::Bencher) {
    let dst = &mut [0_u8; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        unsafe {
            do_copy_nonoverlapping_memory_inline_never(dst.as_mut_ptr(), src.as_ptr(), src.len(), BATCHES);
        }
    })
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

trait MyWriter {
    fn my_write(&mut self, src: &[u8]) -> io::Result<()>;
}

#[inline]
fn do_my_writes<W>(dst: &mut W, src: &[u8], batches: usize) where W: MyWriter {
    for _ in range(0, batches) {
        dst.my_write(src).unwrap();
    }
}

#[inline(never)]
fn do_my_writes_inline_never<W>(dst: &mut W, src: &[u8], batches: usize) where W: MyWriter {
    for _ in range(0, batches) {
        dst.my_write(src).unwrap();
    }
}

#[inline(always)]
fn do_my_writes_inline_always<W>(dst: &mut W, src: &[u8], batches: usize) where W: MyWriter {
    for _ in range(0, batches) {
        dst.my_write(src).unwrap();
    }
}

//////////////////////////////////////////////////////////////////////////////
// Bound check, as `&mut [u8]`.

impl<'a> MyWriter for &'a mut [u8] {
    #[inline]
    fn my_write(&mut self, src: &[u8]) -> io::Result<()> {
        if self.is_empty() { return Err(io::standard_error(io::EndOfFile)); }

        let dst_len = self.len();
        let src_len = src.len();

        let write_len = min(dst_len, src_len);

        slice::bytes::copy_memory(*self, &src[..write_len]);

        unsafe {
            *self = mem::transmute(raw::Slice {
                data: self.as_ptr().offset(write_len as isize),
                len: dst_len - write_len,
            });
        }

        if src_len > dst_len {
            Err(io::standard_error(io::ShortWrite(write_len)))
        } else {
            Ok(())
        }
    }
}

#[test]
fn test_slice_writer() {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    {
        let mut dst = dst.as_mut_slice();
        do_my_writes(&mut dst, src, BATCHES);
    }

    assert!(dst.iter().all(|c| *c == 1));
}

#[bench]
fn bench_slice_writer(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = dst.as_mut_slice();
        do_my_writes(&mut dst, src, BATCHES);
    })
}

#[bench]
fn bench_slice_writer_inline_always(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = dst.as_mut_slice();
        do_my_writes_inline_always(&mut dst, src, BATCHES);
    })
}

#[bench]
fn bench_slice_writer_inline_never(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = dst.as_mut_slice();
        do_my_writes_inline_never(&mut dst, src, BATCHES);
    })
}

//////////////////////////////////////////////////////////////////////////////
// No bound checks

struct UnsafeWriter<'a> {
    dst: *mut u8,
}

impl<'a> UnsafeWriter<'a> {
    #[inline]
    fn new(dst: &'a mut [u8]) -> UnsafeWriter<'a> {
        UnsafeWriter {
            dst: dst.as_mut_ptr(),
        }
    }
}

impl<'a> MyWriter for UnsafeWriter<'a> {
    #[inline]
    fn my_write(&mut self, src: &[u8]) -> io::Result<()> {
        let len = src.len();
        unsafe {
            ptr::copy_nonoverlapping_memory(self.dst, src.as_ptr(), len);
            self.dst = self.dst.offset(len as isize);
        }
        Ok(())
    }
}

#[test]
fn test_unsafe_writer() {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    {
        let mut dst = UnsafeWriter::new(dst);
        do_my_writes(&mut dst, src, BATCHES);
    }

    assert!(dst.iter().all(|c| *c == 1));
}


#[bench]
fn bench_unsafe_writer(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = UnsafeWriter::new(dst);
        do_my_writes(&mut dst, src, BATCHES);
    })
}

#[bench]
fn bench_unsafe_writer_inline_always(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = UnsafeWriter::new(dst);
        do_my_writes_inline_always(&mut dst, src, BATCHES);
    })
}

#[bench]
fn bench_unsafe_writer_inline_never(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = UnsafeWriter::new(dst);
        do_my_writes_inline_never(&mut dst, src, BATCHES);
    })
}

//////////////////////////////////////////////////////////////////////////////
// Bound check, as `&mut [u8]`.

struct BufWriter0<'a> {
    dst: &'a mut [u8],
}

impl<'a> BufWriter0<'a> {
    #[inline]
    fn new(dst: &'a mut [u8]) -> BufWriter0<'a> {
        BufWriter0 {
            dst: dst
        }
    }
}

impl<'a> MyWriter for BufWriter0<'a> {
    #[inline]
    fn my_write(&mut self, src: &[u8]) -> io::Result<()> {
        if self.dst.is_empty() { return Err(io::standard_error(io::EndOfFile)); }

        let dst_len = self.dst.len();
        let src_len = src.len();

        let write_len = min(dst_len, src_len);

        slice::bytes::copy_memory(self.dst, &src[..write_len]);

        unsafe {
            self.dst = mem::transmute(raw::Slice {
                data: self.dst.as_ptr().offset(write_len as isize),
                len: dst_len - write_len,
            });
        }

        if src_len > dst_len {
            Err(io::standard_error(io::ShortWrite(write_len)))
        } else {
            Ok(())
        }
    }
}

#[test]
fn test_buf_writer_0() {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    {
        let mut dst = BufWriter0::new(dst);
        do_my_writes(&mut dst, src, BATCHES);
    }

    assert!(dst.iter().all(|c| *c == 1));
}

#[bench]
fn bench_buf_writer_0(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = BufWriter0::new(dst);
        do_my_writes(&mut dst, src, BATCHES);
    })
}

#[bench]
fn bench_buf_writer_0_inline_always(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = BufWriter0::new(dst);
        do_my_writes_inline_always(&mut dst, src, BATCHES);
    })
}

#[bench]
fn bench_buf_writer_0_inline_never(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = BufWriter0::new(dst);
        do_my_writes_inline_never(&mut dst, src, BATCHES);
    })
}

//////////////////////////////////////////////////////////////////////////////
// Bound checked, but no error messages.

struct BufWriter1<'a> {
    dst: &'a mut [u8],
}

impl<'a> BufWriter1<'a> {
    #[inline]
    fn new(dst: &'a mut [u8]) -> BufWriter1<'a> {
        BufWriter1 {
            dst: dst,
        }
    }
}

impl<'a> MyWriter for BufWriter1<'a> {
    #[inline]
    fn my_write(&mut self, src: &[u8]) -> io::Result<()> {
        let dst_len = self.dst.len();
        let src_len = src.len();
        let write_len = min(dst_len, src_len);

        slice::bytes::copy_memory(self.dst, &src[..write_len]);

        unsafe {
            let self_: &mut raw::Slice<u8> = mem::transmute(self);
            self_.data = self_.data.offset(write_len as isize);
            self_.len = dst_len - write_len;
        }

        Ok(())
    }
}

#[test]
fn test_buf_writer_1() {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    {
        let mut dst = BufWriter1::new(dst);
        do_my_writes(&mut dst, src, BATCHES);
    }

    assert!(dst.iter().all(|c| *c == 1));
}

#[bench]
fn bench_buf_writer_1(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = BufWriter1::new(dst);
        do_my_writes(&mut dst, src, BATCHES);
    })
}

#[bench]
fn bench_buf_writer_1_inline_always(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = BufWriter1::new(dst);
        do_my_writes_inline_always(&mut dst, src, BATCHES);
    })
}

#[bench]
fn bench_buf_writer_1_inline_never(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = BufWriter1::new(dst);
        do_my_writes_inline_never(&mut dst, src, BATCHES);
    })
}

//////////////////////////////////////////////////////////////////////////////
// Bound check, fixed length.

struct BufWriter2<'a> {
    dst: &'a mut [u8],
}

impl<'a> BufWriter2<'a> {
    #[inline]
    fn new(dst: &'a mut [u8]) -> BufWriter2<'a> {
        BufWriter2 {
            dst: dst,
        }
    }
}

impl<'a> MyWriter for BufWriter2<'a> {
    #[inline]
    fn my_write(&mut self, src: &[u8]) -> io::Result<()> {
        let dst_len = self.dst.len();
        let write_len = src.len();

        slice::bytes::copy_memory(self.dst, &src[..write_len]);

        unsafe {
            let self_: &mut raw::Slice<u8> = mem::transmute(self);
            self_.data = self_.data.offset(write_len as isize);
            self_.len = dst_len - write_len;
        }

        Ok(())
    }
}

#[test]
fn test_buf_writer_2() {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    {
        let mut dst = BufWriter2::new(dst);
        do_my_writes(&mut dst, src, BATCHES);
    }

    assert!(dst.iter().all(|c| *c == 1));
}

#[bench]
fn bench_buf_writer_2(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = BufWriter2::new(dst);
        do_my_writes(&mut dst, src, BATCHES);
    })
}

#[bench]
fn bench_buf_writer_2_inline_always(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = BufWriter2::new(dst);
        do_my_writes_inline_always(&mut dst, src, BATCHES);
    })
}

#[bench]
fn bench_buf_writer_2_inline_never(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = BufWriter2::new(dst);
        do_my_writes_inline_never(&mut dst, src, BATCHES);
    })
}

//////////////////////////////////////////////////////////////////////////////
// Bound check, same as std::BufWriter.

struct BufWriter3<'a> {
    dst: &'a mut [u8],
    pos: usize,
}

impl<'a> BufWriter3<'a> {
    #[inline]
    fn new(dst: &'a mut [u8]) -> BufWriter3<'a> {
        BufWriter3 {
            dst: dst,
            pos: 0,
        }
    }
}

impl<'a> MyWriter for BufWriter3<'a> {
    #[inline]
    fn my_write(&mut self, src: &[u8]) -> io::Result<()> {
        // return an error if the entire write does not fit in the buffer
        let cap = if self.pos >= self.dst.len() { 0 } else { self.dst.len() - self.pos };

        if src.len() > cap {
            return Err(io::Error {
                kind: io::Otherio::Error,
                desc: "Trying to write past end of buffer",
                detail: None
            })
        }

        slice::bytes::copy_memory(&mut self.dst[self.pos..], src);
        self.pos += src.len();
        Ok(())
    }
}

#[test]
fn test_buf_writer_3() {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    {
        let mut dst = BufWriter3::new(dst);
        do_my_writes(&mut dst, src, BATCHES);
    }

    assert!(dst.iter().all(|c| *c == 1));
}

#[bench]
fn bench_buf_writer_3(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = BufWriter3::new(dst);
        do_my_writes(&mut dst, src, BATCHES);
    })
}

#[bench]
fn bench_buf_writer_3_inline_always(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = BufWriter3::new(dst);
        do_my_writes_inline_always(&mut dst, src, BATCHES);
    })
}

#[bench]
fn bench_buf_writer_3_inline_never(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = BufWriter3::new(dst);
        do_my_writes_inline_never(&mut dst, src, BATCHES);
    })
}

//////////////////////////////////////////////////////////////////////////////
// Bound check, pointers for beginning and end.

struct BufWriter4<'a> {
    dst: *mut u8,
    end: *mut u8,
}

impl<'a> BufWriter4<'a> {
    #[inline]
    fn new(dst: &'a mut [u8]) -> BufWriter4<'a> {
        let dst_ptr = dst.as_mut_ptr();

        unsafe {
            BufWriter4 {
                dst: dst_ptr,
                end: dst_ptr.offset(dst.len() as isize),
            }
        }
    }

    #[inline]
    fn capacity(&self) -> usize {
        self.end as usize - self.dst as usize
    }
}

impl<'a> MyWriter for BufWriter4<'a> {
    #[inline]
    fn my_write(&mut self, src: &[u8]) -> io::Result<()> {
        let src_len = src.len();

        if src_len > self.capacity() {
            return Err(io::Error {
                kind: io::Otherio::Error,
                desc: "Trying to write past end of buffer",
                detail: None
            })
        }

        unsafe {
            ptr::copy_nonoverlapping_memory(self.dst, src.as_ptr(), src_len);
            self.dst = self.dst.offset(src_len as isize);
        }

        Ok(())
    }
}

#[test]
fn test_buf_writer_4() {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    {
        let mut dst = BufWriter4::new(dst);
        do_my_writes(&mut dst, src, BATCHES);
    }

    assert!(dst.iter().all(|c| *c == 1));
}

#[bench]
fn bench_buf_writer_4(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = BufWriter4::new(dst);
        do_my_writes(&mut dst, src, BATCHES);
    })
}

#[bench]
fn bench_buf_writer_4_inline_always(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = BufWriter4::new(dst);
        do_my_writes_inline_always(&mut dst, src, BATCHES);
    })
}

#[bench]
fn bench_buf_writer_4_inline_never(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = BufWriter4::new(dst);
        do_my_writes_inline_never(&mut dst, src, BATCHES);
    })
}

//////////////////////////////////////////////////////////////////////////////
// Like BufWriter1, but with no `min`.

struct BufWriter5<'a> {
    dst: &'a mut [u8],
}

impl<'a> BufWriter5<'a> {
    #[inline]
    fn new(dst: &'a mut [u8]) -> BufWriter5<'a> {
        BufWriter5 {
            dst: dst,
        }
    }
}

impl<'a> MyWriter for BufWriter5<'a> {
    #[inline]
    fn my_write(&mut self, src: &[u8]) -> io::Result<()> {
        let dst_len = self.dst.len();
        let src_len = src.len();

        let x = (dst_len < src_len) as usize;
        let write_len = dst_len * x + src_len * (1 - x);

        slice::bytes::copy_memory(self.dst, &src[..write_len]);

        unsafe {
            let self_: &mut raw::Slice<u8> = mem::transmute(self);
            self_.data = self_.data.offset(write_len as isize);
            self_.len = dst_len - write_len;
        }

        Ok(())
    }
}

#[test]
fn test_buf_writer_5() {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    {
        let mut dst = BufWriter5::new(dst);
        do_my_writes(&mut dst, src, BATCHES);
    }

    assert!(dst.iter().all(|c| *c == 1));
}

#[bench]
fn bench_buf_writer_5(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = BufWriter5::new(dst);
        do_my_writes(&mut dst, src, BATCHES);
    })
}

#[bench]
fn bench_buf_writer_5_inline_always(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = BufWriter5::new(dst);
        do_my_writes_inline_always(&mut dst, src, BATCHES);
    })
}

#[bench]
fn bench_buf_writer_5_inline_never(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = BufWriter5::new(dst);
        do_my_writes_inline_never(&mut dst, src, BATCHES);
    })
}

//////////////////////////////////////////////////////////////////////////////
// Bound checked with unsafe code, but no error messages.

struct BufWriter6<'a> {
    dst: &'a mut [u8],
}

impl<'a> BufWriter6<'a> {
    #[inline]
    fn new(dst: &'a mut [u8]) -> BufWriter6<'a> {
        BufWriter6 {
            dst: dst,
        }
    }
}

impl<'a> MyWriter for BufWriter6<'a> {
    #[inline]
    fn my_write(&mut self, src: &[u8]) -> io::Result<()> {
        let dst_len = self.dst.len();
        let src_len = src.len();
        let write_len = min(dst_len, src_len);

        unsafe {
            ptr::copy_nonoverlapping_memory(
                self.dst.as_mut_ptr(),
                src.as_ptr(),
                write_len);

            let self_: &mut raw::Slice<u8> = mem::transmute(self);
            self_.data = self_.data.offset(write_len as isize);
            self_.len = dst_len - write_len;
        }

        Ok(())
    }
}

#[test]
fn test_buf_writer_6() {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    {
        let mut dst = BufWriter6::new(dst);
        do_my_writes(&mut dst, src, BATCHES);
    }

    assert!(dst.iter().all(|c| *c == 1));
}

#[bench]
fn bench_buf_writer_6(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = BufWriter6::new(dst);
        do_my_writes(&mut dst, src, BATCHES);
    })
}

#[bench]
fn bench_buf_writer_6_inline_always(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = BufWriter6::new(dst);
        do_my_writes_inline_always(&mut dst, src, BATCHES);
    })
}

#[bench]
fn bench_buf_writer_6_inline_never(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = BufWriter6::new(dst);
        do_my_writes_inline_never(&mut dst, src, BATCHES);
    })
}

//////////////////////////////////////////////////////////////////////////////
// As BufWriter3, but with unsafe code.

struct BufWriter7<'a> {
    dst: &'a mut [u8],
    pos: usize,
}

impl<'a> BufWriter7<'a> {
    #[inline]
    fn new(dst: &'a mut [u8]) -> BufWriter7<'a> {
        BufWriter7 {
            dst: dst,
            pos: 0,
        }
    }
}

impl<'a> MyWriter for BufWriter7<'a> {
    #[inline]
    fn my_write(&mut self, src: &[u8]) -> io::Result<()> {
        // return an error if the entire write does not fit in the buffer
        let dst_len = self.dst.len();
        let src_len = src.len();

        let cap = if self.pos >= dst_len { 0 } else { dst_len - self.pos };

        if src_len > cap {
            return Err(io::Error {
                kind: io::Otherio::Error,
                desc: "Trying to write past end of buffer",
                detail: None
            })
        }

        unsafe {
            ptr::copy_nonoverlapping_memory(
                self.dst.as_mut_ptr().offset(self.pos as isize),
                src.as_ptr(),
                src_len);
        }

        self.pos += src_len;
        Ok(())
    }
}

#[test]
fn test_buf_writer_7() {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    {
        let mut dst = BufWriter7::new(dst);
        do_my_writes(&mut dst, src, BATCHES);
    }

    assert!(dst.iter().all(|c| *c == 1));
}

#[bench]
fn bench_buf_writer_7(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = BufWriter7::new(dst);
        do_my_writes(&mut dst, src, BATCHES);
    })
}

#[bench]
fn bench_buf_writer_7_inline_always(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = BufWriter7::new(dst);
        do_my_writes_inline_always(&mut dst, src, BATCHES);
    })
}

#[bench]
fn bench_buf_writer_7_inline_never(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = BufWriter7::new(dst);
        do_my_writes_inline_never(&mut dst, src, BATCHES);
    })
}

//////////////////////////////////////////////////////////////////////////////
// Bound check, as BufWriter4, pointers for beginning and end, inline capacity

struct BufWriter8<'a> {
    dst: *mut u8,
    end: *mut u8,
}

impl<'a> BufWriter8<'a> {
    #[inline]
    fn new(dst: &'a mut [u8]) -> BufWriter8<'a> {
        let dst_ptr = dst.as_mut_ptr();

        unsafe {
            BufWriter8 {
                dst: dst_ptr,
                end: dst_ptr.offset(dst.len() as isize),
            }
        }
    }
}

impl<'a> MyWriter for BufWriter8<'a> {
    #[inline]
    fn my_write(&mut self, src: &[u8]) -> io::Result<()> {
        let src_len = src.len();
        let cap = self.end as usize - self.dst as usize;

        if src_len > cap {
            return Err(io::Error {
                kind: io::Otherio::Error,
                desc: "Trying to write past end of buffer",
                detail: None
            })
        }

        unsafe {
            ptr::copy_nonoverlapping_memory(self.dst, src.as_ptr(), src_len);
            self.dst = self.dst.offset(src_len as isize);
        }

        Ok(())
    }
}

#[test]
fn test_buf_writer_8() {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    {
        let mut dst = BufWriter8::new(dst);
        do_my_writes(&mut dst, src, BATCHES);
    }

    assert!(dst.iter().all(|c| *c == 1));
}

#[bench]
fn bench_buf_writer_8(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = BufWriter8::new(dst);
        do_my_writes(&mut dst, src, BATCHES);
    })
}

#[bench]
fn bench_buf_writer_8_inline_always(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = BufWriter8::new(dst);
        do_my_writes_inline_always(&mut dst, src, BATCHES);
    })
}

#[bench]
fn bench_buf_writer_8_inline_never(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = BufWriter8::new(dst);
        do_my_writes_inline_never(&mut dst, src, BATCHES);
    })
}

//////////////////////////////////////////////////////////////////////////////
// Bound check, as BufWriter4, pointers for beginning and end, inline capacity

struct BufWriter9<'a> {
    dst: *mut u8,
    end: *mut u8,
}

impl<'a> BufWriter9<'a> {
    #[inline]
    fn new(dst: &'a mut [u8]) -> BufWriter9<'a> {
        let dst_ptr = dst.as_mut_ptr();

        unsafe {
            BufWriter9 {
                dst: dst_ptr,
                end: dst_ptr.offset(dst.len() as isize),
            }
        }
    }
}

impl<'a> MyWriter for BufWriter9<'a> {
    #[inline]
    fn my_write(&mut self, src: &[u8]) -> io::Result<()> {
        unsafe {
            let raw::Slice { data: src_ptr, len: src_len } = src.repr();

            let cap = self.end as usize - self.dst as usize;

            if src_len > cap {
                return Err(io::Error {
                    kind: io::Otherio::Error,
                    desc: "Trying to write past end of buffer",
                    detail: None
                })
            }

            ptr::copy_nonoverlapping_memory(self.dst, src_ptr, src_len);
            self.dst = self.dst.offset(src_len as isize);
        }

        Ok(())
    }
}

#[test]
fn test_buf_writer_9() {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    {
        let mut dst = BufWriter9::new(dst);
        do_my_writes(&mut dst, src, BATCHES);
    }

    assert!(dst.iter().all(|c| *c == 1));
}

#[bench]
fn bench_buf_writer_9(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = BufWriter9::new(dst);
        do_my_writes(&mut dst, src, BATCHES);
    })
}

#[bench]
fn bench_buf_writer_9_inline_always(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = BufWriter9::new(dst);
        do_my_writes_inline_always(&mut dst, src, BATCHES);
    })
}

#[bench]
fn bench_buf_writer_9_inline_never(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = BufWriter9::new(dst);
        do_my_writes_inline_never(&mut dst, src, BATCHES);
    })
}

//////////////////////////////////////////////////////////////////////////////
// Bound check, same as std::BufWriter.

struct BufWriter10<'a> {
    dst: &'a mut [u8],
    pos: usize,
}

impl<'a> BufWriter10<'a> {
    #[inline]
    fn new(dst: &'a mut [u8]) -> BufWriter10<'a> {
        BufWriter10 {
            dst: dst,
            pos: 0,
        }
    }
}

impl<'a> MyWriter for BufWriter10<'a> {
    #[inline]
    fn my_write(&mut self, src: &[u8]) -> io::Result<()> {
        let dst_len = self.dst.len();
        let src_len = src.len();

        if self.pos == dst_len { return Err(io::standard_error(io::EndOfFile)); }

        let cap = dst_len - self.pos;

        let write_len = min(cap, src_len);

        slice::bytes::copy_memory(&mut self.dst[self.pos..], &src[..write_len]);

        if src_len > dst_len {
            return Err(io::standard_error(io::ShortWrite(write_len)));
        }

        self.pos += write_len;
        Ok(())
    }
}

#[test]
fn test_buf_writer_10() {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    {
        let mut dst = BufWriter10::new(dst);
        do_my_writes(&mut dst, src, BATCHES);
    }

    assert!(dst.iter().all(|c| *c == 1));
}

#[bench]
fn bench_buf_writer_10(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = BufWriter10::new(dst);
        do_my_writes(&mut dst, src, BATCHES);
    })
}

#[bench]
fn bench_buf_writer_10_inline_always(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = BufWriter10::new(dst);
        do_my_writes_inline_always(&mut dst, src, BATCHES);
    })
}

#[bench]
fn bench_buf_writer_10_inline_never(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = BufWriter10::new(dst);
        do_my_writes_inline_never(&mut dst, src, BATCHES);
    })
}

//////////////////////////////////////////////////////////////////////////////
// Bound check, same as std::BufWriter.

struct BufWriter11<'a> {
    dst: &'a mut [u8],
    pos: usize,
}

impl<'a> BufWriter11<'a> {
    #[inline]
    fn new(dst: &'a mut [u8]) -> BufWriter11<'a> {
        BufWriter11 {
            dst: dst,
            pos: 0,
        }
    }
}

impl<'a> MyWriter for BufWriter11<'a> {
    #[inline]
    fn my_write(&mut self, src: &[u8]) -> io::Result<()> {
        let dst = &mut self.dst[self.pos..];
        let dst_len = dst.len();

        if dst_len == 0 {
            return Err(io::standard_error(io::EndOfFile));
        }

        let src_len = src.len();

        if dst_len >= src_len {
            unsafe {
                ptr::copy_nonoverlapping_memory(
                    dst.as_mut_ptr(),
                    src.as_ptr(),
                    src_len);
            }

            self.pos += src_len;

            Ok(())
        } else {
            unsafe {
                ptr::copy_nonoverlapping_memory(
                    dst.as_mut_ptr(),
                    src.as_ptr(),
                    dst_len);
            }

            self.pos += dst_len;

            Err(io::standard_error(io::ShortWrite(dst_len)))
        }
    }
}

#[test]
fn test_buf_writer_11() {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    {
        let mut dst = BufWriter11::new(dst);
        do_my_writes(&mut dst, src, BATCHES);
    }

    assert!(dst.iter().all(|c| *c == 1));
}

#[bench]
fn bench_buf_writer_11(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = BufWriter11::new(dst);
        do_my_writes(&mut dst, src, BATCHES);
    })
}

#[bench]
fn bench_buf_writer_11_inline_always(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = BufWriter11::new(dst);
        do_my_writes_inline_always(&mut dst, src, BATCHES);
    })
}

#[bench]
fn bench_buf_writer_11_inline_never(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = BufWriter11::new(dst);
        do_my_writes_inline_never(&mut dst, src, BATCHES);
    })
}

//////////////////////////////////////////////////////////////////////////////
// Bound check, same as std::BufWriter.

struct BufWriter12<'a> {
    dst: &'a mut [u8],
}

impl<'a> BufWriter12<'a> {
    #[inline]
    fn new(dst: &'a mut [u8]) -> BufWriter12<'a> {
        BufWriter12 {
            dst: dst,
        }
    }
}

impl<'a> MyWriter for BufWriter12<'a> {
    #[inline]
    fn my_write(&mut self, src: &[u8]) -> io::Result<()> {
        let dst_len = self.dst.len();

        if dst_len == 0 {
            return Err(io::standard_error(io::EndOfFile));
        }

        let src_len = src.len();

        if dst_len >= src_len {
            unsafe {
                ptr::copy_nonoverlapping_memory(
                    self.dst.as_mut_ptr(),
                    src.as_ptr(),
                    src_len);

                self.dst = mem::transmute(raw::Slice {
                    data: self.dst.as_ptr().offset(src_len as isize),
                    len: dst_len - src_len,
                });
            }

            Ok(())
        } else {
            unsafe {
                ptr::copy_nonoverlapping_memory(
                    self.dst.as_mut_ptr(),
                    src.as_ptr(),
                    dst_len);

                self.dst = mem::transmute(raw::Slice {
                    data: self.dst.as_ptr().offset(dst_len as isize),
                    len: 0,
                });
            }

            Err(io::standard_error(io::ShortWrite(dst_len)))
        }
    }
}

#[test]
fn test_buf_writer_12() {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    {
        let mut dst = BufWriter12::new(dst);
        do_my_writes(&mut dst, src, BATCHES);
    }

    assert!(dst.iter().all(|c| *c == 1));
}

#[bench]
fn bench_buf_writer_12(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = BufWriter12::new(dst);
        do_my_writes(&mut dst, src, BATCHES);
    })
}

#[bench]
fn bench_buf_writer_12_inline_always(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = BufWriter12::new(dst);
        do_my_writes_inline_always(&mut dst, src, BATCHES);
    })
}

#[bench]
fn bench_buf_writer_12_inline_never(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = BufWriter12::new(dst);
        do_my_writes_inline_never(&mut dst, src, BATCHES);
    })
}

//////////////////////////////////////////////////////////////////////////////
// Bound check, same as std::BufWriter.

struct BufWriter13<'a> {
    dst: *mut u8,
    end: *mut u8,
}

impl<'a> BufWriter13<'a> {
    #[inline]
    fn new(dst: &'a mut [u8]) -> BufWriter13<'a> {
        let dst_ptr = dst.as_mut_ptr();

        unsafe {
            BufWriter13 {
                dst: dst_ptr,
                end: dst_ptr.offset(dst.len() as isize),
            }
        }
    }
}

impl<'a> MyWriter for BufWriter13<'a> {
    #[inline]
    fn my_write(&mut self, src: &[u8]) -> io::Result<()> {
        let dst_len = self.end as usize - self.dst as usize;

        if dst_len == 0 {
            return Err(io::standard_error(io::EndOfFile));
        }

        let src_len = src.len();

        if dst_len >= src_len {
            unsafe {
                ptr::copy_nonoverlapping_memory(
                    self.dst,
                    src.as_ptr(),
                    src_len);

                self.dst = self.dst.offset(src_len as isize);
            }

            Ok(())
        } else {
            unsafe {
                ptr::copy_nonoverlapping_memory(
                    self.dst,
                    src.as_ptr(),
                    dst_len);

                self.dst = self.dst.offset(dst_len as isize);
            }

            Err(io::standard_error(io::ShortWrite(dst_len)))
        }
    }
}

#[test]
fn test_buf_writer_13() {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    {
        let mut dst = BufWriter13::new(dst);
        do_my_writes(&mut dst, src, BATCHES);
    }

    assert!(dst.iter().all(|c| *c == 1));
}

#[bench]
fn bench_buf_writer_13(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = BufWriter13::new(dst);
        do_my_writes(&mut dst, src, BATCHES);
    })
}

#[bench]
fn bench_buf_writer_13_inline_always(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = BufWriter13::new(dst);
        do_my_writes_inline_always(&mut dst, src, BATCHES);
    })
}

#[bench]
fn bench_buf_writer_13_inline_never(b: &mut test::Bencher) {
    let dst = &mut [0; BATCHES * SRC_LEN];
    let src = &[1; SRC_LEN];

    b.iter(|| {
        let mut dst = BufWriter13::new(dst);
        do_my_writes_inline_never(&mut dst, src, BATCHES);
    })
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

struct VecWriter1<'a> {
    dst: &'a mut Vec<u8>,
}

impl<'a> VecWriter1<'a> {
    #[inline]
    fn new(dst: &'a mut Vec<u8>) -> VecWriter1<'a> {
        VecWriter1 {
            dst: dst,
        }
    }
}

impl<'a> MyWriter for VecWriter1<'a> {
    #[inline]
    fn my_write(&mut self, src: &[u8]) -> io::Result<()> {
        let src_len = src.len();

        self.dst.reserve(src_len);

        let dst = self.dst.as_mut_slice();

        unsafe {
            // we reserved enough room in `dst` to store `src`.
            ptr::copy_nonoverlapping(
                dst.as_mut_ptr(),
                src.as_ptr(),
                src_len);
        }

        Ok(())
    }
}

#[test]
fn test_vec_writer_1() {
    let mut dst = Vec::with_capacity(BATCHES * SRC_LEN);
    let src = &[1; SRC_LEN];

    {
        let mut dst = VecWriter1::new(&mut dst);
        do_my_writes(&mut dst, src, BATCHES);
    }

    assert!(dst.iter().all(|c| *c == 1));
}

#[bench]
fn bench_vec_writer_1(b: &mut test::Bencher) {
    let mut dst = Vec::with_capacity(BATCHES * SRC_LEN);
    let src = &[1; SRC_LEN];

    b.iter(|| {
        dst.clear();
        let mut dst = VecWriter1::new(&mut dst);
        do_my_writes(&mut dst, src, BATCHES);
    })
}

#[bench]
fn bench_vec_writer_1_inline_always(b: &mut test::Bencher) {
    let mut dst = Vec::with_capacity(BATCHES * SRC_LEN);
    let src = &[1; SRC_LEN];

    b.iter(|| {
        dst.clear();
        let mut dst = VecWriter1::new(&mut dst);
        do_my_writes_inline_always(&mut dst, src, BATCHES);
    })
}

#[bench]
fn bench_vec_writer_1_inline_never(b: &mut test::Bencher) {
    let mut dst = Vec::with_capacity(BATCHES * SRC_LEN);
    let src = &[1; SRC_LEN];

    b.iter(|| {
        dst.clear();
        let mut dst = VecWriter1::new(&mut dst);
        do_my_writes_inline_never(&mut dst, src, BATCHES);
    })
}
