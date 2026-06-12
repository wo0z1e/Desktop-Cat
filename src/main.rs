use eframe::egui;
use std::time::{Duration, Instant};

// Registering all of our decoupled architecture modules
pub mod ai;
pub mod diagnostics;
pub mod intro;
pub mod windows_api;

fn main() -> Result<(), eframe::Error> {
    windows_api::log_diagnostic("INFO", "Starting master framework compilation sequence...");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false)        // Borderless/No title bar
            .with_transparent(true)         // Request native OS transparency
            .with_always_on_top()           // Keep above all desktop windows
            .with_inner_size([72.0, 64.0]), // Let egui scale this baseline logically
        ..Default::default()
    };

    eframe::run_native(
        "Rust Desktop Cat",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(Ket::new(cc))
        }),
    )
}

struct Ket {
    idle: Vec<egui::ImageSource<'static>>,
    idle_to_sleeping: Vec<egui::ImageSource<'static>>,
    sleeping: Vec<egui::ImageSource<'static>>,
    sleeping_to_idle: Vec<egui::ImageSource<'static>>,
    walking_left: Vec<egui::ImageSource<'static>>,
    walking_right: Vec<egui::ImageSource<'static>>,

    x: f32,
    y: f32,
    screen_width: f32,
    screen_height: f32,
    scale_factor: f32,
    i_frame: usize,
    last_update: Instant,

    intro_engine: intro::IntroSequence,
    ai_brain: ai::BehaviorBrain,

    // Performance Audit State Tracking Variables
    resource_monitor: diagnostics::ResourceMonitor,
    last_perf_sample: Instant,
}

impl Ket {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let metrics = windows_api::fetch_desktop_metrics();

        let idle = vec![
            egui::include_image!("../assets/idle1.png"),
            egui::include_image!("../assets/idle2.png"),
            egui::include_image!("../assets/idle3.png"),
            egui::include_image!("../assets/idle4.png"),
        ];
        let idle_to_sleeping = vec![
            egui::include_image!("../assets/sleeping1.png"),
            egui::include_image!("../assets/sleeping2.png"),
            egui::include_image!("../assets/sleeping3.png"),
            egui::include_image!("../assets/sleeping4.png"),
            egui::include_image!("../assets/sleeping5.png"),
            egui::include_image!("../assets/sleeping6.png"),
        ];
        let sleeping = vec![
            egui::include_image!("../assets/zzz1.png"),
            egui::include_image!("../assets/zzz2.png"),
            egui::include_image!("../assets/zzz3.png"),
            egui::include_image!("../assets/zzz4.png"),
        ];
        let sleeping_to_idle = vec![
            egui::include_image!("../assets/sleeping6.png"),
            egui::include_image!("../assets/sleeping5.png"),
            egui::include_image!("../assets/sleeping4.png"),
            egui::include_image!("../assets/sleeping3.png"),
            egui::include_image!("../assets/sleeping2.png"),
            egui::include_image!("../assets/sleeping1.png"),
        ];
        let walking_left = vec![
            egui::include_image!("../assets/walkingleft1.png"),
            egui::include_image!("../assets/walkingleft2.png"),
            egui::include_image!("../assets/walkingleft3.png"),
            egui::include_image!("../assets/walkingleft4.png"),
        ];
        let walking_right = vec![
            egui::include_image!("../assets/walkingright1.png"),
            egui::include_image!("../assets/walkingright2.png"),
            egui::include_image!("../assets/walkingright3.png"),
            egui::include_image!("../assets/walkingright4.png"),
        ];

        Self {
            idle,
            idle_to_sleeping,
            sleeping,
            sleeping_to_idle,
            walking_left,
            walking_right,
            x: 0.0,
            y: 0.0,
            screen_width: metrics.work_width,
            screen_height: metrics.work_height,
            scale_factor: metrics.scale_factor,
            i_frame: 0,
            last_update: Instant::now(),
            intro_engine: intro::IntroSequence::new(),
            ai_brain: ai::BehaviorBrain::new(),
            resource_monitor: diagnostics::ResourceMonitor::new(),
            last_perf_sample: Instant::now(),
        }
    }

    fn advance_animation_frame(&mut self, total_frames: usize, min_roll: u32, max_roll: u32) {
        if self.i_frame < total_frames - 1 {
            self.i_frame += 1;
        } else {
            self.i_frame = 0;
            self.ai_brain.event_number = rand_range(min_roll, max_roll);
        }
    }
}

