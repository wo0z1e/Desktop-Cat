use crate::windows_api::log_diagnostic;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum IntroPhase {
    WalkingOutLeft,    
    Teleporting,       
    WalkingInRight,    
    Done,              
}

pub struct IntroSequence {
    pub current_phase: IntroPhase,
}

impl IntroSequence {
    pub fn new() -> Self {
        Self {
            current_phase: IntroPhase::WalkingOutLeft,
        }
    }

    pub fn tick_sequence(
        &mut self, 
        x: &mut f32, 
        y: &mut f32, 
        screen_w: f32, 
        screen_h: f32,
        scale: f32,
        mut animate_frame: impl FnMut()
    ) -> bool {
        let mut position_modified = false;

        // Normalize baseline structural speeds based on display density scale factor
        let base_exit_speed = 5.0 * scale;
        let base_entry_speed = 4.0 * scale;
        let left_out_threshold = -80.0;
        let right_arrival_threshold = screen_w - 150.0;

        match self.current_phase {
            IntroPhase::WalkingOutLeft => {
                *x -= base_exit_speed;
                position_modified = true;
                animate_frame();

                if *x < left_out_threshold {
                    log_diagnostic("INFO", "Intro script reached left boundary edge. Initiating teleport phase.");
                    self.current_phase = IntroPhase::Teleporting;
                }
            }
            IntroPhase::Teleporting => {
                *x = screen_w + (10.0 * scale);
                *y = screen_h - (64.0 * scale); 
                position_modified = true;

                log_diagnostic("INFO", &format!("Teleported successfully to hidden staging zone: ({:.1}, {:.1})", *x, *y));
                self.current_phase = IntroPhase::WalkingInRight;
            }
            IntroPhase::WalkingInRight => {
                *x -= base_entry_speed;
                position_modified = true;
                animate_frame();

                if *x <= right_arrival_threshold {
                    log_diagnostic("INFO", "Intro cinematic sequence completed safely. Initializing active AI.");
                    self.current_phase = IntroPhase::Done;
                }
            }
            IntroPhase::Done => {}
        }

        position_modified
    }
}