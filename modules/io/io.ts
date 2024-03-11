import type { Definition } from "../../build/codegen.ts";

const symbols: Definition[] = [
  {
    name: "io_uring_check_version",
    parameters: ["i32", "i32"],
    result: "i32",
  },
  {
    name: "io_uring_queue_init",
    parameters: ["u32", "pointer", "u32"],
    result: "i32",
  },
  {
    name: "size_of_io_uring",
    parameters: [],
    result: "usize",
  },
  {
    name: "size_of_io_uring_cqe",
    parameters: [],
    result: "usize",
  },
  {
    name: "io_uring_get_sqe",
    parameters: ["pointer"],
    result: "pointer",
  },
  {
    name: "io_uring_sqe_set_data",
    parameters: ["pointer", "pointer"],
    result: "void",
  },
  {
    name: "io_uring_cqe_get_data",
    parameters: ["pointer"],
    result: "usize",
  },
  {
    name: "io_uring_submit",
    parameters: ["pointer"],
    result: "i32",
  },
  {
    name: "io_uring_prep_accept",
    parameters: ["pointer", "i32", "pointer", "pointer", "i32"],
    result: "void",
  },
  {
    name: "io_uring_prep_readv",
    parameters: ["pointer", "i32", "pointer", "i32", "i64"],
    result: "void",
  },
  {
    name: "io_uring_prep_writev",
    parameters: ["pointer", "i32", "pointer", "i32", "i64"],
    result: "void",
  },
  {
    name: "io_uring_prep_send_zc",
    parameters: ["pointer", "i32", "pointer", "i32", "i32", "i32"],
    result: "void",
  },
  {
    name: "io_uring_wait_cqe",
    parameters: ["pointer", "pointer"],
    result: "i32",
  },
  {
    name: "io_uring_cqe_seen",
    parameters: ["pointer", "pointer"],
    result: "void",
  },

  // custom
  {
    name: "io_uring_wait_cqe2",
    parameters: ["pointer", "pointer"],
    result: "i32",
  },
  {
    name: "io_uring_cqe_create_data",
    parameters: ["u32", "u32"],
    result: "pointer",
  },

  // net
  {
    name: "socket",
    parameters: ["i32", "i32", "i32"],
    result: "i32",
  },
  {
    name: "bind",
    parameters: ["i32", "pointer", "u32"],
    result: "i32",
  },
  {
    name: "listen",
    parameters: ["i32", "i32"],
    result: "i32",
  },
  {
    name: "close",
    parameters: ["i32"],
    result: "i32",
  }
];

export default {
  name: "io",
  symbols,
  output: new URL("./mod.rs", import.meta.url).pathname,
}
