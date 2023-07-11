use crate::math::units::time::Second;
use std::any::Any;

#[derive(Debug, Clone, Copy)]
pub enum DebounceType {
    Rising,
    Falling,
    Both,
}

#[derive(Debug, Clone, Copy)]
pub struct Debouncer {
    debounce_time: Second,
    previous_time: Second,
    debounce_type: DebounceType,
    base_value: bool,
}

impl Debouncer {
    #[must_use]
    pub fn new(debounce_time: Second, debounce_type: DebounceType, base_value: bool) -> Self {
        Self {
            debounce_time,
            previous_time: Second::new(0.0),
            debounce_type,
            base_value,
        }
    }

    pub fn reset_timer(&mut self) {
        self.previous_time = Second::new(0.0);
    }

    pub fn calculate(&mut self, input: bool) -> bool {
        if input == self.base_value {
            self.reset_timer();
        }
        if self.has_elapsed() {
            if self.debounce_type.type_id() == DebounceType::Both.type_id() {
                self.base_value = input;
                self.reset_timer();
            }
            input
        } else {
            self.base_value
        }
    }

    pub fn reset(&mut self, value: bool, current_time: Second) {
        self.base_value = value;
        self.previous_time = current_time;
    }

    fn has_elapsed(&self) -> bool {
        Second::new(0.0) - self.previous_time >= self.debounce_time
    }
}
