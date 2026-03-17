use bevy::prelude::*;
use bevy_pretty_text::parser::TextSpanBundle;

pub fn parse_pretty_text(text: &str) -> Result<Vec<TextSpanBundle>> {
    let Ok(pretty_text) = pretty_text_parser::parse(text) else {
        return Err(BevyError::from(format!("Could not parse text: {}", text)));
    };

    Ok(pretty_text
        .0
        .into_iter()
        .map(|span| TextSpanBundle::from(span))
        .collect::<Vec<_>>())
}