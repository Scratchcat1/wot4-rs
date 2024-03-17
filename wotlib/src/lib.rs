#![cfg_attr(not(test), no_std)]
pub mod servo;
pub mod throttle;

#[cfg(test)]
pub mod test_util {
    pub mod fake_pwm_pin;
}
