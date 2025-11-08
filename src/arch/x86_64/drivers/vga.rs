/// VGA text buffer memory address
const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 25;

/// Current cursor position (static for simple implementation)
static mut CURSOR_POS: usize = 0;

/// Clear the screen with black background
pub fn clear_screen() {
    let vga_buffer = VGA_BUFFER;
    unsafe {
        CURSOR_POS = 0; // Reset cursor position
        for i in 0..(BUFFER_WIDTH * BUFFER_HEIGHT * 2) {
            if i % 2 == 0 {
                *vga_buffer.add(i) = b' '; // space character
            } else {
                *vga_buffer.add(i) = 0x07; // white text on black background
            }
        }
    }
}

/// Print a string at the current cursor position
pub fn print_string(message: &str) {
    let vga_buffer = VGA_BUFFER;
    
    unsafe {
        for &byte in message.as_bytes().iter() {
            if byte == b'\n' {
                // Move to next line
                CURSOR_POS = ((CURSOR_POS / BUFFER_WIDTH) + 1) * BUFFER_WIDTH;
            } else {
                if CURSOR_POS >= BUFFER_WIDTH * BUFFER_HEIGHT {
                    break; // Don't write past screen bounds
                }
                
                let offset = CURSOR_POS * 2;
                *vga_buffer.add(offset) = byte;        // character
                *vga_buffer.add(offset + 1) = 0x0F;    // white on black
                CURSOR_POS += 1;
            }
        }
    }
}

/// Print a string without newline
pub fn print(message: &str) {
    print_string(message);
}

/// Print a string and move to the next line
pub fn println(message: &str) {
    print_string(message);
    print_string("\n");
}
