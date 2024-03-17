use embedded_hal::PwmPin;

#[derive(Default)]
pub struct FakePwmPin {
    current_duty: u16,
}

impl PwmPin for FakePwmPin {
    type Duty = u16;

    fn disable(&mut self) {
        todo!()
    }

    fn enable(&mut self) {
        todo!()
    }

    fn get_duty(&self) -> Self::Duty {
        self.current_duty
    }

    fn get_max_duty(&self) -> Self::Duty {
        todo!()
    }

    fn set_duty(&mut self, duty: Self::Duty) {
        self.current_duty = duty;
    }
}
