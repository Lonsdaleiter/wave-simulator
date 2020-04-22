static SPACE: Letter = Letter {
    index: 0,
    associated_char: Some(' '),
};
static CARET: Letter = Letter {
    index: 1,
    associated_char: None,
};

pub struct Letter {
    pub index: u8,
    pub associated_char: Option<char>,
}

pub struct TerminalBundle {
    //
}
