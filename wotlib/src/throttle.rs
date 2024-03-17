use embedded_hal::PwmPin;

pub trait Throttle {
    fn arm(&mut self);
    fn off(&mut self);
    fn set(&mut self, duty: u8);
}

pub struct PwmThrottle<'a> {
    pub pin: &'a mut dyn PwmPin<Duty = u16>,
    pub arm_min: u16,
    pub arm_max: u16,
}

impl<'a> Throttle for PwmThrottle<'a> {
    // fn set_pos(&mut self, pos: i8) {
    //     let offset_pos = (pos + 127) as u8;
    //     let range = self.max - self.min;
    //     let pos = ((offset_pos as u32) * (range as u32)) + self.min as u32;
    //     self.pin.set_duty(pos as u16);
    // }

    // fn center(&mut self) {
    //     let mid = (self.max + self.min) / 2;
    //     self.pin.set_duty(mid);
    // }

    fn arm(&mut self) {
        self.pin.set_duty(self.arm_min);
        // delay.delay_ms(50);
        self.pin.set_duty(self.arm_max);
        // delay.delay_ms(50);
        self.pin.set_duty(self.arm_min);
        // delay.delay_ms(50);
        self.off()
    }

    fn off(&mut self) {
        self.pin.set_duty(0);
    }

    fn set(&mut self, duty: u8) {
        let scaled_duty = (u16::MAX as u32 * duty as u32) / (u8::MAX as u32);
        self.pin.set_duty(scaled_duty as u16);
    }
}
