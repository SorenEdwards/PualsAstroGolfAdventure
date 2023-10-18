use crate::common::*;
use crate::geo::*;
use crate::render::drawing::*;
use core::cmp::min;
use line_drawing::Bresenham;

pub(crate) fn background(frame: &mut [u8]) {
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let bg_color = [0x48, 0xb2, 0xe8, 0xff];
        pixel.copy_from_slice(&bg_color);
    }
}

pub(crate) fn map_color_to_rgba(items: &[u8]) -> Vec<u8> {
    let mut ret_vec: Vec<u8> = Vec::new();
    for item in items.iter() {
        match item {
            0 => {
                ret_vec.push(128);
                ret_vec.push(128);
                ret_vec.push(128);
                ret_vec.push(255);
            }
            1 => {
                ret_vec.push(255);
                ret_vec.push(255);
                ret_vec.push(255);
                ret_vec.push(255);
            }
            2 => {
                ret_vec.push(73);
                ret_vec.push(39);
                ret_vec.push(245);
                ret_vec.push(255);
            }
            3 => {
                ret_vec.push(0);
                ret_vec.push(0);
                ret_vec.push(0);
                ret_vec.push(255);
            }
            _ => {}
        }
    }
    return ret_vec;
}

pub(crate) fn blit<S>(screen: &mut [u8], dest: &Point, sprite: &S)
where
    S: Drawable,
{
    assert!(dest.x + sprite.width() <= WIDTH);
    assert!(dest.y + sprite.height() <= HEIGHT);

    let pixels = sprite.pixels();
    let width = sprite.width() * 4;

    let mut s = 0;
    for y in 0..sprite.height() {
        let i = dest.x * 4 + dest.y * WIDTH * 4 + y * WIDTH * 4;

        // Merge pixels from sprite into screen
        let zipped = screen[i..i + width].iter_mut().zip(&pixels[s..s + width]);
        for (left, &right) in zipped {
            if right > 0 {
                *left = right;
            }
        }

        s += width;
    }
}

pub(crate) fn line(screen: &mut [u8], p1: &Point, p2: &Point, color: [u8; 4]) {
    let p1 = (p1.x as i64, p1.y as i64);
    let p2 = (p2.x as i64, p2.y as i64);
    for (x, y) in Bresenham::new(p1, p2) {
        let x = min(x as usize, (WIDTH - 1) as usize);
        let y = min(y as usize, (HEIGHT - 1) as usize);

        let i = x * 4 + y * WIDTH as usize * 4;

        screen[i..i + 4].copy_from_slice(&color);
    }
}

pub(crate) fn rect(screen: &mut [u8], p1: &Point, p2: &Point, color: [u8; 4]) {
    let p2 = Point::new(p2.x - 1, p2.y - 1);
    let p3 = Point::new(p1.x, p2.y);
    let p4 = Point::new(p2.x, p1.y);

    line(screen, p1, &p3, color);
    line(screen, &p3, &p2, color);
    line(screen, &p2, &p4, color);
    line(screen, &p4, p1, color);
}
