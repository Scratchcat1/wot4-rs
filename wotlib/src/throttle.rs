use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::PwmPin;

pub trait Throttle {
    fn arm(&mut self, delay: &mut dyn DelayMs<u16>);
    fn disarm(&mut self);
    fn off(&mut self);
    fn set(&mut self, duty: u8);
}

pub struct PwmThrottle<'a> {
    pub pin: &'a mut dyn PwmPin<Duty = u16>,
    pub neutral: u16,
    pub max: u16,
}

impl<'a> PwmThrottle<'a> {
    #[cfg(test)]
    fn get_duty(&self) -> u16 {
        self.pin.get_duty()
    }
}

impl<'a> Throttle for PwmThrottle<'a> {
    fn arm(&mut self, delay: &mut dyn DelayMs<u16>) {
        // (2100..u16::MAX).step_by(10).for_each(|duty| {
        //     self.pin.set_duty(duty);
        //     delay.delay_ms(50);
        // });
        self.pin.set_duty(self.neutral);
        delay.delay_ms(50);
        self.off()
    }

    fn disarm(&mut self) {
        self.pin.set_duty(0);
    }

    fn off(&mut self) {
        self.pin.set_duty(self.neutral);
    }

    fn set(&mut self, duty: u8) {
        let range = self.max - self.neutral;
        let pos = ((u32::from(duty) * u32::from(range)) / 256) + u32::from(self.neutral);
        self.pin.set_duty(pos as u16);
    }
}

#[cfg(test)]
mod test {
    use embedded_hal::PwmPin;

    use crate::test_util::fake_pwm_pin::FakePwmPin;

    use super::{PwmThrottle, Throttle};

    #[test]
    fn off_sets_duty_to_neutral() {
        let mut fake_pin = FakePwmPin::default();
        let mut throttle = PwmThrottle {
            pin: &mut fake_pin,
            neutral: 2500,
            max: 7500,
        };

        throttle.off();

        assert_eq!(throttle.get_duty(), 2500);
    }

    #[test]
    fn sets_min_and_max() {
        let mut fake_pin = FakePwmPin::default();
        let mut thottle = PwmThrottle {
            pin: &mut fake_pin,
            neutral: 2500,
            max: 7500,
        };

        thottle.set(0);
        assert_eq!(thottle.get_duty(), 2500);

        thottle.set(64);
        assert_eq!(thottle.get_duty(), 3750);

        thottle.set(128);
        assert_eq!(thottle.get_duty(), 5000);

        thottle.set(192);
        assert_eq!(thottle.get_duty(), 6250);

        thottle.set(u8::MAX);
        assert_eq!(thottle.get_duty(), 7480);
    }

    #[test]
    fn disarm_sets_duty_to_zero() {
        let mut fake_pin = FakePwmPin::default();
        let mut thottle = PwmThrottle {
            pin: &mut fake_pin,
            neutral: 2500,
            max: 7500,
        };

        thottle.disarm();
        assert_eq!(thottle.get_duty(), 0);
    }
}
