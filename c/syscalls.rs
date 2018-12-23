use super::utils::copy_stat_into_wasm;
use super::varargs::VarArgs;
use crate::webassembly::Instance;
use byteorder::{ByteOrder, LittleEndian};
use std::slice;
use std::mem::size_of;
/// NOTE: TODO: These syscalls only support wasm_32 for now because they assume offsets are u32
/// Syscall list: https://www.cs.utexas.edu/~bismith/test/syscalls/syscalls32.html
use libc::{
    accept,
    bind,
    c_int,
    c_void,
    chown,
    ioctl,
    // fcntl, setsockopt, getppid
    close,
    connect,
    dup2,
    exit,
    fstat,
    getgid,
    getpeername,
    getpid,
    getsockname,
    getsockopt,
    gid_t,
    iovec,
    listen,
    lseek,
    mkdir,
    msghdr,
    off_t,
    open,
    pid_t,
    pread,
    pwrite,
    read,
    readv,
    recvfrom,
    recvmsg,
    sendmsg,
    sendto,
    setsockopt,
    sockaddr,
    socket,
    socklen_t,
    ssize_t,
    stat,
    uname,
    utsname,
    write,
    writev,
    select,
    FIONBIO,
    sa_family_t,
    in_addr_t,
    in_port_t,
    sockaddr_in,
    FIOCLEX,
};

/// exit
pub extern "C" fn ___syscall1(_which: c_int, mut varargs: VarArgs, instance: &mut Instance) {
    debug!("emscripten::___syscall1 (exit)");
    let status: i32 = varargs.get(instance);
    unsafe {
        exit(status);
    }
}

/// read
pub extern "C" fn ___syscall3(
    _which: c_int,
    mut varargs: VarArgs,
    instance: &mut Instance,
) -> ssize_t {
    debug!("emscripten::___syscall3 (read)");
    let fd: i32 = varargs.get(instance);
    let buf: u32 = varargs.get(instance);
    let count: usize = varargs.get(instance);
    debug!("=> fd: {}, buf_offset: {}, count: {}", fd, buf, count);
    let buf_addr = instance.memory_offset_addr(0, buf as usize) as *mut c_void;
    unsafe { read(fd, buf_addr, count) }
}

/// write
pub extern "C" fn ___syscall4(
    _which: c_int,
    mut varargs: VarArgs,
    instance: &mut Instance,
) -> c_int {
    debug!("emscripten::___syscall4 (write)");
    let fd: i32 = varargs.get(instance);
    let buf: u32 = varargs.get(instance);
    let count: u32 = varargs.get(instance);
    debug!("=> fd: {}, buf: {}, count: {}", fd, buf, count);
    let buf_addr = instance.memory_offset_addr(0, buf as usize) as *const c_void;
    unsafe { write(fd, buf_addr, count as usize) as i32 }
}

/// open
pub extern "C" fn ___syscall5(
    _which: c_int,
    mut varargs: VarArgs,
    instance: &mut Instance,
) -> c_int {
    debug!("emscripten::___syscall5 (open)");
    let pathname: u32 = varargs.get(instance);
    let flags: i32 = varargs.get(instance);
    let mode: u32 = varargs.get(instance);
    let pathname_addr = instance.memory_offset_addr(0, pathname as usize) as *const i8;
    let path_str = unsafe { std::ffi::CStr::from_ptr(pathname_addr).to_str().unwrap() };
    let fd = unsafe { open(pathname_addr, flags, mode) };
    debug!(
        "=> pathname: {}, flags: {}, mode: {} = fd: {}\npath: {}",
        pathname, flags, mode, fd, path_str
    );
    fd
}

/// close
pub extern "C" fn ___syscall6(
    _which: c_int,
    mut varargs: VarArgs,
    instance: &mut Instance,
) -> c_int {
    debug!("emscripten::___syscall6 (close)");
    let fd: i32 = varargs.get(instance);
    debug!("fd: {}", fd);
    unsafe { close(fd) }
}

