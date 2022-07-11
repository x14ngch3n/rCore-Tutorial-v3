use core::{arch::asm};

pub unsafe fn print_stack_trace() -> () {
    let mut fp: *const usize;
    asm!("mv {}, fp", out(reg) fp);
    println!("--- Begin stack trace ---");
    while fp != 1 as *const usize {
        let saved_ra = *fp.sub(1);
        let saved_fp = *fp.sub(2);
        println!("Addr: 0x{:016x} fp: 0x{:016x}", saved_ra, saved_fp);
        fp = saved_fp as *const usize;
    }
    println!("--- End stack trace ---");
}
