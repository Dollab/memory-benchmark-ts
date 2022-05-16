use wasm_bindgen::prelude::*;
use malloc_size_of_derive::MallocSizeOf;
use malloc_size_of::{MallocSizeOf, MallocSizeOfOps};



#[derive(Default, MallocSizeOf)]
struct Segment { // 44 bytes
    start: Point,  // 8 bytes
    end: Point,    // 8 bytes
    color: Color,  // 4 bytes
    transform: Transform // 24 bytes
}

#[wasm_bindgen]
#[derive(MallocSizeOf)]
pub struct WasmApp {
    segments: Vec<Segment>

}

#[wasm_bindgen]
impl WasmApp {

    #[wasm_bindgen(constructor)]
    pub fn new() -> Self{
        WasmApp {
            segments: Vec::with_capacity(1000000)
        }
    }

    /// Create and store the given number of segments
    pub fn create_segments(&mut self, num: usize){
        for _ in 0..num {
            self.segments.push(Segment::default());
        }
    }

    /// Returns the memory size in bytes allocated by the app
    pub fn allocated_size(&self) -> usize {
        let mut ops = MallocSizeOfOps::default();
        self.size_of(&mut ops)
    }

    /// Capacity
    pub fn capacity(&self) -> usize {
        let capacity = self.segments.capacity();
        capacity * std::mem::size_of::<Segment>()
    }
}


#[derive(Default, MallocSizeOf)]
struct Point { // 8 bytes
    x: f32,
    y: f32
}

#[derive(Default, MallocSizeOf)]
struct Color { // 4 bytes
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

#[derive(Default, MallocSizeOf)]
struct Transform {
    matrix: [f32;6] // 24 bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_malloc_size_of(){
        let mut app = WasmApp::new();
        app.create_segments(1_000_000);
        println!("Size of the app: {:?}", app.allocated_size());
    }
}