impl eframe::App for Ket {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Trigger our performance monitoring loop exactly once every 5 seconds
        if self.last_perf_sample.elapsed() >= Duration::from_secs(5) {
            self.resource_monitor.sample_usage();
            self.last_perf_sample = Instant::now();
        }

        let current_delay = if self.intro_engine.current_phase != intro::IntroPhase::Done {
            Duration::from_millis(100)
        } else {
            self.ai_brain.frame_delay
        };

        if self.last_update.elapsed() >= current_delay {
            let coordinates_changed = if self.intro_engine.current_phase != intro::IntroPhase::Done {
                let mut animate_hook = || {
                    let total_walk_frames = self.walking_left.len();
                    if self.i_frame < total_walk_frames - 1 {
                        self.i_frame += 1;
                    } else {
                        self.i_frame = 0;
                    }
                };

                self.intro_engine.tick_sequence(
                    &mut self.x,
                    &mut self.y,
                    self.screen_width,
                    self.screen_height,
                    self.scale_factor,
                    &mut animate_hook,
                )
            } else {
                self.ai_brain.interpret_new_event();
                
                match self.ai_brain.current_state {
                    ai::CatState::Idle => { let total = self.idle.len(); self.advance_animation_frame(total, 1, 18); }
                    ai::CatState::IdleToSleeping => { let total = self.idle_to_sleeping.len(); self.advance_animation_frame(total, 19, 19); }
                    ai::CatState::Sleeping => { let total = self.sleeping.len(); self.advance_animation_frame(total, 19, 26); }
                    ai::CatState::SleepingToIdle => { let total = self.sleeping_to_idle.len(); self.advance_animation_frame(total, 1, 1); }
                    ai::CatState::WalkingLeft => { let total = self.walking_left.len(); self.advance_animation_frame(total, 1, 18); }
                    ai::CatState::WalkingRight => { let total = self.walking_right.len(); self.advance_animation_frame(total, 1, 18); }
                }

                self.ai_brain.tick_roaming(&mut self.x, self.screen_width, self.scale_factor)
            };

            if coordinates_changed {
                ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(egui::pos2(self.x, self.y)));
            }
            self.last_update = Instant::now();
        }

        let current_texture = if self.intro_engine.current_phase != intro::IntroPhase::Done {
            &self.walking_left[self.i_frame]
        } else {
            match self.ai_brain.current_state {
                ai::CatState::Idle => &self.idle[self.i_frame],
                ai::CatState::IdleToSleeping => &self.idle_to_sleeping[self.i_frame],
                ai::CatState::Sleeping => &self.sleeping[self.i_frame],
                ai::CatState::SleepingToIdle => &self.sleeping_to_idle[self.i_frame],
                ai::CatState::WalkingLeft => &self.walking_left[self.i_frame],
                ai::CatState::WalkingRight => &self.walking_right[self.i_frame],
            }
        };

        let panel_frame = egui::Frame::none().fill(egui::Color32::TRANSPARENT);

        egui::CentralPanel::default()
            .frame(panel_frame)
            .show(ctx, |ui| {
                ui.centered_and_justified(|ui| {
                    ui.add(egui::Image::new(current_texture.clone()).max_width(72.0));
                });
            });

        // ctx.request_repaint();
        ctx.request_repaint_after(current_delay);
    }
}

fn rand_range(min: u32, max: u32) -> u32 {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};
    let mut hasher = RandomState::new().build_hasher();
    hasher.write_u128(Instant::now().elapsed().as_nanos());
    let raw = hasher.finish() as u32;
    (raw % (max - min + 1)) + min
}