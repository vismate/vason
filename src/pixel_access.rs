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

pub struct AlphaAccess;
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

impl AlphaAccess {
    #[inline(always)]
    fn blend(c1: &mut u32, c2: u32) {
        let [b1, g1, r1, a1] = u32::to_le_bytes(*c1);
        let [b2, g2, r2, a2] = u32::to_le_bytes(c2);

        let (r1, g1, b1, a1) = (r1 as u32, g1 as u32, b1 as u32, a1 as u32);
        let (r2, g2, b2, a2) = (r2 as u32, g2 as u32, b2 as u32, a2 as u32);

        let r = ((r1 * (255 - a2) + r2 * a2) / 255).min(255) as u8;
        let g = ((g1 * (255 - a2) + g2 * a2) / 255).min(255) as u8;
        let b = ((b1 * (255 - a2) + b2 * a2) / 255).min(255) as u8;
        let a = a1 as u8;

        *c1 = u32::from_le_bytes([b, g, r, a]);
    }
}

impl PixelAccess for AlphaAccess {
    #[inline]
    unsafe fn set_pixel_unchecked(
        buffer: &mut [u32],
        x: usize,
        y: usize,
        stride: usize,
        raw_color: u32,
    ) {
        Self::blend(buffer.get_unchecked_mut(y * stride + x), raw_color);
    }

    #[inline]
    fn fill(buffer: &mut [u32], raw_color: u32) {
        for c in buffer {
            Self::blend(c, raw_color);
        }
    }
}
