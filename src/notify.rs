use time::OffsetDateTime;

use crate::Args;

#[cfg(not(feature = "gui"))]
mod cli {

    use super::*;

    pub(crate) fn ask(_args: &mut Args) {
        println!("Alarm time must be specified. Type `--help` for more information.");
    }

    /// Ansi escape sequence that triggers the usually audio alert cue
    const BELL_SEQUENCE: &str = "\x07";

    pub(crate) fn alarm(_target_time: OffsetDateTime, message: &str) {
        println!("{}{}", message, BELL_SEQUENCE);
    }
}

#[cfg(feature = "gui")]
mod gui {

    use eframe::egui;
    use std::{cell::Cell, rc::Rc};

    use super::*;
    use crate::{parse_current_timespec, timespec_fmt};

    pub(crate) fn ask(args: &mut Args) {
        let native_options = eframe::NativeOptions {
            always_on_top: true,
            centered: true,
            initial_window_size: Some(egui::vec2(260.0, 130.0)),
            ..Default::default()
        };

        struct GetSchedule {
            timespec_buf: String,
            message_buf: String,
            to_update: std::rc::Weak<Cell<Args>>,
            first_iteration: bool,
        }

        impl eframe::App for GetSchedule {
            fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let mut enter_pressed = false;
                    ctx.input(|i| {
                        if i.key_pressed(egui::Key::Escape) {
                            frame.close();
                        }
                        enter_pressed = i.key_pressed(egui::Key::Enter);
                    });

                    ui.heading("Alarm trigger time");
                    ui.add_space(10.0);

                    let ts = ui.text_edit_singleline(&mut self.timespec_buf);
                    if self.first_iteration {
                        ts.request_focus();
                    }
                    ui.add_space(8.0);
                    ui.text_edit_singleline(&mut self.message_buf);
                    ui.add_space(10.0);

                    if ui.button("Start").clicked() || enter_pressed {
                        match parse_current_timespec(&self.timespec_buf) {
                            Ok(duration) => {
                                self.to_update
                                    .upgrade()
                                    .expect("Target for new Args value unexpectedly dropped.")
                                    .set(Args {
                                        timespec: Some(duration),
                                        message: std::mem::take(&mut self.message_buf),
                                    });
                            }
                            Err(e) => {
                                let _ = &e;
                                log!("Invalid time specification ignored: {e}");
                            }
                        }
                        frame.close();
                    }
                    self.first_iteration = false;
                });
            }
        }

        let new_args = Rc::new(Cell::new(args.clone()));

        let gui_state = GetSchedule {
            to_update: Rc::downgrade(&new_args),
            timespec_buf: String::new(),
            message_buf: args.message.clone(),
            first_iteration: true,
        };

        eframe::run_native(
            "Set alarm clock",
            native_options,
            Box::new(move |_cc| Box::new(gui_state)),
        )
        .unwrap();

        let Ok(new_args) = Rc::try_unwrap(new_args) else {
            panic!("Only single reference to updated value of Args should exist now.");
        };
        *args = new_args.into_inner();
    }

    pub(crate) fn alarm(target_time: OffsetDateTime, message: &str) {
        let native_options = eframe::NativeOptions {
            always_on_top: true,
            centered: true,
            initial_window_size: Some(egui::vec2(240.0, 110.0)),
            ..Default::default()
        };

        struct TimeTriggered {
            message: String,
            target_time: OffsetDateTime,
        }

        impl eframe::App for TimeTriggered {
            fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
                ctx.input(|i| {
                    if i.key_pressed(egui::Key::Escape) || i.key_pressed(egui::Key::Q) {
                        frame.close();
                    }
                });

                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("Alarm triggered!");
                    ui.add_space(6.0);

                    ui.label(&format!("at {}", timespec_fmt(self.target_time)));
                    ui.add_space(10.0);
                    ui.label(&self.message);
                });
            }
        }

        let gui_state = TimeTriggered {
            target_time,
            message: message.to_owned(),
        };

        eframe::run_native(
            "Alarm clock",
            native_options,
            Box::new(move |_cc| Box::new(gui_state)),
        )
        .unwrap();
    }
}

#[cfg(not(feature = "gui"))]
pub(crate) use cli::{alarm, ask};

#[cfg(feature = "gui")]
pub(crate) use gui::{alarm, ask};
