use std::num::NonZeroU32;
use softbuffer::{Surface, Context};
use winit::{
    event::{Event, WindowEvent, ElementState, MouseButton},
    event_loop::{EventLoop, ControlFlow},
    window::WindowBuilder,
};

// Здесь будет эмулятор x86 для выполнения kernel.bin
// Пока просто заглушка

fn main() {
    println!("VertexOS Loader");
    println!("Loading kernel.bin...");
    
    // Загружаем скомпилированное ядро
    let kernel_data = include_bytes!("../../kernel/build/kernel.bin");
    println!("Kernel size: {} bytes", kernel_data.len());
    
    // TODO: Запустить эмуляцию x86 и передать управление ядру
    
    println!("Kernel loaded successfully!");
}