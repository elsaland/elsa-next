pub use libc::*;
pub use liburing::*;

pub fn size_of_io_uring() -> usize {
  std::mem::size_of::<io_uring>()
}

pub fn size_of_io_uring_cqe() -> usize {
  std::mem::size_of::<io_uring_cqe>()
}

#[repr(C)]
pub struct cqe_data {
  tok: u32,
  res: u32,
}

pub fn io_uring_wait_cqe2(ring: *mut io_uring, mut data: *mut cqe_data) {
    unsafe {
        let mut cqe = std::mem::zeroed();
        io_uring_wait_cqe(ring, &mut cqe);
        let tok = io_uring_cqe_get_data(cqe) as u32;

        (*data).tok = tok;
        (*data).res = (*cqe).res as u32;

        io_uring_cqe_seen(ring, cqe);
    }
}
