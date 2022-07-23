#![no_std]

use core::fmt::Debug;
use core::mem;
use core::ops::Range;
use rand::Rng;
use smart_leds::hsv::Hsv;
use smart_leds::{hsv, SmartLedsWrite, RGB8 as Rgb8};
use ufmt::uWrite;

const NUM_LEDS: usize = 60;
const NORMAL_BRIGHTNESS: u8 = 0xA0;
const FOCUS_COLOR: Rgb8 = Rgb8::new(0xFF, 0xFF, 0xFF);
const SATURATION: u8 = 0xFF;

#[derive(Default, Copy, Clone)]
pub struct Pixel {
    hue: u8,
    led: Rgb8,
}

impl Pixel {
    fn set(&mut self, hue: u8, sat: u8, val: u8) {
        self.hue = hue;
        self.led = hsv::hsv2rgb(Hsv {
            hue,
            sat,
            val
        })
    }
}

pub struct RainbowSortStrip<W, U> {
    pub pixels: [Pixel; NUM_LEDS],
    write: W,
    serial: U
}

impl<W, U> RainbowSortStrip<W, U> where W: SmartLedsWrite, <W as SmartLedsWrite>::Color: From<Rgb8>, <W as SmartLedsWrite>::Error: Debug, U: uWrite {
    pub fn new(write: W, serial: U) -> Self {
        Self {
            pixels: [Pixel::default(); NUM_LEDS],
            write,
            serial
        }
    }

    fn write_pixels(&mut self) {
        self.write.write(smart_leds::gamma(self.pixels.iter().map(|p| p.led))).unwrap()
    }

    pub fn fill_rainbow_effect(&mut self) {
        const HUE_FACTOR: f32 = u8::MAX as f32 / NUM_LEDS as f32;
        for (index, pixel) in self.pixels.iter_mut().enumerate() {
            pixel.set((HUE_FACTOR * index as f32) as u8, SATURATION, NORMAL_BRIGHTNESS);
        }
        self.write_pixels();
    }

    pub fn shuffle_effect(&mut self, rng: &mut impl Rng, delay_ms: u16) {
        for _ in 0..NUM_LEDS {
            for _ in 0..8 {
                self.pixels.swap(rng.gen_range(0..NUM_LEDS), rng.gen_range(0..NUM_LEDS))
            }
            self.write_pixels();
            arduino_hal::delay_ms(delay_ms);
        }
    }

    fn focus<const COUNT: usize>(&mut self, indexes: [usize; COUNT], delay_ms: u16) {
        let mut old_leds = [FOCUS_COLOR; COUNT];
        for (old_leds_index, index) in indexes.iter().copied().enumerate() {
            if self.pixels[index].led != FOCUS_COLOR {
                mem::swap(&mut self.pixels[index].led, &mut old_leds[old_leds_index]);
            }
        }

        self.write_pixels();
        arduino_hal::delay_ms(delay_ms);

        for (old_leds_index, index) in indexes.iter().copied().enumerate() {
            if self.pixels[index].led == FOCUS_COLOR {
                mem::swap(&mut self.pixels[index].led, &mut old_leds[old_leds_index]);
            }
        }
    }

    pub fn bubblesort_effect(&mut self, delay_ms: u16) {
        for len in (2..=NUM_LEDS).rev() {
            for index in 0..len - 1 {
                if self.pixels[index].hue > self.pixels[index + 1].hue {
                    self.pixels.swap(index, index + 1)
                }
                self.focus([index, index + 1], delay_ms);
            }
        }
        self.write_pixels();
    }

    pub fn quicksort_effect(&mut self, delay_ms: u16) {
        self.quicksort_effect_inner(0..self.pixels.len(), delay_ms);
        self.write_pixels();
    }

    fn quicksort_effect_inner(&mut self, range: Range<usize>, delay_ms: u16) {
        if range.len() < 2 {
            return;
        }

        let pivot_i = range.end - 1;
        let mut greater_start = range.start;

        for candidate_i in range.start..pivot_i {
            self.focus([pivot_i, candidate_i, greater_start], delay_ms);
            if self.pixels[pivot_i].hue > self.pixels[candidate_i].hue {
                self.pixels.swap(candidate_i, greater_start);
                greater_start += 1;
            }
        }

        self.pixels.swap(greater_start, pivot_i);

        self.quicksort_effect_inner(range.start..greater_start, delay_ms);
        self.quicksort_effect_inner((greater_start+1)..range.end, delay_ms);
    }

    pub fn selection_sort_effect(&mut self, delay_ms: u16) {
        for i in 0..self.pixels.len()-1 {
            let mut j_min = i;
            for j in i+1..self.pixels.len() {
                self.focus([i, j_min, j], delay_ms);
                if self.pixels[j].hue < self.pixels[j_min].hue {
                    j_min = j;
                }
            }
            if j_min != i {
                self.pixels.swap(i, j_min)
            }
        }
        self.write_pixels();
    }
}

/*
pub fn split_and_reverse_effect<W>(leds: &mut [Rgb8], write: &mut W, delay_ms: u16) where W: SmartLedsWrite, <W as SmartLedsWrite>::Color: From<Rgb8>, <W as SmartLedsWrite>::Error: Debug {
    if leds.len() <= 1 {
        return;
    }

    // reverse
    for i in 0..leds.len() / 2 {
        leds.swap(i, leds.len() - 1 - i);
        write.
        arduino_hal::delay_ms(delay_ms);
    }

    // recurse
    let (leds_left, leds_not_left) = pixels.split_at_mut(pixels.len() / 2);
    let leds_right = &mut leds_not_left[1..];
    split_and_reverse_effect(leds_left, write, delay_ms);
    split_and_reverse_effect(leds_right, write, delay_ms);

    // unreverse
    for i in 0..leds.len() / 2 {
        leds.swap(i, leds.len() - 1 - i);
        self.write_pixels();
        arduino_hal::delay_ms(delay_ms);
    }
}
*/