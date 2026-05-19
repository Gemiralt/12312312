// colors.vx - Цветовые темы

struct Colors {
    background: u32,
    text: u32,
    accent: u32,
    button: u32,
    button_hover: u32,
    cursor: u32,
}

fn get_default_theme() -> Colors {
    let c: Colors;
    c.background = 0x1A1A2E;
    c.text = 0xFFFFFF;
    c.accent = 0xE94560;
    c.button = 0x16213E;
    c.button_hover = 0x0F3460;
    c.cursor = 0xFF0000;
    return c;
}

fn get_dark_theme() -> Colors {
    let c: Colors;
    c.background = 0x000000;
    c.text = 0x00FF00;
    c.accent = 0x00AA00;
    c.button = 0x002200;
    c.button_hover = 0x004400;
    c.cursor = 0x00FF00;
    return c;
}