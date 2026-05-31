use ply_engine::Color;

pub(crate) struct Theme {
    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_error: Color,
    pub text_dark: Color,
    pub surface: (Color, Color, Color),
    pub accent: (Color, Color, Color),
}