// chdir
pub extern "C" fn ___syscall12(
    _which: c_int,
    mut varargs: VarArgs,
    instance: &mut Instance,
) -> c_int {
    debug!("emscripten::___syscall12 (chdir)");
    let path_addr: i32 = varargs.get(instance);
    let _path = unsafe {
        let path_ptr = instance.memory_offset_addr(0, path_addr as usize) as *const i8;
        let path = std::ffi::CStr::from_ptr(path_ptr).to_str().unwrap();
        debug!("=> path: {}", path);
        path
    };
    0
}

// getpid
pub extern "C" fn ___syscall20() -> pid_t {
    debug!("emscripten::___syscall20 (getpid)");
    unsafe { getpid() }
}

// mkdir
pub extern "C" fn ___syscall39(
    _which: c_int,
    mut varargs: VarArgs,
    instance: &mut Instance,
) -> c_int {
    debug!("emscripten::___syscall39 (mkdir)");
    let pathname: u32 = varargs.get(instance);
    let mode: u32 = varargs.get(instance);
    let pathname_addr = instance.memory_offset_addr(0, pathname as usize) as *const i8;

    unsafe { mkdir(pathname_addr, mode as _) };
    0
}

// getppid
pub extern "C" fn ___syscall64() -> pid_t {
    debug!("emscripten::___syscall64 (getppid)");
    unsafe { getpid() }
}

/// ioctl
pub extern "C" fn ___syscall54(
    _which: c_int,
    mut varargs: VarArgs,
    instance: &mut Instance,
) -> c_int {
    debug!("emscripten::___syscall54 (ioctl)");
    let fd: i32 = varargs.get(instance);
    let request: u32 = varargs.get(instance);
    debug!("fd: {}, op: {}", fd, request);

    match request as _ {
        21537 => { // FIONBIO
            let argp: u32 = varargs.get(instance);
            let argp_ptr = instance.memory_offset_addr(0, argp as _);
            let ret = unsafe { ioctl(fd, FIONBIO, argp_ptr) };
            debug!("ret: {}", ret);
            ret
        },
        _ => {
            debug!("emscripten::___syscall54 -> non implemented case {}", request);
            0
        }
    }
}

