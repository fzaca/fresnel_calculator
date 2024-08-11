use eframe::egui::{self, Align, Color32, Hyperlink, Layout, RichText, TextStyle, Vec2, Visuals};
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
    selected_tab: Tab,
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

#[derive(PartialEq)]
enum Tab {
    Calculator,
    Readme,
}

impl Default for Tab {
    fn default() -> Self {
        Tab::Calculator
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        // Set dark or light theme based on system settings
        ctx.set_visuals(Visuals::dark());

        // Responsive layout
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui
                    .selectable_label(self.selected_tab == Tab::Calculator, "Calculator")
                    .clicked()
                {
                    self.selected_tab = Tab::Calculator;
                }
                if ui
                    .selectable_label(self.selected_tab == Tab::Readme, "README")
                    .clicked()
                {
                    self.selected_tab = Tab::Readme;
                }
            });

            ui.separator(); // Add a line separator between the tabs and content

            match self.selected_tab {
                Tab::Calculator => {
                    self.show_calculator_tab(ui);
                }
                Tab::Readme => {
                    self.show_readme_tab(ui);
                }
            }
        });
    }
}

impl MyApp {
    fn show_calculator_tab(&mut self, ui: &mut egui::Ui) {
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
                    egui::Slider::new(&mut self.frequency_ghz, 0.1..=50.0)
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
                    egui::Button::new(RichText::new("Calculate").size(22.0).color(Color32::WHITE))
                        .fill(Color32::from_rgb(0, 128, 255)),
                )
                .clicked()
            {
                let distance_km = match self.unit {
                    Unit::Kilometers => self.distance,
                    Unit::Miles => self.distance * 1.60934, // Convert miles to kilometers
                };
                self.fresnel_zone = Some(calculate_fresnel_zone(distance_km, self.frequency_ghz));
            }

            ui.add_space(20.0);

            // Display result with enhanced text style
            if let Some(fz) = self.fresnel_zone {
                ui.label(
                    RichText::new(format!("Fresnel Zone: {:.2}m", fz))
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

            // Footer with hyperlink to GitHub profile
            ui.add(Hyperlink::from_label_and_url(
                "Developed by: fzaca",
                "https://github.com/fzaca",
            ));
            ui.add_space(20.0);
        });
    }

    fn show_readme_tab(&mut self, ui: &mut egui::Ui) {
        ui.with_layout(Layout::top_down(Align::Center), |ui| {
            ui.add_space(20.0);
            ui.label(
                RichText::new("How to Use the Fresnel Zone Calculator")
                    .size(30.0)
                    .color(Color32::from_rgb(100, 255, 100))
                    .text_style(TextStyle::Heading),
            );

            ui.add_space(10.0);

            ui.label(
                "1. In the Calculator tab, enter the distance between the two points.\n\
                 2. Select the unit of distance (Kilometers or Miles).\n\
                 3. Enter the frequency of the signal in GHz.\n\
                 4. Click 'Calculate' to see the Fresnel zone in meters.\n\n\
                 The Fresnel zone is a crucial concept in wireless communication, \
                 indicating the area around the line-of-sight that must be free of obstacles \
                 to avoid signal interference.",
            );

            ui.add_space(30.0);

            ui.add(Hyperlink::from_label_and_url(
                "Developed by: fzaca",
                "https://github.com/fzaca",
            ));
            ui.add_space(20.0);
        });
    }
}

fn calculate_fresnel_zone(distance_km: f32, frequency_ghz: f32) -> f32 {
    8.656 * ((distance_km / frequency_ghz).sqrt())
}
