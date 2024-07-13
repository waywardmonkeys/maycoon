//! The classic counter app example.

use may_core::app::update::Update;
use may_core::app::MayApp;
use may_core::config::MayConfig;
use may_core::layout::{AlignItems, Dimension, FlexDirection, LayoutStyle};
use may_macro::{val, State};
use may_widgets::button::Button;
use may_widgets::container::Container;
use may_widgets::text::Text;
use nalgebra::Vector2;

#[derive(Default, State)]
struct MyState {
    count: i32,
}

fn main() {
    MayApp::new(MayConfig::default()).run::<MyState, _>(
        Container::new(vec![
            Box::new(Button::new(Text::new(val!("Increase"))).with_on_pressed(
                |s: &mut MyState| {
                    s.count += 1;
                    Update::DRAW
                },
            )),
            Box::new(Button::new(Text::new(val!("Decrease"))).with_on_pressed(
                |s: &mut MyState| {
                    s.count -= 1;
                    Update::DRAW
                },
            )),
            Box::new(Text::new(val!(|state: &MyState| {
                state.count.to_string()
            }))),
        ])
        .with_layout_style(LayoutStyle {
            size: Vector2::<Dimension>::new(Dimension::Percent(1.0), Dimension::Percent(1.0)),
            flex_direction: FlexDirection::Column,
            align_items: Some(AlignItems::Center),
            ..Default::default()
        }),
        MyState::default(),
    );
}
