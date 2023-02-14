#![allow(non_upper_case_globals)]
#![allow(dead_code)]
use crate::egui::*;
use crate::egui::style::*;

pub const nord0: Color32 = Color32::from_rgb(46, 52, 64);
pub const nord1: Color32 = Color32::from_rgb(59, 66, 82);
pub const nord2: Color32 = Color32::from_rgb(67, 76, 94);
pub const nord3: Color32 = Color32::from_rgb(76, 86, 106);
pub const nord4: Color32 = Color32::from_rgb(216, 222, 233);
pub const nord5: Color32 = Color32::from_rgb(229, 233, 240);
pub const nord6: Color32 = Color32::from_rgb(236, 239, 244);
pub const nord7: Color32 = Color32::from_rgb(143, 188, 187);
pub const nord8: Color32 = Color32::from_rgb(136, 192, 208);
pub const nord9: Color32 = Color32::from_rgb(129, 161, 193);
pub const nord10: Color32 = Color32::from_rgb(94, 129, 172);
pub const nord11: Color32 = Color32::from_rgb(191, 97, 106);
pub const nord12: Color32 = Color32::from_rgb(208, 135, 112);
pub const nord13: Color32 = Color32::from_rgb(235, 203, 139);
pub const nord14: Color32 = Color32::from_rgb(163, 190, 140);
pub const nord15: Color32 = Color32::from_rgb(180, 142, 173);

pub fn nord() -> Visuals {
    Visuals {
        dark_mode: true,
        override_text_color: None,
        widgets: Widgets {
            noninteractive: WidgetVisuals {
                weak_bg_fill: nord2,
                bg_fill: nord2,
                bg_stroke: Stroke::new(1.0, nord3), // separators, indentation lines
                fg_stroke: Stroke::new(1.0, nord5), // normal text color
                rounding: Rounding::same(2.0),
                expansion: 0.0,
            },
            inactive: WidgetVisuals {
                weak_bg_fill: nord1, // button background
                bg_fill: nord1,      // checkbox background
                bg_stroke: Stroke::new(1.0, nord3),
                fg_stroke: Stroke::new(1.0, nord5), // button text
                rounding: Rounding::same(2.0),
                expansion: 0.0,
            },
            hovered: WidgetVisuals {
                weak_bg_fill: nord3,
                bg_fill: nord3,
                bg_stroke: Stroke::new(1.0, nord3), // e.g. hover over window edge or button
                fg_stroke: Stroke::new(1.5, nord5),
                rounding: Rounding::same(3.0),
                expansion: 1.0,
            },
            active: WidgetVisuals {
                weak_bg_fill: nord2,
                bg_fill: nord2,
                bg_stroke: Stroke::new(1.0, nord6),
                fg_stroke: Stroke::new(2.0, nord6),
                rounding: Rounding::same(2.0),
                expansion: 1.0,
            },
            open: WidgetVisuals {
                weak_bg_fill: nord2,
                bg_fill: nord2,
                bg_stroke: Stroke::new(1.0, nord4),
                fg_stroke: Stroke::new(1.0, nord4),
                rounding: Rounding::same(2.0),
                expansion: 0.0,
            },
        },
        selection: Selection::default(),
        hyperlink_color: nord8,
        faint_bg_color: nord1, // visible, but barely so
        extreme_bg_color: nord0,            // e.g. TextEdit background
        code_bg_color: nord1,
        warn_fg_color: nord12, // orange
        error_fg_color: nord11,  // red

        window_rounding: Rounding::same(6.0),
        window_shadow: epaint::Shadow::big_dark(),
        window_fill: nord0,
        window_stroke: Stroke::new(1.0, Color32::from_gray(60)),

        menu_rounding: Rounding::same(6.0),

        panel_fill: nord0,

        popup_shadow: epaint::Shadow::small_dark(),
        resize_corner_size: 12.0,
        text_cursor_width: 2.0,
        text_cursor_preview: false,
        clip_rect_margin: 3.0, // should be at least half the size of the widest frame stroke + max WidgetVisuals::expansion
        button_frame: true,
        collapsing_header_frame: false,
        indent_has_left_vline: true,

        striped: false,

        slider_trailing_fill: false,
    }
}