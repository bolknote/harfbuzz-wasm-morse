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
    
    let buf_u8: Vec<u8> = buffer.glyphs.iter().map(|g| g.codepoint as u8).collect();
    let str_buf = String::from_utf8_lossy(&buf_u8);
    
    // Morse code mappings
    let morse_code_map: Vec<&str> = vec![
        ".-",    // A
        "-...",  // B
        "-.-.",  // C
        "-..",   // D
        ".",     // E
        "..-.",  // F
        "--.",   // G
        "....",  // H
        "..",    // I
        ".---",  // J
        "-.-",   // K
        ".-..",  // L
        "--",    // M
        "-.",    // N
        "---",   // O
        ".--.",  // P
        "--.-",  // Q
        ".-.",   // R
        "...",   // S
        "-",     // T
        "..-",   // U
        "...-",  // V
        ".--",   // W
        "-..-",  // X
        "-.--",  // Y
        "--..",  // Z
        "-----", // 0
        ".----", // 1
        "..---", // 2
        "...--", // 3
        "....-", // 4
        ".....", // 5
        "-....", // 6
        "--...", // 7
        "---..", // 8
        "----.", // 9
    ];
    
    let mut morse_glyphs = Vec::new();
    for (i, c) in str_buf.chars().enumerate() {
        let idx = match c {
            'A'..='Z' => (c as usize) - ('A' as usize),
            '0'..='9' => ('Z' as usize - 'A' as usize + 1) + (c as usize) - ('0' as usize),
            _ => continue, // Skip characters that are not A-Z or 0-9
        };

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
    
    buffer.glyphs = morse_glyphs;
    
    for glyph in &mut buffer.glyphs {
        glyph.codepoint = font.get_glyph(glyph.codepoint, 0);
        glyph.x_advance = font.get_glyph_h_advance(glyph.codepoint);
    }
    
    1
}
