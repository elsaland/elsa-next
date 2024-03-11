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
  usr_data: u32,
  res: u32,
}

#[repr(C)]
pub struct cqe_inner_data {
  tok: u32,
  usr_data: u32,
}

pub fn io_uring_cqe_create_data(tok: u32, usr_data: u32) -> *mut cqe_inner_data {
  let data = cqe_inner_data {
    tok,
    usr_data
  };
  let ptr = Box::into_raw(Box::new(data));
  ptr
}

pub fn io_uring_wait_cqe2(ring: *mut io_uring, mut data: *mut cqe_data) -> i32 {
    unsafe {
        let mut cqe = std::mem::zeroed();
        let e = io_uring_wait_cqe(ring, &mut cqe);
        if (*cqe).res < 0 {
            // Bad address
            let err = (*cqe).res;
            let s = std::ffi::CStr::from_ptr(strerror(err));
            let e = s.to_str().unwrap();
            println!("io_uring_wait_cqe: {}", e);
            std::process::exit(1);
        }

        println!("io_uring_wait_cqe: {}", e);
        if e < 0 {
            return e;
        }

        let tok = (*cqe).user_data as *mut cqe_inner_data;
        (*data).usr_data = (*tok).usr_data;
        (*data).tok = (*tok).tok;
        (*data).res = (*cqe).res as u32;

        io_uring_cqe_seen(ring, cqe);
        return 0;
    }
}
