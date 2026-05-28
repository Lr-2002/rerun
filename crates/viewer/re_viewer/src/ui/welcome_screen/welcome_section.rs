use re_ui::{DesignTokens, UiExt as _};

pub(super) const WELCOME_SCREEN_TITLE: &str = "欢迎使用 Delta";

const WELCOME_SCREEN_INTRO: &str = "Delta 是 DOHC 的运动数据可视化前端，与 Rerun 后端联动，支持离线 rrd 文件与在线推流两种数据源。";

const WELCOME_SCREEN_SECTIONS: &[(&str, &[&str])] = &[
    (
        "1. 选视图",
        &[
            "`/demo` — 六宫格，覆盖六路数据全景",
            "`/analyze` — 三宫格，聚焦三路关键指标",
        ],
    ),
    (
        "2. 给数据",
        &[
            "离线：选择本地路径，系统自动解析 rrd",
            "在线：填入推流 IP / Port，接入 3 路视频 + 1 路数据（x, y, vx, vy, vz, wx, wy, wz）",
        ],
    ),
    (
        "3. 读图（从左到右）",
        &[
            "左：XY 轨迹俯视图，颜色随时间渐变",
            "中：线速度 V — X / Y / Z 三轴合并",
            "右：角速度 Ω — X / Y / Z 三轴合并",
        ],
    ),
];

/// Show the welcome section.
pub(super) fn welcome_section_ui(ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        let max_width = ui.available_width().min(620.0);
        ui.set_max_width(max_width);

        egui::Frame::new()
            .inner_margin(egui::Margin::symmetric(24, 22))
            .corner_radius(8)
            .fill(ui.visuals().extreme_bg_color)
            .stroke(ui.tokens().native_frame_stroke)
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    title_ui(ui);
                    ui.add_space(10.0);
                    body_ui(ui, WELCOME_SCREEN_INTRO);
                    ui.add_space(18.0);

                    ui.add(
                        egui::Label::new(
                            egui::RichText::new("三步上手")
                                .strong()
                                .text_style(DesignTokens::welcome_screen_body()),
                        )
                        .wrap(),
                    );
                    ui.add_space(8.0);

                    for (index, (title, bullets)) in WELCOME_SCREEN_SECTIONS.iter().enumerate() {
                        step_ui(ui, title, bullets);
                        if index + 1 != WELCOME_SCREEN_SECTIONS.len() {
                            ui.add_space(8.0);
                        }
                    }

                    ui.add_space(20.0);
                    cta_buttons_ui(ui);
                });
            });
    });
}

fn title_ui(ui: &mut egui::Ui) {
    let width = ui.available_width();
    let (style, line_height) = if width > 400.0 {
        (DesignTokens::welcome_screen_h1(), 46.0)
    } else if width > 260.0 {
        (DesignTokens::welcome_screen_h2(), 32.0)
    } else {
        (DesignTokens::welcome_screen_body(), 24.0)
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
}

fn body_ui(ui: &mut egui::Ui, text: &str) {
    ui.add(
        egui::Label::new(
            egui::RichText::new(text)
                .color(ui.visuals().weak_text_color())
                .text_style(DesignTokens::welcome_screen_body()),
        )
        .wrap(),
    );
}

fn step_ui(ui: &mut egui::Ui, title: &str, bullets: &[&str]) {
    ui.add(
        egui::Label::new(
            egui::RichText::new(title)
                .strong()
                .text_style(DesignTokens::welcome_screen_body()),
        )
        .wrap(),
    );

    ui.add_space(3.0);
    for bullet in bullets {
        ui.horizontal_top(|ui| {
            ui.add_space(1.0);
            ui.bullet(ui.visuals().weak_text_color());
            ui.add_space(4.0);
            body_ui(ui, bullet);
        });
    }
}

fn cta_buttons_ui(ui: &mut egui::Ui) {
    ui.horizontal_wrapped(|ui| {
        if ui
            .primary_button("进入 Demo")
            .on_hover_text("打开 /demo，并保留当前 query params")
            .clicked()
        {
            navigate_to_route(ui, "/demo");
        }

        if ui
            .secondary_button("进入 Analyze")
            .on_hover_text("打开 /analyze，并保留当前 query params")
            .clicked()
        {
            navigate_to_route(ui, "/analyze");
        }
    });
}

fn navigate_to_route(ui: &mut egui::Ui, route: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        let _ = ui;
        let Some(window) = web_sys::window() else {
            re_log::warn!("Failed to navigate DOHC route: window is unavailable");
            return;
        };

        let location = window.location();
        let search = location.search().unwrap_or_default();
        let hash = location.hash().unwrap_or_default();
        let url = format!("{route}{search}{hash}");

        if let Err(err) = location.assign(&url) {
            re_log::warn!(
                "Failed to navigate DOHC route to {url:?}: {}",
                crate::web_tools::string_from_js_value(err)
            );
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        ui.open_url(egui::OpenUrl::same_tab(route.to_owned()));
    }
}
