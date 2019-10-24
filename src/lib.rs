use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct Frustal {
    width: u32,
    height: u32,
    data: Vec<u8>,
}

#[wasm_bindgen]
impl Frustal {
    pub fn new(width: u32, height: u32) -> Frustal {
        Frustal {
            width,
            height,
            data: vec![0; (width * height * 4) as usize],
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.data = vec![0; (width * height * 4) as usize]
    }

    pub fn data(&self) -> *const u8 {
        self.data.as_ptr()
    }

    pub fn render(&mut self) {
        for v in &mut self.data {
            *v = 125
        }
    }
}
