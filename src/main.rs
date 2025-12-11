#![windows_subsystem = "windows"]
use eframe::Frame;
use egui::{CentralPanel, MenuBar, TopBottomPanel, Vec2, ViewportCommand};
use encoding::{DecoderTrap, Encoding, all::KOI8_R};
use megafon_cfg_viewer::config::{ScreenConfig, parse};
use rfd::FileDialog;
use std::{fs::File, io::Read};

struct AppState {
    screen_cfg: ScreenConfig,
    selected_panel: usize,
    last_error: Option<String>,
    // modal_opened: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            screen_cfg: ScreenConfig::default(),
            selected_panel: 0,
            last_error: None,
            // modal_opened: false,
        }
    }
}

impl AppState {
    fn open_cfg_via_dialog(&mut self) {
        if let Some(path) = FileDialog::new()
            .add_filter("CFG files", &["conf"])
            .set_title("Open Megafon config")
            .pick_file()
        {
            if let Some(p) = path.to_str() {
                if let Ok(mut f) = File::open(p) {
                    let mut buffer = Vec::new();
                    if let Ok(_) = f.read_to_end(&mut buffer) {
                        match KOI8_R.decode(&buffer, DecoderTrap::Strict) {
                            Ok(s) => {
                                if let Ok(c) = parse(&s.trim()) {
                                    self.screen_cfg = c;
                                }
                            }
                            Err(_) => {}
                        };
                    }
                }
            }
        }
    }

    fn close_cfg(&mut self) {
        self.screen_cfg = ScreenConfig::default();
        self.selected_panel = 0;
    }
}
impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        // if self.modal_opened {
        //     // egui::Window::new("My Popup Window")
        //     //     .open(&mut self.modal_opened) // Adds a close button that sets window_open to false
        //     //     .show(ctx, |ui| {
        //     //         ui.label(self.last_error.clo.unwrap_or_default());
        //     //         if ui.button("Close manually").clicked() {
        //     //             self.modal_opened = false;
        //     //         }
        //     //     });
        //     egui::Window::new("Modal Dialog")
        //         .open(&mut self.modal_opened)
        //         .collapsible(false)
        //         .resizable(false)
        //         .show(ctx, |ui| {
        //             ui.label("This is a modal dialog.");
        //         });
        // }

        // --- Menu bar ---
        TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open…").clicked() {
                        ui.close();
                        self.open_cfg_via_dialog();
                    }

                    if ui.button("Close").clicked() {
                        ui.close();
                        self.close_cfg();
                    }

                    if ui.button("Quit").clicked() {
                        ui.close();
                        ctx.send_viewport_cmd(ViewportCommand::Close);
                    }
                });
            });
        });

        // --- Main content area ---
        CentralPanel::default().show(ctx, |ui| {
            if let Some(err) = &self.last_error {
                ui.colored_label(egui::Color32::RED, err);
                ui.add_space(8.0);
            }

            if self.screen_cfg.phone_panels.is_empty() {
                ui.label(
                    "Use File --> Open… to load a config file and see phone panels with buttons.",
                );
                return;
            }

            // --- Tabs (phone panels) ---
            ui.horizontal_wrapped(|ui| {
                for (idx, panel) in self.screen_cfg.phone_panels.iter().enumerate() {
                    let selected = self.selected_panel == idx;
                    let response = ui.selectable_label(selected, &panel.id);
                    if response.clicked() {
                        self.selected_panel = idx;
                    }
                }
            });

            ui.add_space(8.0);
            ui.separator();
            ui.add_space(8.0);

            // --- Canvas with normalized buttons for the selected panel ---
            if let Some(panel) = self.screen_cfg.phone_panels.get(self.selected_panel) {
                ui.add_space(8.0);

                let available_size = ui.available_size();
                egui::ScrollArea::both().show(ui, |ui| {
                    ui.set_min_size(available_size);

                    let painter = ui.painter().clone();
                    let canvas_rect = ui.max_rect();
                    let canvas_width = canvas_rect.width();
                    let canvas_height = canvas_rect.height();

                    for (idx, btn) in panel.buttons.iter().enumerate() {
                        let x = canvas_rect.left() + btn.position_x * canvas_width;
                        let y = canvas_rect.top() + btn.position_y * canvas_height;
                        let w = btn.size_width * canvas_width;
                        let h = btn.size_height * canvas_height;
                        let rect = egui::Rect::from_min_size(egui::pos2(x, y), Vec2::new(w, h));

                        let id =
                            ui.make_persistent_id(format!("btn_{}_{}", self.selected_panel, idx));

                        let response = ui.interact(rect, id, egui::Sense::click());

                        let fill = if response.hovered() {
                            egui::Color32::from_rgb(40, 200, 40)
                        } else {
                            egui::Color32::from_rgb(40, 140, 40)
                        };

                        painter.rect_filled(rect, 4.0, fill);
                        painter.rect_stroke(
                            rect,
                            4.0,
                            egui::Stroke::new(1.0, egui::Color32::BLACK),
                            egui::StrokeKind::Outside,
                        );

                        // painter.text(
                        //     rect.center(),
                        //     egui::Align2::CENTER_CENTER,
                        //     &btn.text,
                        //     egui::TextStyle::Button.resolve(ui.style()),
                        //     egui::Color32::WHITE,
                        // );

                        // Draw button text with word wrapping inside the button rect.
                        // If the text does not fit on one line, it is wrapped to new lines
                        // at word boundaries (handled by egui's layout engine).
                        let padding = egui::vec2(8.0, 8.0);
                        let inner_rect = rect.shrink2(padding);
                        let base_font = egui::TextStyle::Button.resolve(ui.style());
                        let max_width = inner_rect.width().max(0.0);
                        let galley = ui.painter().layout(
                            btn.text.clone(),
                            base_font.clone(),
                            egui::Color32::WHITE,
                            max_width,
                        );
                        // Center the (potentially multi-line) text inside the inner rect.
                        let galley_size = galley.size();
                        let text_pos = egui::pos2(
                            inner_rect.center().x - galley_size.x / 2.0,
                            inner_rect.center().y - galley_size.y / 2.0,
                        );
                        painter.galley(text_pos, galley, egui::Color32::WHITE);

                        if response.clicked() {
                            // self.last_error =
                            //     format!("Button clicked: [{}] {}", panel.id, btn.text).into();
                            // self.modal_opened = true;
                            // eprintln!("Button clicked: [{}] {}", panel.id, btn.text);
                        }
                    }
                });
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_min_inner_size([640.0, 480.0])
            .with_inner_size([640.0, 480.0]) // wide enough for the drag-drop overlay text
            .with_drag_and_drop(true),
        ..Default::default()
    };
    eframe::run_native(
        "Инспектор конфигурации РМ СКРС \"Мегафон\" v 0.1.0",
        options,
        Box::new(|_cc| Ok(Box::<AppState>::default())),
    )
}
