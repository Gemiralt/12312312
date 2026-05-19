// cursor.vx - Курсор мыши

let cursor_x = 400;
let cursor_y = 300;
let cursor_hover = 0;

fn update_cursor(mx: i32, my: i32) {
    cursor_x = mx;
    cursor_y = my;
}

fn set_cursor_hover(hover: i32) {
    cursor_hover = hover;
}

fn draw_cursor() {
    let color = 0xFF0000;  // Красный
    
    if cursor_hover != 0 {
        // Режим наведения: красный кружок
        let radius = 6;
        let mut dy = -radius;
        while dy <= radius {
            let mut dx = -radius;
            while dx <= radius {
                if dx * dx + dy * dy <= radius * radius {
                    draw_pixel(cursor_x + dx, cursor_y + dy, color);
                }
                dx = dx + 1;
            }
            dy = dy + 1;
        }
    } else {
        // Обычный режим: красный крестик
        let mut i = -5;
        while i <= 5 {
            draw_pixel(cursor_x + i, cursor_y + i, color);
            draw_pixel(cursor_x + i, cursor_y - i, color);
            i = i + 1;
        }
        // Утолщаем центр
        let mut i = -1;
        while i <= 1 {
            let mut j = -1;
            while j <= 1 {
                if i == 0 || j == 0 {
                    draw_pixel(cursor_x + i, cursor_y + j, color);
                }
                j = j + 1;
            }
            i = i + 1;
        }
    }
}