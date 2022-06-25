//use std::default;
use egui::{
    plot::{Line, MarkerShape, Plot, Value, Values},
    text::LayoutJob,
    Style, TextStyle, Ui,
};
use nalgebra::*;

use eframe::epaint::{CircleShape, Fonts, RectShape, TextShape};
use egui::{Color32, Frame, Pos2};
use lib_genetic_algo::Statistics;
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
use lib_simulation::*;
use rand::thread_rng;

struct Logger {
    data: Vec<Statistics>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,
    #[serde(skip)]
    simulation: Simulation,
    #[serde(skip)]
    statistics: Statistics,
    rect_scale: f32,

    #[serde(skip)]
    logger: Logger,
    #[serde(skip)]
    generation: usize,
    // this how you opt-out of serialization of a member
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            rect_scale: 0.7,
            simulation: Simulation::randomize(&mut thread_rng()),
            statistics: Statistics {
                avg_fitness: 0.0,
                max_fitness: 0.0,
                min_fitness: 0.0,
            },
            logger: Logger { data: Vec::new() },
            generation: 1,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        cc.egui_ctx.set_visuals(egui::Visuals {
            dark_mode: true,
            ..Default::default()
        });
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self {
            label: _,
            rect_scale,
            simulation,
            statistics,
            logger,
            generation,
        } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        ctx.request_repaint();

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Settings");

            ui.add(egui::Slider::new(rect_scale, 0.1..=10.0).text("Scale"));
            ui.horizontal(|ui| {
                if ui.button("Zoom in").clicked() {
                    *rect_scale += 0.1;
                }
                if ui.button("Zoom out").clicked() {
                    *rect_scale -= 0.1;
                }
            });
            ui.separator();
            ui.heading("Statistics");

            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Number of ants:");
                    ui.label("50");
                });
                ui.horizontal(|ui| {
                    ui.label("Food quantity:");
                    ui.label("30");
                });
                ui.horizontal(|ui| {
                    ui.label("Generation:");
                    ui.label(format!("{}", generation));
                });
                ui.horizontal(|ui| {
                    ui.label("Average fitness:");
                    ui.label(format!("{}", statistics.avg_fitness));
                });
                ui.horizontal(|ui| {
                    ui.label("Max fitness: ");
                    ui.label(format!("{}", statistics.max_fitness));
                });
                ui.horizontal(|ui| {
                    ui.label("Min fitness: ");
                    ui.label(format!("{}", statistics.min_fitness));
                });
            });
            egui::Window::new("Plot").open(&mut true).show(ctx, |ui| {
                let avg_data = (0..logger.data.len())
                    .map(|point| Value::new(point as f32, logger.data[point].avg_fitness));
                let max_data = (0..logger.data.len())
                    .map(|point| Value::new(point as f32, logger.data[point].max_fitness));
                let min_data = (0..logger.data.len())
                    .map(|point| Value::new(point as f32, logger.data[point].min_fitness));

                let plot_avg = Line::new(Values::from_values_iter(avg_data))
                    .fill(0.0)
                    .color(Color32::from_rgb(0, 204, 153))
                    .name("Average fitness");
                let _plot_max = Line::new(Values::from_values_iter(max_data))
                    .fill(0.0)
                    .color(Color32::from_rgb(255, 0, 102))
                    .name("Max fitness");
                let _plot_min = Line::new(Values::from_values_iter(min_data))
                    .fill(0.0)
                    .color(Color32::from_rgb(51, 153, 255))
                    .name("Min fitness");

                let plot = Plot::new("fitness")
                    .height(350.0)
                    .legend(egui::plot::Legend::default())
                    .data_aspect(0.4)
                    .center_y_axis(false)
                    .center_y_axis(false);

                ui.small("Average score per generation");

                plot.show(ui, |plot_ui| plot_ui.line(plot_avg));
            });

            ui.separator();
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("Created by ");
                    ui.hyperlink_to("crimsondamask", "https://github.com/crimsondamask");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("Simulation viewport");
            egui::warn_if_debug_build(ui);
            ui.small("The simulation is rendered at 60fps.");
            if let Some(new_statistics) = simulation.step_forward(&mut thread_rng()) {
                *statistics = new_statistics.clone();
                logger.data.push(new_statistics);
                *generation += 1;
            }

            let font_id = TextStyle::Monospace.resolve(ui.style());
            let style = Style {
                visuals: egui::Visuals {
                    extreme_bg_color: Color32::from_rgb(19, 38, 58),
                    ..Default::default()
                },
                ..Default::default()
            };
            Frame::canvas(&style).show(ui, |ui| {
                let (response, painter) =
                    ui.allocate_painter(ui.available_size_before_wrap(), egui::Sense::hover());
                let galley = ui.painter().layout_no_wrap(
                    format!("Ants: 50\nFood: 30\nGenerations: {}", *generation - 1),
                    font_id.clone(),
                    Color32::LIGHT_GRAY,
                );
                let window_size_width = response.rect.width();

                let window_size_height = response.rect.height();
                let text_pos = Pos2::new(window_size_width + 150.0, window_size_height);

                painter.add(TextShape::new(text_pos, galley));

                let r = (response.rect.width() * 0.003) * *rect_scale;
                let ant_color = Color32::from_rgb(0, 204, 153);
                let food_color = Color32::from_rgb(255, 102, 204);

                for ant in simulation.world().animals() {
                    // let vision_input = &ant.vision_input;
                    // let vision_len = &vision_input.len();
                    let rot = ant.rotation().angle();
                    let circle_pos = Pos2::new(
                        ant.position().x * (window_size_width) * *rect_scale + 200.0,
                        ant.position().y * window_size_height * *rect_scale + 100.0,
                    );
                    painter.add(CircleShape::filled(circle_pos, r, ant_color));

                    let ant_pointer_pos = Pos2::new(
                        circle_pos.x + 8.0 * rot.cos(),
                        circle_pos.y + 8.0 * rot.sin(),
                    );
                    painter.add(CircleShape::filled(ant_pointer_pos, r * 0.7, ant_color));
                }

                for food in simulation.world().food() {
                    let center = Pos2::new(
                        food.position().x * (window_size_width) * *rect_scale + 200.0,
                        food.position().y * window_size_height * *rect_scale + 100.0,
                    );
                    painter.add(CircleShape::filled(center, r * 0.7, food_color));
                }

                // let to_screen = egui::emath::RectTransform::from_to(
                //     egui::Rect::from_min_size(egui::Pos2::ZERO, response.rect.square_proportions()),
                //     response.rect,
                // );
            })
        });
    }
}
