use eframe::egui::{self, Align, Color32, Layout, RichText, TextStyle, Vec2, Visuals};
use eframe::{App, Frame, NativeOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let window_size = Vec2::new(400.0, 500.0);

    let options = NativeOptions {
        resizable: false,
        initial_window_size: Some(window_size),
        min_window_size: Some(window_size),
        max_window_size: Some(window_size),
        ..Default::default()
    };

    eframe::run_native(
        "Fresnel Zone Calculator",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
    Ok(())
}

#[derive(Default)]
struct MyApp {
    distance: f32,
    frequency_ghz: f32,
    fresnel_zone: Option<f32>,
    unit: Unit,
}

#[derive(PartialEq)]
enum Unit {
    Kilometers,
    Miles,
}

impl Default for Unit {
    fn default() -> Self {
        Unit::Kilometers
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        // Set dark or light theme based on system settings
        ctx.set_visuals(Visuals::dark());

        // Customize visual style
        let mut visuals = ctx.style().visuals.clone();
        visuals.widgets.active.rounding = egui::Rounding::same(10.0);
        visuals.widgets.inactive.rounding = egui::Rounding::same(10.0);
        visuals.widgets.hovered.rounding = egui::Rounding::same(10.0);
        ctx.set_visuals(visuals);

        // Responsive layout
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                // Title with large, bold text
                ui.add_space(20.0);
                ui.label(
                    RichText::new("Fresnel Zone Calculator")
                        .size(30.0)
                        .color(Color32::from_rgb(100, 255, 100))
                        .text_style(TextStyle::Heading),
                );

                ui.add_space(10.0);

                // Introduction or instructions
                ui.label(
                    RichText::new(
                        "Calculate the Fresnel zone easily by entering the distance and frequency.",
                    )
                    .italics()
                    .size(16.0),
                );

                ui.add_space(20.0);

                // Input section for distance
                ui.group(|ui| {
                    ui.label(RichText::new("Step 1: Enter the distance:").size(20.0));
                    ui.add_space(10.0);

                    ui.horizontal(|ui| {
                        ui.add(
                            egui::Slider::new(&mut self.distance, 0.0..=1000.0)
                                .text("")
                                .clamp_to_range(true)
                                .logarithmic(true),
                        );
                    });

                    ui.horizontal(|ui| {
                        ui.radio_value(&mut self.unit, Unit::Kilometers, "Kilometers");
                        ui.radio_value(&mut self.unit, Unit::Miles, "Miles");
                    });
                });

                ui.add_space(20.0);

                // Input section for frequency
                ui.group(|ui| {
                    ui.label(RichText::new("Step 2: Enter the frequency in GHz:").size(20.0));
                    ui.add_space(10.0);

                    ui.add(
                        egui::Slider::new(&mut self.frequency_ghz, 0.1..=100.0)
                            .text("")
                            .clamp_to_range(true)
                            .logarithmic(true),
                    );
                });

                ui.add_space(30.0);

                // Calculate button with color and size customization
                if ui
                    .add_sized(
                        [150.0, 40.0],
                        egui::Button::new(
                            RichText::new("Calculate").size(22.0).color(Color32::WHITE),
                        )
                        .fill(Color32::from_rgb(0, 128, 255)),
                    )
                    .clicked()
                {
                    let distance_km = match self.unit {
                        Unit::Kilometers => self.distance,
                        Unit::Miles => self.distance * 1.60934, // Convert miles to kilometers
                    };
                    self.fresnel_zone =
                        Some(calculate_fresnel_zone(distance_km, self.frequency_ghz));
                }

                ui.add_space(20.0);

                // Display result with enhanced text style
                if let Some(fz) = self.fresnel_zone {
                    ui.label(
                        RichText::new(format!("Fresnel Zone (F1) [m]: {:.2}", fz))
                            .size(24.0)
                            .color(Color32::from_rgb(0, 255, 0))
                            .text_style(TextStyle::Button),
                    );
                } else {
                    ui.label(
                        RichText::new("Press Calculate to see the result.")
                            .color(Color32::GRAY)
                            .size(20.0),
                    );
                }

                ui.add_space(30.0);

                // Footer with developer info and softer text color
                ui.colored_label(Color32::from_rgb(180, 180, 180), "Developed by: Your Name");
                ui.add_space(20.0);
            });
        });
    }
}

fn calculate_fresnel_zone(distance_km: f32, frequency_ghz: f32) -> f32 {
    8.656 * ((distance_km / frequency_ghz).sqrt())
}