// socketcall
pub extern "C" fn ___syscall102(
    _which: c_int,
    mut varargs: VarArgs,
    instance: &mut Instance,
) -> c_int {
    // debug!("emscripten::___syscall102 (socketcall)");
    let call: u32 = varargs.get(instance);
    let mut socket_varargs: VarArgs = varargs.get(instance);

    match call {
        1 => {
            debug!("emscripten::___syscall102.x (socket)");
            // socket (domain: c_int, ty: c_int, protocol: c_int) -> c_int
            let domain: i32 = socket_varargs.get(instance);
            let ty: i32 = socket_varargs.get(instance);
            let protocol: i32 = socket_varargs.get(instance);
            let socket = unsafe { socket(domain, ty, protocol) };

            debug!(
                "=> domain: {} (AF_INET/2), type: {} (SOCK_STREAM/1), protocol: {} = fd: {}",
                domain, ty, protocol, socket
            );
            // set_cloexec
            unsafe {
                ioctl(socket, FIOCLEX);
            };
            socket
        }
        2 => {
            debug!("emscripten::___syscall102.x (bind)");
            // bind (socket: c_int, address: *const sockaddr, address_len: socklen_t) -> c_int
            // TODO: Emscripten has a different signature.
            let socket: i32 = socket_varargs.get(instance);
            let address: u32 = socket_varargs.get(instance);
            let address_len: u32 = socket_varargs.get(instance);
            let address = instance.memory_offset_addr(0, address as usize) as *mut sockaddr_in;

            let address_len: u32 = 16;
            unsafe {
                // (*address).sin_family = 2; // AF_INET
                // (*address).sin_addr.s_addr = 16777343; // 127.0.0.1
                // (*address).sin_port = 37663; // 8083
                (*address).sin_family = 2; // AF_INET
                (*address).sin_addr.s_addr = 16777343; // 127.0.0.1
                (*address).sin_port = 37663; // 8083
                println!(
                    "\t\t-------\n>> family = {}, address = {}, port = {}",
                    (*address).sin_family,
                    (*address).sin_addr.s_addr,
                    (*address).sin_port,
                );
            }

            // let address = instance.memory_offset_addr(0, address as usize) as *mut GuestSockaddrIn;


            // unsafe {
                // (*address).sin_family = 2; // AF_INET
            //     (*address).sin_addr.s_addr = 16777343; // 127.0.0.1
            //     (*address).sin_port = 8083; // 8783
            //     println!(
            //         "\t\t-------\n>> family = {}, address = {}, port = {}",
            //         (*address).sin_family,
            //         (*address).sin_addr.s_addr,
            //         (*address).sin_port,
            //     );
            // }

            let status = unsafe { bind(socket, address as *mut sockaddr, address_len) };

            debug!(
                "=> socketfd: {}, address: {:?}, address_len: {} = status: {}",
                socket, address, address_len, status
            );
            status
        }
        3 => {
            debug!("emscripten::___syscall102.x (connect)");
            // connect (socket: c_int, address: *const sockaddr, len: socklen_t) -> c_int
            // TODO: Emscripten has a different signature.
            let socket: i32 = socket_varargs.get(instance);
            let address: u32 = socket_varargs.get(instance);
            let address_len: u32 = socket_varargs.get(instance);
            let address = instance.memory_offset_addr(0, address as usize) as *mut sockaddr;
            unsafe { connect(socket, address, address_len) }
        }
        4 => {
            debug!("emscripten::___syscall102.x (listen)");
            // listen (socket: c_int, backlog: c_int) -> c_int
            let socket: i32 = socket_varargs.get(instance);
            let backlog: i32 = 0xFFFFFFF; let _: i32 = socket_varargs.get(instance);
            let status = unsafe { listen(socket, backlog) };

            debug!(
                "=> socketfd: {}, backlog: {} = status: {}",
                socket, backlog, status
            );
            status
        }
        5 => {
            debug!("emscripten::___syscall102.x (accept)");
            // accept (socket: c_int, address: *mut sockaddr, address_len: *mut socklen_t) -> c_int
            let socket: i32 = socket_varargs.get(instance);
            let address: u32 = socket_varargs.get(instance);
            let address_len: u32 = socket_varargs.get(instance);
            let address = instance.memory_offset_addr(0, address as usize) as *mut sockaddr_in;

            let address_len: u32 = 16;
            unsafe {
                (*address).sin_family = 2; // AF_INET
                (*address).sin_addr.s_addr = 16777343; // 127.0.0.1
                (*address).sin_port = 37663; // 8083
                println!(
                    "\t\t-------\n>> family = {}, address = {}, port = {}",
                    (*address).sin_family,
                    (*address).sin_addr.s_addr,
                    (*address).sin_port,
                );
            }

            // let address = instance.memory_offset_addr(0, address as usize) as *mut GuestSockaddrIn;


            // unsafe {
            //     (*address).sin_family = 2; // AF_INET
            //     (*address).sin_addr.s_addr = 2130706433; // 127.0.0.1
            //     (*address).sin_port = 8083; // 8783
            //     println!(
            //         "\t\t-------\n>> family = {}, address = {}, port = {}",
            //         (*address).sin_family,
            //         (*address).sin_addr.s_addr,
            //         (*address).sin_port,
            //     );
            // }


            let address_len_addr =
                instance.memory_offset_addr(0, address_len as usize) as *mut socklen_t;
            let fd = unsafe { accept(socket, address as *mut sockaddr, address_len_addr) };
            debug!(
                "=> socketfd: {}, address: {:?}, address_len: {} = fd: {}",
                socket, address, address_len, fd
            );

            // nix::unistd::write(fd, "Hello, World!".as_bytes()).unwrap();
            // nix::unistd::fsync(fd).unwrap();
            fd
        }
        6 => {
            debug!("emscripten::___syscall102.x (getsockname)");
            // getsockname (socket: c_int, address: *mut sockaddr, address_len: *mut socklen_t) -> c_int
            let socket: i32 = socket_varargs.get(instance);
            let address: u32 = socket_varargs.get(instance);
            let address_len: u32 = socket_varargs.get(instance);
            let address = instance.memory_offset_addr(0, address as usize) as *mut sockaddr;
            let address_len_addr =
                instance.memory_offset_addr(0, address_len as usize) as *mut socklen_t;
            unsafe { getsockname(socket, address, address_len_addr) }
        }
        7 => {
            debug!("emscripten::___syscall102.x (getpeername)");
            // getpeername (socket: c_int, address: *mut sockaddr, address_len: *mut socklen_t) -> c_int
            let socket: i32 = socket_varargs.get(instance);
            let address: u32 = socket_varargs.get(instance);
            let address_len: u32 = socket_varargs.get(instance);
            let address = instance.memory_offset_addr(0, address as usize) as *mut sockaddr;
            let address_len_addr =
                instance.memory_offset_addr(0, address_len as usize) as *mut socklen_t;
            unsafe { getpeername(socket, address, address_len_addr) }
        }
        11 => {
            debug!("emscripten::___syscall102.x (sendto)");
            // sendto (socket: c_int, buf: *const c_void, len: size_t, flags: c_int, addr: *const sockaddr, addrlen: socklen_t) -> ssize_t
            let socket: i32 = socket_varargs.get(instance);
            let buf: u32 = socket_varargs.get(instance);
            let flags: usize = socket_varargs.get(instance);
            let len: i32 = socket_varargs.get(instance);
            let address: u32 = socket_varargs.get(instance);
            let address_len: u32 = socket_varargs.get(instance);
            let buf_addr = instance.memory_offset_addr(0, buf as usize) as *mut c_void;
            let address = instance.memory_offset_addr(0, address as usize) as *mut sockaddr;
            unsafe { sendto(socket, buf_addr, flags, len, address, address_len) as i32 }
        }
        12 => {
            debug!("emscripten::___syscall102.x (recvfrom)");
            // recvfrom (socket: c_int, buf: *const c_void, len: size_t, flags: c_int, addr: *const sockaddr, addrlen: socklen_t) -> ssize_t
            let socket: i32 = socket_varargs.get(instance);
            let buf: u32 = socket_varargs.get(instance);
            let flags: usize = socket_varargs.get(instance);
            let len: i32 = socket_varargs.get(instance);
            let address: u32 = socket_varargs.get(instance);
            let address_len: u32 = socket_varargs.get(instance);
            let buf_addr = instance.memory_offset_addr(0, buf as usize) as *mut c_void;
            let address = instance.memory_offset_addr(0, address as usize) as *mut sockaddr;
            let address_len_addr =
                instance.memory_offset_addr(0, address_len as usize) as *mut socklen_t;
            unsafe { recvfrom(socket, buf_addr, flags, len, address, address_len_addr) as i32 }
        }
        14 => {
            debug!("emscripten::___syscall102.x (setsockopt)");
            // NOTE: Emscripten seems to be passing the wrong values to this syscall
            //      level: Em passes 1 as SOL_SOCKET; SOL_SOCKET is 0xffff in BSD
            //      name: Em passes SO_ACCEPTCONN, but Nginx complains about REUSEADDR
            //      https://github.com/openbsd/src/blob/master/sys/sys/socket.h#L156
            // setsockopt (socket: c_int, level: c_int, name: c_int, value: *const c_void, option_len: socklen_t) -> c_int
            let socket: i32 = socket_varargs.get(instance);
            // SOL_SOCKET = 0xffff in BSD
            let level: i32 = 0xffff;
            let _: u32 = socket_varargs.get(instance);
            // SO_ACCEPTCONN = 0x4
            let name: i32 = 0x4;
            let _: u32 = socket_varargs.get(instance);
            let value: u32 = socket_varargs.get(instance);
            let option_len: u32 = socket_varargs.get(instance);
            let value_addr = instance.memory_offset_addr(0, value as usize) as *mut c_void; // Endian problem
            let ret = unsafe { setsockopt(socket, level, name, value_addr, option_len) };

            // debug!("option_value = {:?}", unsafe { *(value_addr as *const u32) });

            debug!("=> socketfd: {}, level: {} (SOL_SOCKET/0xffff), name: {} (SO_REUSEADDR/4), value_addr: {:?}, option_len: {} = status: {}", socket, level, name, value_addr, option_len, ret);
            ret
        }
        15 => {
            debug!("emscripten::___syscall102.x (getsockopt)");
            // getsockopt (sockfd: c_int, level: c_int, optname: c_int, optval: *mut c_void, optlen: *mut socklen_t) -> c_int
            let socket: i32 = socket_varargs.get(instance);
            let level: i32 = socket_varargs.get(instance);
            let name: i32 = socket_varargs.get(instance);
            let value: u32 = socket_varargs.get(instance);
            let option_len: u32 = socket_varargs.get(instance);
            let value_addr = instance.memory_offset_addr(0, value as usize) as *mut c_void;
            let option_len_addr =
                instance.memory_offset_addr(0, option_len as usize) as *mut socklen_t;
            unsafe { getsockopt(socket, level, name, value_addr, option_len_addr) }
        }
        16 => {
            debug!("emscripten::___syscall102.x (sendmsg)");
            // sendmsg (fd: c_int, msg: *const msghdr, flags: c_int) -> ssize_t
            let socket: i32 = socket_varargs.get(instance);
            let msg: u32 = socket_varargs.get(instance);
            let flags: i32 = socket_varargs.get(instance);
            let msg_addr = instance.memory_offset_addr(0, msg as usize) as *const msghdr;
            unsafe { sendmsg(socket, msg_addr, flags) as i32 }
        }
        17 => {
            debug!("emscripten::___syscall102.x (recvmsg)");
            // recvmsg (fd: c_int, msg: *mut msghdr, flags: c_int) -> ssize_t
            let socket: i32 = socket_varargs.get(instance);
            let msg: u32 = socket_varargs.get(instance);
            let flags: i32 = socket_varargs.get(instance);
            let msg_addr = instance.memory_offset_addr(0, msg as usize) as *mut msghdr;
            unsafe { recvmsg(socket, msg_addr, flags) as i32 }
        }
        _ => {
            // others
            -1
        }
    }
}

