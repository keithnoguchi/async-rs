use std::io;

fn main() {
    let msg = "Hello world from syscall!\n";
    let msg = msg.to_string();
    syscall(msg).unwrap();
}

#[cfg(target_family = "unix")]
#[link(name = "C")]
extern "C" {
    fn write(fd: u32, buf: *const u8, count: usize) -> i32;
}

fn syscall(msg: String) -> io::Result<()> {
    let ptr = msg.as_ptr();
    let n = msg.len();
    let ret = unsafe { write(1, ptr, n) };

    if ret == -1 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}
