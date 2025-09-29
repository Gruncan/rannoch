use crate::vga::{ColorCode, ScreenChar, BUFFER_HEIGHT, BUFFER_WIDTH};
use lazy_static::lazy_static;


macro_rules! vga_buf {
    () => (None);
    ($ascii:expr, $foreground:expr, $background:expr) => { Some(ScreenChar {
        ascii_character: $ascii,
        color_code: ColorCode::new_from_int($background, $foreground)
    })};
}


pub struct VgaImage {
    pub(crate) buf: [[Option<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

lazy_static! {
    pub static ref LOAD_SCREEN_BUFFER: VgaImage = VgaImage {
        buf: [
            [vga_buf!(); 80],
            [vga_buf!(); 80],
            [vga_buf!(); 80],
            [vga_buf!(); 80],
            [vga_buf!(); 80],
            [vga_buf!(); 80],
            [vga_buf!(); 80],
            [vga_buf!(); 80],
            [vga_buf!(); 80],
            [vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(b' ',0x1,0x1), vga_buf!(b' ',0x1,0x1), vga_buf!(b' ',0x1,0x1), vga_buf!(b' ',0x1,0x1), vga_buf!(            ), vga_buf!(), vga_buf!(b' ',0x1,0x1), vga_buf!(b' ',0x1,0x1), vga_buf!(b' ',0x1,0x1), vga_buf!(b' ',0x1,0x1), vga_buf!(b' ',0x1,0x1), vga_buf!(), vga_buf!(b' ',0x1,0x1), vga_buf!(b' ',0x1,0x1), vga_buf!(            ), vga_buf!(            ), vga_buf!(b' ',0x1,0x1), vga_buf!(), vga_buf!(b' ',0x1,0x1), vga_buf!(b' ',0x1,0x1), vga_buf!(            ), vga_buf!(            ), vga_buf!(b' ',0x1,0x1), vga_buf!(), vga_buf!(b' ',0x1,0x1), vga_buf!(b' ',0x1,0x1), vga_buf!(b' ',0x1,0x1), vga_buf!(b' ',0x1,0x1), vga_buf!(b' ',0x1,0x1), vga_buf!(), vga_buf!(b' ',0x1,0x1), vga_buf!(b' ',0x1,0x1), vga_buf!(b' ',0x1,0x1), vga_buf!(b' ',0x1,0x1), vga_buf!(b' ',0x1,0x1), vga_buf!(), vga_buf!(b' ',0x1,0x1), vga_buf!(            ), vga_buf!(            ), vga_buf!(            ), vga_buf!(b' ',0x1,0x1), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!()],
            [vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(b' ',0x1,0x1), vga_buf!(            ), vga_buf!(            ), vga_buf!(b' ',0x1,0x1), vga_buf!(            ), vga_buf!(), vga_buf!(b' ',0x1,0x1), vga_buf!(            ), vga_buf!(            ), vga_buf!(            ), vga_buf!(b' ',0x1,0x1), vga_buf!(), vga_buf!(b' ',0x1,0x1), vga_buf!(            ), vga_buf!(b' ',0x1,0x1), vga_buf!(            ), vga_buf!(b' ',0x1,0x1), vga_buf!(), vga_buf!(b' ',0x1,0x1), vga_buf!(            ), vga_buf!(b' ',0x1,0x1), vga_buf!(            ), vga_buf!(b' ',0x1,0x1), vga_buf!(), vga_buf!(b' ',0x1,0x1), vga_buf!(            ), vga_buf!(            ), vga_buf!(            ), vga_buf!(b' ',0x1,0x1), vga_buf!(), vga_buf!(b' ',0x1,0x1), vga_buf!(            ), vga_buf!(            ), vga_buf!(            ), vga_buf!(            ), vga_buf!(), vga_buf!(b' ',0x1,0x1), vga_buf!(            ), vga_buf!(            ), vga_buf!(            ), vga_buf!(b' ',0x1,0x1), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!()],
            [vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(b' ',0x1,0x1), vga_buf!(b' ',0x1,0x1), vga_buf!(b' ',0x1,0x1), vga_buf!(b' ',0x1,0x1), vga_buf!(            ), vga_buf!(), vga_buf!(b' ',0x1,0x1), vga_buf!(b' ',0x1,0x1), vga_buf!(b' ',0x1,0x1), vga_buf!(b' ',0x1,0x1), vga_buf!(b' ',0x1,0x1), vga_buf!(), vga_buf!(b' ',0x1,0x1), vga_buf!(            ), vga_buf!(b' ',0x1,0x1), vga_buf!(            ), vga_buf!(b' ',0x1,0x1), vga_buf!(), vga_buf!(b' ',0x1,0x1), vga_buf!(            ), vga_buf!(b' ',0x1,0x1), vga_buf!(            ), vga_buf!(b' ',0x1,0x1), vga_buf!(), vga_buf!(b' ',0x1,0x1), vga_buf!(            ), vga_buf!(            ), vga_buf!(            ), vga_buf!(b' ',0x1,0x1), vga_buf!(), vga_buf!(b' ',0x1,0x1), vga_buf!(            ), vga_buf!(            ), vga_buf!(            ), vga_buf!(            ), vga_buf!(), vga_buf!(b' ',0x1,0x1), vga_buf!(b' ',0x1,0x1), vga_buf!(b' ',0x1,0x1), vga_buf!(b' ',0x1,0x1), vga_buf!(b' ',0x1,0x1), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!()],
            [vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(b' ',0x1,0x1), vga_buf!(            ), vga_buf!(            ), vga_buf!(            ), vga_buf!(b' ',0x1,0x1), vga_buf!(), vga_buf!(b' ',0x1,0x1), vga_buf!(            ), vga_buf!(            ), vga_buf!(            ), vga_buf!(b' ',0x1,0x1), vga_buf!(), vga_buf!(b' ',0x1,0x1), vga_buf!(            ), vga_buf!(b' ',0x1,0x1), vga_buf!(            ), vga_buf!(b' ',0x1,0x1), vga_buf!(), vga_buf!(b' ',0x1,0x1), vga_buf!(            ), vga_buf!(b' ',0x1,0x1), vga_buf!(            ), vga_buf!(b' ',0x1,0x1), vga_buf!(), vga_buf!(b' ',0x1,0x1), vga_buf!(            ), vga_buf!(            ), vga_buf!(            ), vga_buf!(b' ',0x1,0x1), vga_buf!(), vga_buf!(b' ',0x1,0x1), vga_buf!(            ), vga_buf!(            ), vga_buf!(            ), vga_buf!(            ), vga_buf!(), vga_buf!(b' ',0x1,0x1), vga_buf!(            ), vga_buf!(            ), vga_buf!(            ), vga_buf!(b' ',0x1,0x1), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!()],
            [vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(b' ',0x1,0x1), vga_buf!(            ), vga_buf!(            ), vga_buf!(            ), vga_buf!(b' ',0x1,0x1), vga_buf!(), vga_buf!(b' ',0x1,0x1), vga_buf!(            ), vga_buf!(            ), vga_buf!(            ), vga_buf!(b' ',0x1,0x1), vga_buf!(), vga_buf!(b' ',0x1,0x1), vga_buf!(            ), vga_buf!(            ), vga_buf!(b' ',0x1,0x1), vga_buf!(b' ',0x1,0x1), vga_buf!(), vga_buf!(b' ',0x1,0x1), vga_buf!(            ), vga_buf!(            ), vga_buf!(b' ',0x1,0x1), vga_buf!(b' ',0x1,0x1), vga_buf!(), vga_buf!(b' ',0x1,0x1), vga_buf!(b' ',0x1,0x1), vga_buf!(b' ',0x1,0x1), vga_buf!(b' ',0x1,0x1), vga_buf!(b' ',0x1,0x1), vga_buf!(), vga_buf!(b' ',0x1,0x1), vga_buf!(b' ',0x1,0x1), vga_buf!(b' ',0x1,0x1), vga_buf!(b' ',0x1,0x1), vga_buf!(b' ',0x1,0x1), vga_buf!(), vga_buf!(b' ',0x1,0x1), vga_buf!(            ), vga_buf!(            ), vga_buf!(            ), vga_buf!(b' ',0x1,0x1), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!(), vga_buf!()],
            [vga_buf!(); 80],
            [vga_buf!(); 80],
            [vga_buf!(); 80],
            [vga_buf!(); 80],
            [vga_buf!(); 80],
            [vga_buf!(); 80],
            [vga_buf!(); 80],
            [vga_buf!(); 80],
            [vga_buf!(); 80],
            [vga_buf!(); 80],
            [vga_buf!(); 80],
        ]
    };
}