type TimerCallback = fn();

pub struct Timer {
    pub timer_value: u8,
    is_timer_active: bool,
    timer_callback: TimerCallback,
}

impl Timer {
    pub fn new(user_callback: TimerCallback) -> Timer {
        Timer { 
            timer_value: 0,
            is_timer_active: false,
            timer_callback: user_callback,
        }
    }

    fn no_callback_function() {}

    pub fn new_without_callback() -> Timer {
        Timer::new(Timer::no_callback_function)
    }

    pub fn tick(&mut self) {
        if self.is_timer_active {
            self.timer_value -= 1;
        }

        if self.timer_value > 0 && !self.is_timer_active {
            self.is_timer_active = true;
            self.timer_value -= 1;
            let user_callback = self.timer_callback;
            user_callback();
        }

        if self.timer_value == 0 {
            self.is_timer_active = false;
        }
    }
}