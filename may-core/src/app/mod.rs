use winit::dpi::{LogicalPosition, LogicalSize, Position, Size};
use winit::event_loop::EventLoopBuilder;
use winit::platform::windows::WindowAttributesExtWindows;
use winit::window::WindowAttributes;

use may_theme::theme::Theme;

use crate::app::handler::AppHandler;
use crate::config::MayConfig;
use crate::state::State;
use crate::widget::Widget;

pub mod handler;
pub mod info;
pub mod update;

pub struct MayApp<T: Theme> {
    config: MayConfig<T>,
}

impl<T: Theme> MayApp<T> {
    pub fn new(config: MayConfig<T>) -> Self {
        Self { config }
    }

    pub fn run<S: State, W: Widget<S>>(mut self, widget: W, state: S) {
        let event_loop = EventLoopBuilder::default()
            .build()
            .expect("Failed to create event loop");

        let mut attrs = WindowAttributes::default()
            .with_inner_size(LogicalSize::new(
                self.config.window.size.x,
                self.config.window.size.y,
            ))
            .with_resizable(self.config.window.resizable)
            .with_enabled_buttons(self.config.window.buttons)
            .with_title(self.config.window.title.clone())
            .with_maximized(self.config.window.maximized)
            .with_visible(self.config.window.visible)
            .with_transparent(self.config.window.transparent)
            .with_blur(self.config.window.blur)
            .with_decorations(self.config.window.decorations)
            .with_window_icon(self.config.window.icon.clone())
            .with_corner_preference(self.config.window.corners)
            .with_content_protected(self.config.window.content_protected)
            .with_window_level(self.config.window.level)
            .with_active(self.config.window.active)
            .with_cursor(self.config.window.cursor.clone());

        attrs.max_inner_size = self
            .config
            .window
            .max_size
            .map(|v| Size::Logical(LogicalSize::new(v.x, v.y)));
        attrs.min_inner_size = self
            .config
            .window
            .min_size
            .map(|v| Size::Logical(LogicalSize::new(v.x, v.y)));
        attrs.position = self
            .config
            .window
            .position
            .map(|v| Position::Logical(LogicalPosition::new(v.x, v.y)));
        attrs.resize_increments = self
            .config
            .window
            .resize_increments
            .map(|v| Size::Logical(LogicalSize::new(v.x, v.y)));

        event_loop
            .run_app(&mut AppHandler::new(attrs, self.config, widget, state))
            .expect("Failed to run event loop");
    }
}