/// uname
// NOTE: Wondering if we should return custom utsname, like Emscripten.
pub extern "C" fn ___syscall122(
    _which: c_int,
    mut varargs: VarArgs,
    instance: &mut Instance,
) -> c_int {
    debug!("emscripten::___syscall122 (uname)");
    let buf: u32 = varargs.get(instance);
    debug!("=> buf: {}", buf);
    let buf_addr = instance.memory_offset_addr(0, buf as usize) as *mut utsname;
    unsafe { uname(buf_addr) }
}

// mmap2
pub extern "C" fn ___syscall192(
    _which: c_int,
    mut varargs: VarArgs,
    instance: &mut Instance,
) -> c_int {
    debug!("emscripten::___syscall192 (mmap2)");
    let addr: i32 = varargs.get(instance);
    let len: u32 = varargs.get(instance);
    let prot: i32 = varargs.get(instance);
    let flags: i32 = varargs.get(instance);
    let fd: i32 = varargs.get(instance);
    let off: i32 = varargs.get(instance);
    debug!(
        "=> addr: {}, len: {}, prot: {}, flags: {}, fd: {}, off: {}",
        addr, len, prot, flags, fd, off
    );

    let (memalign, memset) = {
        let emscripten_data = &instance.emscripten_data.as_ref().unwrap();
        (emscripten_data.memalign, emscripten_data.memset)
    };

    if fd == -1 {
        let ptr = memalign(16384, len, instance);
        if ptr == 0 {
            return -1;
        }
        memset(ptr, 0, len, instance);
        ptr as _
    } else {
        -1
    }
}

