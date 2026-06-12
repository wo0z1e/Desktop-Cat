use crate::windows_api::log_diagnostic;
use std::time::Duration;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum CatState {
    Idle,
    IdleToSleeping,
    Sleeping,
    SleepingToIdle,
    WalkingLeft,
    WalkingRight,
}

pub struct BehaviorBrain {
    pub current_state: CatState,
    pub event_number: u32,
    pub frame_delay: Duration,
}

impl BehaviorBrain {
    pub fn new() -> Self {
        Self {
            current_state: CatState::Idle,
            event_number: 1,
            frame_delay: Duration::from_millis(400),
        }
    }

    pub fn interpret_new_event(&mut self) {
        let idle_pool = 1..=11;
        let walk_left_pool = 13..=15;
        let walk_right_pool = 16..=18;
        let sleep_pool = 19..=25;

        let previous_state = self.current_state;

        if idle_pool.contains(&self.event_number) {
            self.current_state = CatState::Idle;
            self.frame_delay = Duration::from_millis(400);
        } else if self.event_number == 12 {
            self.current_state = CatState::IdleToSleeping;
            self.frame_delay = Duration::from_millis(100);
        } else if walk_left_pool.contains(&self.event_number) {
            self.current_state = CatState::WalkingLeft;
            self.frame_delay = Duration::from_millis(100);
        } else if walk_right_pool.contains(&self.event_number) {
            self.current_state = CatState::WalkingRight;
            self.frame_delay = Duration::from_millis(100);
        } else if sleep_pool.contains(&self.event_number) {
            self.current_state = CatState::Sleeping;
            self.frame_delay = Duration::from_millis(400);
        } else if self.event_number == 26 {
            self.current_state = CatState::SleepingToIdle;
            self.frame_delay = Duration::from_millis(100);
        }

        if previous_state != self.current_state {
            log_diagnostic(
                "INFO",
                &format!("AI brain shifted state: {:?} -> {:?}", previous_state, self.current_state)
            );
        }
    }

    pub fn tick_roaming(&mut self, x: &mut f32, screen_w: f32, scale: f32) -> bool {
        let mut location_modified = false;
        let roam_speed = 4.0 * scale;
        let left_edge_limit = 100.0 * scale;
        let right_edge_limit = screen_w - (100.0 * scale);

        match self.current_state {
            CatState::WalkingLeft => {
                if *x > left_edge_limit {
                    *x -= roam_speed;
                    location_modified = true;
                } else {
                    self.current_state = CatState::WalkingRight;
                    log_diagnostic("DEBUG", "Left taskbar edge reached. Turning right.");
                }
            }
            CatState::WalkingRight => {
                if *x < right_edge_limit {
                    *x += roam_speed;
                    location_modified = true;
                } else {
                    self.current_state = CatState::WalkingLeft;
                    log_diagnostic("DEBUG", "Right system tray edge reached. Turning left.");
                }
            }
            _ => {} 
        }

        location_modified
    }
}