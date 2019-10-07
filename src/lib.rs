trait ICarAlarmSystem {
    fn opened(&self) -> bool;
    fn closed(&self) -> bool;
    fn locked(&self) -> bool;
    fn unlocked(&self) -> bool;
    fn flash(&self) -> bool;
    fn sound(&self) -> bool;
    fn armed(&self) -> bool;

    fn lock(&mut self);
    fn unlock(&mut self);
    fn close(&mut self);
    fn open(&mut self);
    fn tick(&mut self);
}

#[allow(dead_code)]
struct CarAlarmSystem {
    open: bool,
    locked: bool,
    flash: bool,
    sound: bool,
    armed: bool,
    clock: u128,
}

#[allow(dead_code)]
impl CarAlarmSystem {
    fn new() -> Self {
        CarAlarmSystem {
            open: false,
            locked: false,
            flash: false,
            sound: false,
            armed: false,
            clock: 0,
        }
    }
}

impl ICarAlarmSystem for CarAlarmSystem {
    fn opened(&self) -> bool {
        self.open
    }

    fn closed(&self) -> bool {
        !self.open
    }

    fn locked(&self) -> bool {
        self.locked
    }

    fn unlocked(&self) -> bool {
        !self.locked
    }
    fn flash(&self) -> bool {
        self.flash
    }
    fn sound(&self) -> bool {
        self.sound
    }
    fn armed(&self) -> bool {
        self.armed
    }

    fn lock(&mut self) {
        if self.closed() {
            self.locked = true;
        }
    }

    fn unlock(&mut self) {
        self.clock = 0;
        if self.armed {
            self.armed = false;
            self.flash = false;
            self.sound = false;
        }
    }

    fn close(&mut self) {
        self.open = false;
    }

    fn open(&mut self) {
        self.open = true;

        if self.armed {
            self.clock = 0;
            self.flash = true;
            self.sound = true;
        }
    }

    fn tick(&mut self) {
        self.clock += 1;

        // Arm the car after two seconds
        if self.clock == 2 && !self.open && self.locked && !self.armed {
            self.armed = true;
        }

        if self.clock == 3 && self.sound {
            self.sound = false;
        }

        if self.clock == 30 && self.flash {
            self.flash = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lock_when_open_is_true() {
        let mut cas = CarAlarmSystem::new();
        cas.open = true;
        cas.lock();
        assert!(cas.unlocked());
    }

    #[test]
    fn test_lock_when_open_is_false() {
        let mut cas = CarAlarmSystem::new();
        cas.lock();
        assert!(cas.locked());
    }

    #[test]
    fn test_unlock_when_armed_is_false() {
        let mut cas = CarAlarmSystem::new();
        cas.unlock();
        assert!(!cas.flash() && !cas.sound() && !cas.armed() && cas.unlocked());
    }

    #[test]
    fn test_unlock_when_armed_is_true() {
        let mut cas = CarAlarmSystem::new();
        cas.flash = true;
        cas.sound = true;
        cas.armed = true;
        cas.unlock();
        assert!(!cas.flash() && !cas.sound() && !cas.armed() && cas.unlocked());
    }

    #[test]
    fn test_open_when_armed_is_false() {
        let mut cas = CarAlarmSystem::new();
        cas.open();
        assert!(!cas.flash() && !cas.sound() && !cas.armed() && cas.opened());
    }

    #[test]
    fn test_open_when_armed_is_true() {
        let mut cas = CarAlarmSystem::new();
        cas.armed = true;
        cas.open();
        assert!(cas.flash() && cas.sound() && cas.armed() && cas.opened());
    }

    #[test]
    fn test_tick_when_should_be_armed() {
        let mut cas = CarAlarmSystem::new();
        cas.clock = 1;
        cas.close();
        cas.lock();
        cas.armed = false;
        cas.tick();
        assert!(cas.armed());
    }

    #[test]
    fn test_tick_when_should_not_be_armed() {
        let mut cas = CarAlarmSystem::new();
        cas.tick();
        assert!(!cas.armed());
    }

    #[test]
    fn test_tick_when_should_turn_sound_off() {
        let mut cas = CarAlarmSystem::new();
        cas.clock = 2;
        cas.sound = true;
        cas.tick();
        assert!(!cas.sound());
    }

    #[test]
    fn test_tick_when_should_sound_stays_on() {
        let mut cas = CarAlarmSystem::new();
        cas.sound = true;
        cas.tick();
        assert!(cas.sound());
    }

    #[test]
    fn test_tick_when_should_turn_flash_off() {
        let mut cas = CarAlarmSystem::new();
        cas.clock = 29;
        cas.flash = true;
        cas.tick();
        assert!(!cas.flash());
    }

    #[test]
    fn test_tick_when_flash_stays_on() {
        let mut cas = CarAlarmSystem::new();
        cas.flash = true;
        cas.tick();
        assert!(cas.flash());
    }
}
