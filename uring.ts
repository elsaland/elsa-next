/* io_uring TCP tcp server example */

if (!io_uring_check_version(2, 5)) {
  throw new Error("io_uring version 2.5 or later is required");
}

const SIZE_OF_io_uring = size_of_io_uring();
const ring = new Uint8Array(SIZE_OF_io_uring);

const QUEUE_DEPTH = 1024;

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

  const sv_addr = sockaddr_in("0.0.0.0", 3000);
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

function usrData(tok, data) {
  return io_uring_cqe_create_data(tok, data);
}

function add_accept_request(
  sock,
  addr,
) {
  const sqe = io_uring_get_sqe(ring);
  if (sqe === 0) {
    throw new Error("Failed to get sqe");
  }

  io_uring_prep_accept(sqe, sock, addr, addr.byteLength, 0);

  io_uring_sqe_set_data(sqe, usrData(Accept, 0));
  io_uring_submit(ring);
}

const readBuf = new ArrayBuffer(8192);
function add_read_request(sock) {
  const sqe = io_uring_get_sqe(ring);
  if (sqe === 0) {
    throw new Error("Failed to get sqe");
  }

  io_uring_prep_readv(sqe, sock, readBuf, 1, 0);

  io_uring_sqe_set_data(sqe, usrData(Read, sock));
  io_uring_submit(ring);
}

const HTTP_RESPONSE = "HTTP/1.1 200 OK\r\nContent-Length: 11\r\n\r\nHello World";
const HTTP_RESPONSE_BUF = HTTP_RESPONSE.split("").map((c) => c.charCodeAt(0));

const iovec = new Uint8Array(16);
const buf = new Uint8Array(HTTP_RESPONSE_BUF);
globalThis.buf = buf;

function add_write_request(sock) {
  const sqe = io_uring_get_sqe(ring);
  if (sqe === 0) {
    throw new Error("Failed to get sqe");
  }

  // io_uring_prep_writev(sqe, sock, iovec, 1, 0);
  io_uring_prep_send_zc(sqe, sock, buf, buf.byteLength, 0);

  io_uring_sqe_set_data(sqe, usrData(Write, sock));
  io_uring_submit(ring);
}

const cqedata = new Uint8Array(4 * 3);
const cqedata_v = new DataView(cqedata.buffer);

const client_sockaddr = new ArrayBuffer(16);
add_accept_request(sock, client_sockaddr);

let fd, req;
for (;;) {
  const ret = io_uring_wait_cqe2(ring, cqedata);
  if (ret < 0) {
    throw new Error("Failed to wait cqe");
    break;
  }

  const type = cqedata_v.getUint32(0, true);
  switch (type) {
    case Accept:
      add_accept_request(sock, client_sockaddr);
      req = cqedata_v.getUint32(8, true);
      if (req < 0) {
        throw new Error("Accept failed");
      }
      print("Accept: " + req);
      add_read_request(req);
      break;
    case Read:
      fd = cqedata_v.getUint32(4, true);
      req = cqedata_v.getUint32(8, true);
      if (req == 0) {
        print("Empty request");
        close(fd);
      } else {
        print("Read: " + fd);
        add_write_request(fd);
      }
      break;
    case Write:
      print("Write: " + cqedata_v.getUint32(4, true));
      fd = cqedata_v.getUint32(4, true);
      close(fd);
      break;
    default:
      print("Unknown type: " + type);
      throw new Error("Unknown user data");
  }
}
