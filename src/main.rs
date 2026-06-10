use eframe::egui;
use std::time::{Duration, Instant};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false)        // Borderless/No title bar
            .with_transparent(true)         // Native OS transparency
            .with_always_on_top()           // Keep above all apps
            .with_inner_size([72.0, 64.0]), // Window bounds
        ..Default::default()
    };

    eframe::run_native(
        "Rust Desktop Cat",
        options,
        Box::new(|cc| {
            // Initialize the image loaders so egui can read the compiled PNGs
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::new(Ket::new(cc))
        }),
    )
}

struct Ket {
    // Assets baked directly into the executable binary
    idle: Vec<egui::ImageSource<'static>>,
    idle_to_sleeping: Vec<egui::ImageSource<'static>>,
    sleeping: Vec<egui::ImageSource<'static>>,
    sleeping_to_idle: Vec<egui::ImageSource<'static>>,
    walking_left: Vec<egui::ImageSource<'static>>,
    walking_right: Vec<egui::ImageSource<'static>>,

    // Window and logic variables
    x: f32,
    y: f32,
    screen_width: f32,
    i_frame: usize,
    state: u8,
    event_number: u32,
    last_update: Instant,
    frame_delay: Duration,
}

impl Ket {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Find screen width & calculate taskbar coordinates natively via context
        let monitor_size = cc.egui_ctx.screen_rect();
        let screen_width = monitor_size.width();
        let work_height = monitor_size.height();

        // Match your starting location: 80% screen width, sitting right on top of taskbar
        let x = screen_width * 0.8;
        let y = work_height - 64.0;

        // Baking all asset frames directly into compilation memory
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
            x,
            y,
            screen_width,
            i_frame: 0,
            state: 1,
            event_number: rand_range(1, 3),
            last_update: Instant::now(),
            frame_delay: Duration::from_millis(100),
        }
    }

    // Handles logic loops and random events
    fn handle_event_logic(&mut self) {
        let idle_num = 1..=11;
        let walk_left = 13..=15;
        let walk_right = 16..=18;
        let sleep_num = 19..=25;

        if idle_num.contains(&self.event_number) {
            self.state = 0;
            self.frame_delay = Duration::from_millis(400);
        } else if self.event_number == 12 {
            self.state = 1;
            self.frame_delay = Duration::from_millis(100);
        } else if walk_left.contains(&self.event_number) {
            self.state = 4;
            self.frame_delay = Duration::from_millis(100);
        } else if walk_right.contains(&self.event_number) {
            self.state = 5;
            self.frame_delay = Duration::from_millis(100);
        } else if sleep_num.contains(&self.event_number) {
            self.state = 2;
            self.frame_delay = Duration::from_millis(400);
        } else if self.event_number == 26 {
            self.state = 3;
            self.frame_delay = Duration::from_millis(100);
        }
    }

    // Increments frames or rolls for new sequence transitions
    fn animate(&mut self, total_frames: usize, min_rand: u32, max_rand: u32) {
        if self.i_frame < total_frames - 1 {
            self.i_frame += 1;
        } else {
            self.i_frame = 0;
            self.event_number = rand_range(min_rand, max_rand);
        }
    }
}

impl eframe::App for Ket {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Run logic step when frame-delay timer ticks over
        if self.last_update.elapsed() >= self.frame_delay {
            self.handle_event_logic();

            match self.state {
                0 => {
                    let max = self.idle.len();
                    self.animate(max, 1, 18);
                }
                1 => {
                    let max = self.idle_to_sleeping.len();
                    self.animate(max, 19, 19);
                }
                2 => {
                    let max = self.sleeping.len();
                    self.animate(max, 19, 26);
                }
                3 => {
                    let max = self.sleeping_to_idle.len();
                    self.animate(max, 1, 1);
                }
                4 => {
                    if self.x > 0.0 { self.x -= 3.0; }
                    let max = self.walking_left.len();
                    self.animate(max, 1, 18);
                }
                5 => {
                    if self.x < (self.screen_width - 72.0) { self.x += 3.0; }
                    let max = self.walking_right.len();
                    self.animate(max, 1, 18);
                }
                _ => {} 
            }

            // Note: The problematic OuterPosition command has been safely removed from here
            self.last_update = Instant::now();
        }

        // Draw the cat texture onto clear OS layer
        let current_texture = match self.state {
            0 => &self.idle[self.i_frame],
            1 => &self.idle_to_sleeping[self.i_frame],
            2 => &self.sleeping[self.i_frame],
            3 => &self.sleeping_to_idle[self.i_frame],
            4 => &self.walking_left[self.i_frame],
            5 => &self.walking_right[self.i_frame],
            _ => &self.idle[0],
        };

        let panel_frame = egui::Frame {
            fill: egui::Color32::TRANSPARENT,
            ..Default::default()
        };

        egui::CentralPanel::default()
            .frame(panel_frame)
            .show(ctx, |ui| {
                ui.centered_and_justified(|ui| {
                    ui.add(egui::Image::new(current_texture.clone()).max_width(72.0));
                });
            });

        // Loop the UI repaint refresh
        ctx.request_repaint();
    }
}

// Minimal, lightweight pseudo-random number generator to avoid big dependencies
fn rand_range(min: u32, max: u32) -> u32 {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};
    let mut hasher = RandomState::new().build_hasher();
    hasher.write_u128(Instant::now().elapsed().as_nanos());
    let raw = hasher.finish() as u32;
    (raw % (max - min + 1)) + min
}

