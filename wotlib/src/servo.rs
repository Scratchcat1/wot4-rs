use embedded_hal::PwmPin;

pub trait Servo {
    fn set_pos(&mut self, pos: i8);
    fn center(&mut self);
}

pub struct PwmServo<'a> {
    pub pin: &'a mut dyn PwmPin<Duty = u16>,
    pub min: u16,
    pub max: u16,
}

impl<'a> PwmServo<'a> {
    #[cfg(test)]
    fn get_duty(&self) -> u16 {
        self.pin.get_duty()
    }
}

impl<'a> Servo for PwmServo<'a> {
    fn set_pos(&mut self, pos: i8) {
        let offset_pos = pos as i32 + 128;
        let range = self.max - self.min;
        let pos = (((offset_pos as u32) * (range as u32)) / 256) + self.min as u32;
        self.pin.set_duty(pos as u16);
    }

    fn center(&mut self) {
        let mid = (self.max + self.min) / 2;
        self.pin.set_duty(mid);
    }
}

#[cfg(test)]
mod test {
    use embedded_hal::PwmPin;

    use crate::test_util::fake_pwm_pin::FakePwmPin;

    use super::{PwmServo, Servo};

    #[test]
    fn center_sets_mid_point() {
        let mut fake_pin = FakePwmPin::default();
        let mut servo = PwmServo {
            pin: &mut fake_pin,
            min: 2500,
            max: 7500,
        };

        servo.center();

        assert_eq!(servo.get_duty(), 5000);
    }

    #[test]
    fn sets_min_and_max_positions() {
        let mut fake_pin = FakePwmPin::default();
        let mut servo = PwmServo {
            pin: &mut fake_pin,
            min: 2500,
            max: 7500,
        };

        servo.set_pos(i8::MIN);
        assert_eq!(servo.get_duty(), 2500);

        servo.set_pos(-64);
        assert_eq!(servo.get_duty(), 3750);

        servo.set_pos(0);
        assert_eq!(servo.get_duty(), 5000);

        servo.set_pos(64);
        assert_eq!(servo.get_duty(), 6250);

        servo.set_pos(i8::MAX);
        assert_eq!(servo.get_duty(), 7480);
    }

    #[test]
    fn sets_zero_if_min_is_zero() {
        let mut fake_pin = FakePwmPin::default();
        let mut servo = PwmServo {
            pin: &mut fake_pin,
            min: 0,
            max: 7500,
        };

        servo.set_pos(i8::MIN);
        assert_eq!(servo.get_duty(), 0);
    }
}
