use bevy::prelude::*;

#[derive(Bundle)]
pub struct TextBundle {
    pub text: Text,
    pub text_font: TextFont,
    pub text_shadow: TextShadow,
    pub text_layout: TextLayout,
    pub node: Node,
}

impl Default for TextBundle {
    fn default() -> Self {
        Self {
            text: Text::new(""),
            text_font: TextFont {
                font_size: 16.0,
                ..default()
            },
            text_shadow: TextShadow::default(),
            text_layout: TextLayout::new_with_justify(JustifyText::Left),
            node: Node::default(),
        }
    }
}

impl TextBundle {
    fn new() -> Self {
        default()
    }
}
