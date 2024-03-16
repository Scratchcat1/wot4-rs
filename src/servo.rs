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
