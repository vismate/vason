pub trait PixelAccess {
    unsafe fn set_pixel_unchecked(
        buffer: &mut [u32],
        x: usize,
        y: usize,
        stride: usize,
        raw_color: u32,
    );
    fn fill(buffer: &mut [u32], raw_color: u32);
}

pub struct AlpaAccess;
pub struct NoAlphaAccess;

impl PixelAccess for NoAlphaAccess {
    #[inline(always)]
    unsafe fn set_pixel_unchecked(
        buffer: &mut [u32],
        x: usize,
        y: usize,
        stride: usize,
        raw_color: u32,
    ) {
        *buffer.get_unchecked_mut(y * stride + x) = raw_color;
    }

    #[inline(always)]
    fn fill(buffer: &mut [u32], raw_color: u32) {
        buffer.fill(raw_color);
    }
}
