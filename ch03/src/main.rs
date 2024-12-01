use std::arch::asm;

fn main() {
    let message = "Hello world from raw syscall!\n";
    let msg = message.to_string();
    syscall(msg)
}

#[inline(never)]
fn syscall(msg: String) {
    let ptr = msg.as_ptr();
    let n = msg.len();

    unsafe {
        asm!(
            "mov x16, 4",
            "mov x0, 1",
            "svc 0",
            in("x1") ptr,
            in("x2") n,
            out("x16") _,
            out("x0") _,
            lateout("x1") _,
            lateout("x2") _
        );
    }
}
