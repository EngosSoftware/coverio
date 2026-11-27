use antex::{Color, ColorMode, StyledText, Text};
use std::process::ExitCode;

fn main() -> ExitCode {
    coverio::run().unwrap_or_else(|reason| {
        let error_msg = Text::new(ColorMode::default())
            .color(Color::Red)
            .bold()
            .s("error:")
            .r(' ')
            + reason.to_string();
        eprintln!("{error_msg}");
        ExitCode::FAILURE
    })
}