/// lseek
pub extern "C" fn ___syscall140(
    _which: c_int,
    mut varargs: VarArgs,
    instance: &mut Instance,
) -> off_t {
    debug!("emscripten::___syscall140 (lseek)");
    let fd: i32 = varargs.get(instance);
    let offset: i64 = varargs.get(instance);
    let whence: i32 = varargs.get(instance);
    debug!("=> fd: {}, offset: {}, whence = {}", fd, offset, whence);
    unsafe { lseek(fd, offset, whence) }
}

/// readv
pub extern "C" fn ___syscall145(
    _which: c_int,
    mut varargs: VarArgs,
    instance: &mut Instance,
) -> ssize_t {
    // debug!("emscripten::___syscall145 (readv)");
    let fd: i32 = varargs.get(instance);
    let iov: u32 = varargs.get(instance);
    let iovcnt: i32 = varargs.get(instance);
    // debug!("=> fd: {}, iov: {}, iovcnt = {}", fd, iov, iovcnt);
    let iov_addr = instance.memory_offset_addr(0, iov as usize) as *mut iovec;
    unsafe { readv(fd, iov_addr, iovcnt) }
}

// writev
pub extern "C" fn ___syscall146(
    _which: c_int,
    mut varargs: VarArgs,
    instance: &mut Instance,
) -> ssize_t {
    debug!("emscripten::___syscall146 (writev)");
    let fd: i32 = varargs.get(instance);
    let iov: i32 = varargs.get(instance);
    let iovcnt: i32 = varargs.get(instance);

    #[repr(C)]
    struct GuestIovec {
        iov_base: i32,
        iov_len: i32,
    }

    debug!("=> fd: {}, iov: {}, iovcnt = {}", fd, iov, iovcnt);
    let mut ret = 0;
    unsafe {
        for i in 0..iovcnt {
            let guest_iov_addr = instance.memory_offset_addr(0, (iov + i*8) as usize) as *mut GuestIovec;
            let iov_base = instance.memory_offset_addr(0, (*guest_iov_addr).iov_base as usize) as *const c_void;
            let iov_len: usize = (*guest_iov_addr).iov_len as _;
            // debug!("=> iov_addr: {:?}, {:?}", iov_base, iov_len);
            let curr = write(fd, iov_base, iov_len);
            if curr < 0 {
                return -1
            }
            ret = ret + curr;
        }
        return ret
    }
}

