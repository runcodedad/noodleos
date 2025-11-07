/// VGA text buffer memory address
const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 25;

/// Clear the screen with black background
pub fn clear_screen() {
    let vga_buffer = VGA_BUFFER;
    unsafe {
        for i in 0..(BUFFER_WIDTH * BUFFER_HEIGHT * 2) {
            if i % 2 == 0 {
                *vga_buffer.add(i) = b' '; // space character
            } else {
                *vga_buffer.add(i) = 0x07; // white text on black background
            }
        }
    }
}

/// Print a string at the top-left of the screen
pub fn print_string(message: &str) {
    let vga_buffer = VGA_BUFFER;
    
    unsafe {
        for (i, &byte) in message.as_bytes().iter().enumerate() {
            if i >= 80 { break; } // Don't go past first line
            
            let offset = i * 2;
            *vga_buffer.add(offset) = byte;        // character
            *vga_buffer.add(offset + 1) = 0x0F;    // white on black
        }
    }
}
