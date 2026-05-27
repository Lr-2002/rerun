use re_ui::DesignTokens;

pub(super) const DOCS_URL: &str = "https://www.rerun.io/docs";
pub(super) const WELCOME_SCREEN_TITLE: &str = "Welcome to Delta";
pub(super) const WELCOME_SCREEN_BULLET_TEXT: &[&str] = &[
    "Inspect DOHC RGB, skeleton, teleoperation, and motion telemetry in one viewer",
    "Open /demo for the three-panel operator view",
    "Open /dashboard for the six-panel analysis layout",
];

/// Show the welcome section.
pub(super) fn welcome_section_ui(ui: &mut egui::Ui) {
    ui.vertical(|ui| {
        let (style, line_height) = if ui.available_width() > 400.0 {
            (DesignTokens::welcome_screen_h1(), 50.0)
        } else {
            (DesignTokens::welcome_screen_h2(), 36.0)
        };

        ui.add(
            egui::Label::new(
                egui::RichText::new(WELCOME_SCREEN_TITLE)
                    .strong()
                    .line_height(Some(line_height))
                    .text_style(style),
            )
            .wrap(),
        );
    });
}
