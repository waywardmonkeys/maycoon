use femtovg::{Color, Paint};
use mint::Vector2;

use may_core::layout::{Layout, Style};
use may_core::widget::interaction::InteractionInfo;
use may_core::widget::update::Update;
use may_core::widget::{PathMode, Sketch, Widget};
use may_theme::scheme::SchemeValue;
use may_theme::theme::{Theme, WidgetType};

pub struct Text<'a> {
    text: String,
    children: Vec<Box<dyn Widget<'a> + 'a>>,
    style: Style,
    font_size: f32,
}

impl<'a> Text<'a> {
    pub fn id() -> String {
        String::from("may-widgets:Text")
    }

    pub fn new(text: impl ToString) -> Self {
        Text {
            text: text.to_string(),
            children: Vec::new(),
            style: Style::default(),
            font_size: 16.0,
        }
    }

    pub fn set_text(mut self, text: impl ToString) -> Self {
        self.text = text.to_string();
        self
    }

    pub fn with_text(mut self, text: impl ToString) -> Self {
        self.text = text.to_string();
        self
    }

    pub fn with_font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }

    pub fn set_font_size(&mut self, font_size: f32) {
        self.font_size = font_size;
    }
}

impl<'a> Widget<'a> for Text<'a> {
    fn render(&mut self, layout: &Layout, theme: &Box<dyn Theme>) -> Vec<Sketch> {
        let scheme = theme.scheme_of("may-widgets:Text".to_string())
            .unwrap_or(theme.default_scheme_of(WidgetType::Content));

        vec![Sketch::Text(
            self.text.clone(),
            Vector2::from([layout.location.x, layout.location.y + self.font_size]),
            scheme.get("color")
                .unwrap_or(&SchemeValue::Paint(Paint::color(Color::black())))
                .as_paint()
                .unwrap()
                .clone()
                .with_font_size(self.font_size),
            PathMode::Fill,
        )]
    }

    fn update(&mut self, info: &InteractionInfo, layout: &Layout) -> Update {
        Update::empty()
    }

    fn children(&self) -> &Vec<Box<dyn Widget<'a> + 'a>> {
        &self.children
    }

    fn children_mut(&mut self) -> &mut Vec<Box<dyn Widget<'a> + 'a>> {
        &mut self.children
    }

    fn style(&self) -> &Style {
        &self.style
    }

    fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }
}
