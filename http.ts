const le = (() => {
  const buffer = new ArrayBuffer(2);
  new DataView(buffer).setInt16(0, 256, true);
  return new Int16Array(buffer)[0] === 256;
})();

// struct kevent {
// 	uintptr_t	ident;		/* identifier for this event */
// 	short		filter;		/* filter for event */
// 	u_short		flags;
// 	u_int		fflags;
// 	intptr_t	data;
// 	void		*udata;		/* opaque user data identifier */
// };
function ev_set(ident, filter, flags, fflags, data, udata) {
  const buf = new ArrayBuffer(32);
  const w = new DataView(buf);
  w.setBigUint64(0, BigInt(ident), le);
  w.setInt16(8, filter, le);
  w.setUint16(10, flags, le);
  w.setUint32(12, fflags, le);
  w.setBigUint64(16, BigInt(data), le);
  w.setBigUint64(24, BigInt(udata), le);
  return new Uint8Array(buf);
}

const kq = kqueue();

const nevents = 4096;
const evbuf = new ArrayBuffer(nevents * 24);
const evu8 = new Uint8Array(evbuf);
const events = new Uint32Array(evbuf);
const handles = {};

const EVFILT_READ = -1;
const EV_ADD = 0x1;
const EV_ENABLE = 0x4;

function poll() {
  const rc = kevent(kq, 0, 0, evu8, nevents, 0);
  if (rc > 0) {
    let offset = 0;
    for (let i = 0; i < rc; i++) {
      const fd = events[offset];
      const event = events[offset + 1];
      offset += 6;
      handles[fd] && handles[fd](event);
    }
  }
}

function register(fd, callback) {
  let rc = kevent(
    kq,
    ev_set(fd, EVFILT_READ, EV_ADD | EV_ENABLE, 0, 0, 0),
    1,
    0,
    0,
    0,
  );
  handles[fd] = callback;
}

//
const EPOLLERR = 0x8;
const EPOLLHUP = 0x10;

const bufSize = 16 * 1024;
const AF_INET = 2;
const SOCK_STREAM = 1;
const SOCK_NONBLOCK = 2048;
const O_NONBLOCK = 2048;
const SOL_SOCKET = 1;
const SO_REUSEPORT = 15;

const on = new Uint32Array([1]);
const backlog = 128;

const buf = new ArrayBuffer(bufSize);
const u8 = new Uint8Array(buf);

function inet_aton(ip) {
  const [b0, b1, b2, b3] = ip.split(".").map((v) => (parseInt(v, 10) & 0xff));
  return (b0 << 24) + (b1 << 16) + (b2 << 8) + b3;
}

function struct_sockaddr_in(ip, port) {
  const buf = new ArrayBuffer(16);
  const dv = new DataView(buf);
  dv.setInt16(0, AF_INET, true);
  dv.setUint16(2, port & 0xffff);
  dv.setUint32(4, inet_aton(ip));
  return new Uint8Array(buf);
}

const res = "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, World!";
const resUi8 = new Uint8Array(res.split("").map((x) => x.charCodeAt(0)));

function connect(addr, port) {
  const sfd = socket(AF_INET, SOCK_STREAM, 0);
  print(`sfd: ${sfd}`);
  let rc = setsockopt(sfd, SOL_SOCKET, SO_REUSEPORT, on, 32);
  rc = setsockopt(sfd, SOL_SOCKET, SOCK_NONBLOCK, on, 32);
  rc = bind(sfd, struct_sockaddr_in(addr, port), 16);

  rc = listen(sfd, backlog);
  register(sfd, (event) => {
    if (event & EPOLLERR || event & EPOLLHUP) {
      // ...
    }
    const newfd = accept(sfd, 0, 0, O_NONBLOCK);
    print(`event: ${event} newfd: ${newfd}`);

    register(newfd, (event) => {
      if (event & EPOLLERR || event & EPOLLHUP) {
        // ...
      }

      handles[newfd] = (event) => {
        const bytes = recv(newfd, u8, bufSize, 0);
        // print(`bytes: ${bytes}`);

        if (bytes > 0) {
          send(newfd, resUi8, resUi8.byteLength, 0);
          return;
        }

        close(newfd);
      };
    });
  });
  print("listening on 127.0.0.1:3000");
}

connect("127.0.0.1", 3000);

while (true) {
  poll();
}
