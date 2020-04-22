static SPACE: Letter = Letter {
    index: 0,
    aspect_ratio: 2.0, // 2 / 1
    associated_char: Some(' '),
};
static CARET: Letter = Letter {
    index: 1,
    aspect_ratio: 1.5, // 3 / 2
    associated_char: None,
};

pub struct Letter {
    pub index: u8,
    pub aspect_ratio: f32,
    pub associated_char: Option<char>,
}

pub struct TerminalBundle {
    //
}
