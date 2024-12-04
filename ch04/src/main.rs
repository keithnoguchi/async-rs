fn main() {
    let bitflag_a: i32 = 1 << 31;
    let bitflag_b: i32 = 0x1;
    let bitmask: i32 = bitflag_a|bitflag_b;
    println!("{bitflag_a:032b}");
    println!("{bitflag_b:032b}");
    println!("{bitmask:032b}");
    check(bitmask);
}

fn check(bitmask: i32) {
    const EPOLLIN: i32 = 0x1;
    const EPOLLET: i32 = 1 << 31;
    const EPOLLONESHOT: i32 = 0x40000000;
    let read = bitmask & EPOLLIN != 0;
    let et = bitmask & EPOLLET != 0;
    let oneshot = bitmask & EPOLLONESHOT != 0;
    println!("read? {read}, edge_triggered? {et}, oneshot? {oneshot}");
}
