// ui.vx - UI компоненты

struct Button {
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    text: *mut u8,
    hover: i32,
}

fn draw_button(btn: *mut Button, colors: *mut Colors) {
    let c = if btn.hover != 0 { (*colors).button_hover } else { (*colors).button };
    
    // Тень
    let mut y = (*btn).y + 3;
    while y < (*btn).y + (*btn).h as i32 + 3 {
        let mut x = (*btn).x + 3;
        while x < (*btn).x + (*btn).w as i32 + 3 {
            draw_pixel(x, y, 0x00000088);
            x = x + 1;
        }
        y = y + 1;
    }
    
    // Кнопка
    draw_rect((*btn).x, (*btn).y, (*btn).w, (*btn).h, c);
    
    // Рамка
    draw_rect((*btn).x, (*btn).y, (*btn).w, 1, (*colors).accent);
    draw_rect((*btn).x, (*btn).y + (*btn).h as i32 - 1, (*btn).w, 1, (*colors).accent);
    draw_rect((*btn).x, (*btn).y, 1, (*btn).h, (*colors).accent);
    draw_rect((*btn).x + (*btn).w as i32 - 1, (*btn).y, 1, (*btn).h, (*colors).accent);
}

fn is_hover(btn: *mut Button, mx: i32, my: i32) -> i32 {
    if mx >= (*btn).x && mx <= (*btn).x + (*btn).w as i32 &&
       my >= (*btn).y && my <= (*btn).y + (*btn).h as i32 {
        return 1;
    }
    return 0;
}