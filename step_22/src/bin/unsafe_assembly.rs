use std::arch::asm;

unsafe fn add(a: u64, b: u64) -> u64 {
    let mut result: u64;
    
    unsafe {
        // assembly needs to be marked as unsafe
        // x86/x86_64 assembly
        asm!(
            "add {0}, {1}",
            inout(reg) a => result,
            in(reg) b,
        );
    }

    result
}

fn main() {
    let a: u64 = 5;
    let b: u64 = 10;
    let result = unsafe { add(a, b) };
    println!("Result: {}", result);
}