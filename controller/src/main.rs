//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

use bsp::entry;
use bsp::hal::{
    clocks::{Clock, init_clocks_and_plls},
    pac, pwm,
    sio::Sio,
    watchdog::Watchdog,
};
use panic_probe as _;
use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::v2::OutputPin;
// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use rp_pico as bsp;

use wotlib::servo::{PwmServo, Servo};
use wotlib::throttle::PwmThrottle;

// use sparkfun_pro_micro_rp2040 as bsp;

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );
    // Init PWMs
    let mut pwm_slices = pwm::Slices::new(pac.PWM, &mut pac.RESETS);

    // Configure PWM4
    let pwm0 = &mut pwm_slices.pwm0;
    pwm0.set_ph_correct();
    pwm0.set_div_int(20u8); // 50 hz
    pwm0.enable();

    let pwm1 = &mut pwm_slices.pwm1;
    pwm1.set_ph_correct();
    pwm1.set_div_int(20u8); // 50 hz
    pwm1.enable();

    // Output channel B on PWM4 to GPIO 25
    let aileron_channel = &mut pwm0.channel_a;
    aileron_channel.output_to(pins.gpio16);
    let ailerons = PwmServo {
        min: 3600,
        max: 5900,
        pin: aileron_channel,
    };

    let elevator_channel = &mut pwm0.channel_b;
    elevator_channel.output_to(pins.gpio17);
    let elevator = PwmServo {
        min: 3700,
        max: 6400,
        pin: elevator_channel,
    };

    let rudder_channel = &mut pwm1.channel_a;
    rudder_channel.output_to(pins.gpio18);
    let rudder = PwmServo {
        min: 3600,
        max: 4900,
        pin: rudder_channel,
    };

    let throttle_channel = &mut pwm1.channel_b;
    throttle_channel.output_to(pins.gpio19);
    let throttle = PwmThrottle {
        arm_min: 6500,
        arm_max: 7000,
        pin: throttle_channel,
    };

    let mut led_pin = pins.led.into_push_pull_output();

    let start_up_delay_ms = 1000;
    let step_size = 1;

    [elevator].iter_mut().for_each(|servo| {
        servo.center();
        delay.delay_ms(start_up_delay_ms);
        servo.center();
        delay.delay_ms(start_up_delay_ms);
        servo.set_pos(i8::MIN);
        delay.delay_ms(start_up_delay_ms);
        servo.center();
        delay.delay_ms(start_up_delay_ms);
        servo.set_pos(i8::MAX);
        delay.delay_ms(start_up_delay_ms);
        servo.center();
        delay.delay_ms(start_up_delay_ms);
        led_pin.set_high().unwrap();
        delay.delay_ms(start_up_delay_ms);
        led_pin.set_low().unwrap();

        for i in (i8::MIN..=i8::MAX)
            .step_by(step_size)
            .chain((i8::MIN..=i8::MAX).step_by(step_size).rev())
        {
            led_pin.set_high().unwrap();
            servo.set_pos(i);
            delay.delay_ms(10);

            led_pin.set_low().unwrap();
            delay.delay_ms(10);
        }
        servo.center();
    });

    loop {
        delay.delay_ms(start_up_delay_ms);
        led_pin.set_high().unwrap();
        delay.delay_ms(start_up_delay_ms);
        led_pin.set_low().unwrap();
    }
}

// End of file
