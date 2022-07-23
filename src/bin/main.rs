#![no_std]
#![no_main]

use arduino_hal::spi;
use panic_halt as _;
use rand::SeedableRng;
use ws2812::Ws2812;
use ws2812_spi as ws2812;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let (spi, _) = spi::Spi::new(
        dp.SPI,
        pins.d13.into_output(),
        pins.d11.into_output(),
        pins.d12.into_pull_up_input(),
        pins.d10.into_output(),
        Default::default(),
    );

    // initialize rng with seed from A0
    let rng_seed_pin = pins.a0.into_analog_input(&mut adc);
    let mut seed = 0u64;
    for i in 0..4 {
        seed |= (rng_seed_pin.analog_read(&mut adc) as u64) << (i * 16);
    }
    let mut rng = rand::rngs::SmallRng::seed_from_u64(seed);

    let ws = Ws2812::new(spi);
    let mut strip = ledsort::RainbowSortStrip::new(ws, serial);
    strip.fill_rainbow_effect();
    arduino_hal::delay_ms(1000);

    loop {
        strip.shuffle_effect(&mut rng, 10);
        arduino_hal::delay_ms(1000);
        strip.quicksort_effect(50);
        arduino_hal::delay_ms(3000);
        strip.shuffle_effect(&mut rng, 10);
        arduino_hal::delay_ms(1000);
        strip.selection_sort_effect(10);
        arduino_hal::delay_ms(3000);
        strip.shuffle_effect(&mut rng, 10);
        arduino_hal::delay_ms(1000);
        strip.bubblesort_effect(10);
        arduino_hal::delay_ms(3000);
    }
}
