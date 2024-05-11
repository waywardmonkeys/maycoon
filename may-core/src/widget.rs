use taffy::{Layout, Style};

use may_theme::id::WidgetId;
use may_theme::scheme::Scheme;
use may_theme::theme::WidgetType;

use crate::app::context::Context;
use crate::app::update::Update;
use crate::render::RenderCommand;
use crate::state::State;

pub trait Widget<S: State> {
    fn render(&self, theme: Scheme, layout: WidgetLayoutNode) -> Vec<RenderCommand>;
    fn id(&self) -> WidgetId;
    fn update(&mut self, state: &mut S, ctx: &Context) -> Update;
    fn style_node(&self) -> WidgetStyleNode;
    fn widget_type(&self) -> WidgetType;
}

#[derive(Default, Debug, Clone)]
pub struct WidgetStyleNode {
    pub style: Style,
    pub children: Vec<WidgetStyleNode>,
}

#[derive(Debug, Clone)]
pub struct WidgetLayoutNode {
    pub layout: Layout,
    pub children: Vec<WidgetLayoutNode>,
}