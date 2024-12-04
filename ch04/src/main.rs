fn main() {
    let bitflag_a: i32 = 1 << 31;
    let bitflag_b: i32 = 0x1;
    let bitmask: i32 = bitflag_a|bitflag_b;
    println!("{bitflag_a:032b}");
    println!("{bitflag_b:032b}");
    println!("{bitmask:032b}");
}
