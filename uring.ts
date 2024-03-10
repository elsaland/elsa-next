// io_uring tcp server

if (!io_uring_check_version(2, 5)) {
  throw new Error("io_uring version 2.5 or later is required");
}

const SIZE_OF_io_uring = size_of_io_uring();
const ring = new Uint8Array(SIZE_OF_io_uring);

const QUEUE_DEPTH = 128;

/* Initialize the io_uring */
io_uring_queue_init(QUEUE_DEPTH, ring, 0);

function sockaddr_in(ip, port) {
  const AF_INET = 2;
  const buf = new ArrayBuffer(16);
  const dv = new DataView(buf);
  dv.setInt16(0, AF_INET, true);
  dv.setUint16(2, port & 0xffff);
  dv.setUint32(4, inet_aton(ip));
  return new Uint8Array(buf);
}

function inet_aton(ip) {
  const [b0, b1, b2, b3] = ip.split(".").map((v) => (parseInt(v, 10) & 0xff));
  return (b0 << 24) + (b1 << 16) + (b2 << 8) + b3;
}

function setup_socket() {
  const PF_INET = 2;
  const SOCK_STREAM = 1;

  const sock = socket(PF_INET, SOCK_STREAM, 0);
  if (sock < 0) {
    throw new Error("Failed to create socket");
  }

  const sv_addr = sockaddr_in("127.0.0.1", 3000);
  if (bind(sock, sv_addr, sv_addr.byteLength) < 0) {
    throw new Error("Failed to bind");
  }

  if (listen(sock, 10) < 0) {
    throw new Error("Failed to listen");
  }

  return sock;
}

const sock = setup_socket();

const Accept = 1;
const Read = 2;
const Write = 3;

function add_accept_request(
  sock,
  addr,
) {
  const sqe = io_uring_get_sqe(ring);
  if (sqe === 0) {
    throw new Error("Failed to get sqe");
  }

  const addrlen = new Uint8Array(8);
  new DataView(addrlen.buffer).setUint32(0, addr.byteLength, true);

  io_uring_prep_accept(sqe, sock, addr, addrlen, 0);

  io_uring_sqe_set_data(sqe, Accept);
  io_uring_submit(ring);
}

function add_read_request(sock) {
  const sqe = io_uring_get_sqe(ring);
  if (sqe === 0) {
    throw new Error("Failed to get sqe");
  }

  const buf = new ArrayBuffer(1024);
  io_uring_prep_readv(sqe, sock, buf, 1, 0);

  io_uring_sqe_set_data(sqe, Read);
  io_uring_submit(ring);
}

const HTTP_RESPONSE = `HTTP/1.1 200 OK
Content-Length: 12
Content-Type: text/plain

Hello, World`;
const HTTP_RESPONSE_BUF = HTTP_RESPONSE.split("").map((c) => c.charCodeAt(0));

function add_write_request(sock) {
  const sqe = io_uring_get_sqe(ring);
  if (sqe === 0) {
    throw new Error("Failed to get sqe");
  }

  const iovec = new Uint8Array(16);
  const buf = new Uint8Array(HTTP_RESPONSE_BUF);

  const buf_ptr = ptr(buf);
  const view = new DataView(iovec.buffer);
  view.setBigUint64(0, BigInt(buf_ptr), true);
  view.setBigUint64(8, BigInt(buf.byteLength), true);

  io_uring_prep_writev(sqe, sock, iovec, 1, 0);

  io_uring_sqe_set_data(sqe, Write);
  io_uring_submit(ring);
}

const cqedata = new Uint8Array(4 * 2);
const cqedata_v = new DataView(cqedata.buffer);

const client_sockaddr = new ArrayBuffer(16);
add_accept_request(sock, client_sockaddr);

let fd = -1;
for (;;) {
  io_uring_wait_cqe2(ring, cqedata);
  const type = cqedata_v.getUint32(0, true);
  switch (type) {
    case Accept:
      add_accept_request(sock, client_sockaddr);

      const req = cqedata_v.getUint32(4, true);
      fd = req;
      add_read_request(req);
      break;
    case Read:
      add_write_request(fd);
      break;
    case Write:
      close(fd);
      break;
    default:
      print("Unknown type: " + type);
      throw new Error("Unknown user data");
  }
}
