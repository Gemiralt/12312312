use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("VertexC Compiler v1.3");
        println!("Usage: compiler <input.vx> [output.bin]");
        return;
    }

    let mut source = String::new();
    let main_file = &args[1];
    
    // Поддержка include
    let mut processed = Vec::new();
    load_file(&main_file, &mut source, &mut processed);
    
    let output = if args.len() > 2 { &args[2] } else { "output.bin" };
    
    let mut bytes = vec![0xB8, 0x00, 0x80, 0x0B, 0x00];
    let mut pos = 0;
    let mut i = 0;
    let chars: Vec<char> = source.chars().collect();
    
    while i < chars.len() {
        // Пропускаем пробелы
        while i < chars.len() && chars[i].is_whitespace() {
            i += 1;
        }
        if i >= chars.len() { break; }
        
        // Обработка while
        if i + 5 < chars.len() && chars[i] == 'w' && chars[i+1] == 'h' && chars[i+2] == 'i' && chars[i+3] == 'l' && chars[i+4] == 'e' {
            i += 5;
            // Пропускаем до {
            while i < chars.len() && chars[i] != '{' {
                i += 1;
            }
            i += 1;
            
            let loop_start = bytes.len();
            
            // Тело цикла
            let mut brace_count = 1;
            while i < chars.len() && brace_count > 0 {
                if chars[i] == '{' { brace_count += 1; }
                if chars[i] == '}' { brace_count -= 1; }
                if brace_count > 0 && chars[i] != '}' {
                    // Парсим инструкции внутри цикла
                    if chars[i] == 'v' && i+3 < chars.len() && chars[i+1] == 'g' && chars[i+2] == 'a' && chars[i+3] == '[' {
                        i += 4;
                        let mut num_str = String::new();
                        while i < chars.len() && chars[i] != ']' {
                            num_str.push(chars[i]);
                            i += 1;
                        }
                        let idx = num_str.parse::<u8>().unwrap_or(0);
                        i += 1;
                        
                        while i < chars.len() && chars[i] != '\'' {
                            i += 1;
                        }
                        i += 1;
                        let ch = chars[i] as u8;
                        i += 1;
                        while i < chars.len() && chars[i] != ';' {
                            i += 1;
                        }
                        i += 1;
                        
                        bytes.push(0xC7);
                        bytes.push(0x40);
                        bytes.push(idx);
                        bytes.push(ch);
                        bytes.push(0x0F);
                        bytes.push(0x00);
                        bytes.push(0x00);
                        pos += 2;
                        continue;
                    }
                }
                i += 1;
            }
            
            // JMP loop_start (EB rel8)
            let offset = (loop_start as i32 - bytes.len() as i32 - 2) as i8;
            bytes.push(0xEB);
            bytes.push(offset as u8);
            continue;
        }
        
        // Обычный парсинг vga[X] = 'Y'
        if chars[i] == 'v' && i+3 < chars.len() && chars[i+1] == 'g' && chars[i+2] == 'a' && chars[i+3] == '[' {
            i += 4;
            let mut num_str = String::new();
            while i < chars.len() && chars[i] != ']' {
                num_str.push(chars[i]);
                i += 1;
            }
            let idx = num_str.parse::<u8>().unwrap_or(0);
            i += 1;
            
            while i < chars.len() && chars[i] != '\'' {
                i += 1;
            }
            i += 1;
            let ch = chars[i] as u8;
            i += 1;
            
            let mut color = 0x0F;
            if i < chars.len() && chars[i] == ',' {
                i += 1;
                let mut color_str = String::new();
                while i < chars.len() && chars[i] != ';' && !chars[i].is_whitespace() {
                    color_str.push(chars[i]);
                    i += 1;
                }
                if let Ok(c) = u8::from_str_radix(&color_str, 16) {
                    color = c;
                }
            }
            
            while i < chars.len() && chars[i] != ';' {
                i += 1;
            }
            i += 1;
            
            let imm = ((color as u32) << 8) | (ch as u32);
            
            bytes.push(0xC7);
            bytes.push(0x40);
            bytes.push(idx);
            bytes.push(imm as u8);
            bytes.push((imm >> 8) as u8);
            bytes.push(0x00);
            bytes.push(0x00);
            pos += 2;
            continue;
        }
        
        i += 1;
    }
    
    bytes.push(0xEB);
    bytes.push(0xFE); // Бесконечный цикл
    
    fs::write(output, &bytes).unwrap();
    println!("Compiled {} -> {} ({} bytes)", args[1], output, bytes.len());
}

fn load_file(path: &str, target: &mut String, processed: &mut Vec<String>) {
    if processed.contains(&path.to_string()) {
        return;
    }
    processed.push(path.to_string());
    
    let content = fs::read_to_string(path).unwrap();
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("include ") {
            let include_file = trimmed[8..].trim().trim_matches('"');
            let dir = std::path::Path::new(path).parent().unwrap();
            let full_path = dir.join(include_file).to_str().unwrap().to_string();
            load_file(&full_path, target, processed);
        } else {
            target.push_str(line);
            target.push('\n');
        }
    }
}