// pread
pub extern "C" fn ___syscall180(
    _which: c_int,
    mut varargs: VarArgs,
    instance: &mut Instance,
) -> c_int {
    debug!("emscripten::___syscall180 (pread)");
    let fd: i32 = varargs.get(instance);
    let buf: u32 = varargs.get(instance);
    let count: u32 = varargs.get(instance);
    {
        let zero: u32 = varargs.get(instance);
        assert_eq!(zero, 0);
    }
    let offset: i64 = varargs.get(instance);

    let buf_ptr = instance.memory_offset_addr(0, buf as _) as _;

    unsafe { pread(fd, buf_ptr, count as _, offset) as _ }
}

// pwrite
pub extern "C" fn ___syscall181(
    _which: c_int,
    mut varargs: VarArgs,
    instance: &mut Instance,
) -> c_int {
    debug!("emscripten::___syscall181 (pwrite)");
    let fd: i32 = varargs.get(instance);
    let buf: u32 = varargs.get(instance);
    let count: u32 = varargs.get(instance);
    {
        let zero: u32 = varargs.get(instance);
        assert_eq!(zero, 0);
    }
    let offset: i64 = varargs.get(instance);

    let buf_ptr = instance.memory_offset_addr(0, buf as _) as _;
    let status = unsafe { pwrite(fd, buf_ptr, count as _, offset) as _ };
    debug!(
        "=> fd: {}, buf: {}, count: {}, offset: {} = status:{}",
        fd, buf, count, offset, status
    );
    status
}

// stat64
pub extern "C" fn ___syscall195(
    _which: c_int,
    mut varargs: VarArgs,
    instance: &mut Instance,
) -> c_int {
    debug!("emscripten::___syscall195 (stat64)");
    let pathname: u32 = varargs.get(instance);
    let buf: u32 = varargs.get(instance);

    let pathname_addr = instance.memory_offset_addr(0, pathname as usize) as *const i8;

    unsafe {
        let mut _stat: stat = std::mem::zeroed();
        let ret = stat(pathname_addr, &mut _stat);
        debug!("ret: {}", ret);
        if ret != 0 {
            return ret;
        }
        copy_stat_into_wasm(instance, buf, &_stat);
    }
    0
}

// fstat64
pub extern "C" fn ___syscall197(
    _which: c_int,
    mut varargs: VarArgs,
    instance: &mut Instance,
) -> c_int {
    debug!("emscripten::___syscall197 (fstat64)");
    let fd: c_int = varargs.get(instance);
    let buf: u32 = varargs.get(instance);

    unsafe {
        let mut stat = std::mem::zeroed();
        let ret = fstat(fd, &mut stat);
        debug!("ret: {}", ret);
        if ret != 0 {
            return ret;
        }
        copy_stat_into_wasm(instance, buf, &stat);
    }

    0
}

