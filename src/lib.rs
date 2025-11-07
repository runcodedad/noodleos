#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;
mod interrupts;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Kernel entry point called by the bootloader
#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    // Clear the screen
    vga_buffer::clear_screen();
    
    // Print our message - now we're truly in 64-bit long mode!
    vga_buffer::println("Hello from NoodleOS - 64-bit Long Mode!");
    
    // Initialize the IDT
    interrupts::setup_idt();
    vga_buffer::println("IDT initialized successfully!");
    
    // Halt the CPU - simple infinite loop
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}
