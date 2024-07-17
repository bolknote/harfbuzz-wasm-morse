use harfbuzz_wasm::{Font, Glyph, GlyphBuffer};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn shape(
    _shape_plan: u32,
    font_ref: u32,
    buf_ref: u32,
    _features: u32,
    _num_features: u32,
) -> i32 {
    let font = Font::from_ref(font_ref);
    let mut buffer = GlyphBuffer::from_ref(buf_ref);
    
    // Get buffer as string
    let buf_u8: Vec<u8> = buffer.glyphs.iter().map(|g| g.codepoint as u8).collect();
    let str_buf = String::from_utf8_lossy(&buf_u8);
    
    // Define Morse code mappings (example mapping)
    let morse_code_map: Vec<&str> = vec![
        ".-",   // A
        "-...", // B
        "-.-.", // C
        "-..",  // D
        ".",    // E
        "..-.", // F
        "--.",  // G
        "....", // H
        "..",   // I
        ".---", // J
        "-.-",  // K
        ".-..", // L
        "--",   // M
        "-.",   // N
        "---",  // O
        ".--.", // P
        "--.-", // Q
        ".-.",  // R
        "...",  // S
        "-",    // T
        "..-",  // U
        "...-", // V
        ".--",  // W
        "-..-", // X
        "-.--", // Y
        "--.."  // Z
    ];
    
    // Convert each Latin letter to its Morse code glyph
    let mut morse_glyphs = Vec::new();
    for (i, c) in str_buf.chars().enumerate() {
        let idx = (c as usize) - ('A' as usize); // Assuming only uppercase Latin letters
        if idx < morse_code_map.len() {
            let morse_str = morse_code_map[idx];
            if i > 0 {
                // Insert space as a new glyph
                morse_glyphs.push(Glyph {
                    codepoint: ' ' as u32,
                    flags: 0,
                    x_advance: 0,
                    y_advance: 0,
                    cluster: morse_glyphs.len() as u32,
                    x_offset: 0,
                    y_offset: 0,
                });
            }
            for (ix, x) in morse_str.chars().enumerate() {
                morse_glyphs.push(Glyph {
                    codepoint: x as u32,
                    flags: 0,
                    x_advance: 0,
                    y_advance: 0,
                    cluster: (morse_glyphs.len() + ix) as u32,
                    x_offset: 0,
                    y_offset: 0,
                });
            }
        }
    }
    
    // Update buffer with Morse code glyphs
    buffer.glyphs = morse_glyphs;
    
    for item in buffer.glyphs.iter_mut() {
        // Map character to glyph
        item.codepoint = font.get_glyph(item.codepoint, 0);
        // Set advance width
        item.x_advance = font.get_glyph_h_advance(item.codepoint);
    }
    
    // Buffer is written back to HB on drop
    1
}