// /// fcntl64
// pub extern "C" fn ___syscall221(_which: c_int, mut varargs: VarArgs, instance: &mut Instance) -> c_int {
//     debug!("emscripten::___syscall221");
//     let fd: i32 = varargs.get(instance);
//     let cmd: i32 = varargs.get(instance);
//     debug!("fd: {}, cmd: {}", fd, cmd);
//     unsafe { fcntl(fd, cmd) }
// }

// getgid
pub extern "C" fn ___syscall201() -> gid_t {
    debug!("emscripten::___syscall201 (getgid)");
    unsafe {
        // Maybe fix: Emscripten returns 0 always
        getgid()
    }
}

// getgid32
pub extern "C" fn ___syscall202() -> gid_t {
    debug!("emscripten::___syscall202 (getgid32)");
    unsafe {
        // Maybe fix: Emscripten returns 0 always
        getgid()
    }
}

// chown
pub extern "C" fn ___syscall212(
    _which: c_int,
    mut varargs: VarArgs,
    instance: &mut Instance,
) -> c_int {
    debug!("emscripten::___syscall212 (chown)");

    let pathname: u32 = varargs.get(instance);
    let owner: u32 = varargs.get(instance);
    let group: u32 = varargs.get(instance);

    let pathname_addr = instance.memory_offset_addr(0, pathname as usize) as *const i8;

    unsafe { chown(pathname_addr, owner, group) }
}

// fcntl64
pub extern "C" fn ___syscall221(
    _which: c_int,
    mut varargs: VarArgs,
    instance: &mut Instance,
) -> c_int {
    debug!("emscripten::___syscall221 (fcntl64)");
    // fcntl64
    let _fd: i32 = varargs.get(instance);
    let cmd: u32 = varargs.get(instance);
    match cmd {
        2 => 0,
        _ => -1,
    }
}

// prlimit64
pub extern "C" fn ___syscall340(
    _which: c_int,
    mut varargs: VarArgs,
    instance: &mut Instance,
) -> c_int {
    debug!("emscripten::___syscall340 (prlimit64)");
    // NOTE: Doesn't really matter. Wasm modules cannot exceed WASM_PAGE_SIZE anyway.
    let _pid: i32 = varargs.get(instance);
    let _resource: i32 = varargs.get(instance);
    let _new_limit: u32 = varargs.get(instance);
    let old_limit: u32 = varargs.get(instance);

    if old_limit != 0 {
        // just report no limits
        let buf_ptr = instance.memory_offset_addr(0, old_limit as _) as *mut u8;
        let buf = unsafe { slice::from_raw_parts_mut(buf_ptr, 16) };

        LittleEndian::write_i32(&mut buf[..], -1); // RLIM_INFINITY
        LittleEndian::write_i32(&mut buf[4..], -1); // RLIM_INFINITY
        LittleEndian::write_i32(&mut buf[8..], -1); // RLIM_INFINITY
        LittleEndian::write_i32(&mut buf[12..], -1); // RLIM_INFINITY
    }

    0
}

// dup2
pub extern "C" fn ___syscall63(
    _which: c_int,
    mut varargs: VarArgs,
    instance: &mut Instance,
) -> c_int {
    debug!("emscripten::___syscall63 (dup2)");

    let src: i32 = varargs.get(instance);
    let dst: i32 = varargs.get(instance);

    unsafe { dup2(src, dst) }
}

// newselect
pub extern "C" fn ___syscall142(
    _which: c_int,
    mut varargs: VarArgs,
    instance: &mut Instance,
) -> c_int {
    debug!("emscripten::___syscall142 (newselect)");

    let nfds: i32 = varargs.get(instance);
    let readfds: u32 = varargs.get(instance);
    let writefds: u32 = varargs.get(instance);
    let exceptfds: u32 = varargs.get(instance);
    let _timeout: i32 = varargs.get(instance);

    assert!(nfds <= 64, "`nfds` must be less than or equal to 64");
    assert!(exceptfds == 0, "`exceptfds` is not supporrted");

    let readfds_ptr = instance.memory_offset_addr(0, readfds as _) as _;
    let writefds_ptr = instance.memory_offset_addr(0, writefds as _) as _;

    unsafe { select(nfds, readfds_ptr, writefds_ptr, 0 as _, 0 as _) }
}
