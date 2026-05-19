; Тестовое ядро с клавиатурой
BITS 32
ORG 0x100000

start:
    ; Разрешаем прерывания
    sti
    
main_loop:
    ; Читаем клавишу из порта 0x60
    mov dx, 0x60
    in al, dx
    
    ; Если клавиша нажата (не 0)
    cmp al, 0
    je check_timer
    
    ; Сохраняем символ в VGA
    mov ebx, 0xB8000
    mov [ebx], al
    mov byte [ebx+1], 0x0F
    
check_timer:
    ; Читаем таймер
    mov dx, 0x40
    in al, dx
    
    ; Показываем значение таймера
    mov ebx, 0xB8004
    mov [ebx], al
    mov byte [ebx+1], 0x0E
    
    jmp main_loop

; Прерывание от клавиатуры (IRQ1)
keyboard_int:
    mov al, 'K'
    mov ebx, 0xB8008
    mov [ebx], al
    iret

; Прерывание от таймера (IRQ0)
timer_int:
    mov al, 'T'
    mov ebx, 0xB800A
    mov [ebx], al
    iret