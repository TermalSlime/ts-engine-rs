
pub fn hex_to_floats(hex: u32) -> (f32, f32, f32, f32) {
    let red = (hex & 0xff000000) >> 24;
    let gre = (hex & 0x00ff0000) >> 16;
    let blu = (hex & 0x0000ff00) >> 8;
    let alp = hex & 0x000000ff;

    (
        red as f32 / 255.0,
        gre as f32 / 255.0,
        blu as f32 / 255.0,
        alp as f32 / 255.0,
    )
}
