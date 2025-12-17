// #![allow(dead_code)]
// #![allow(unused_variables)]

use crate::{
    Changes,
    Hex
};

use iced::{
    advanced::{
        text::Wrapping::Word
    },
    alignment::{
        Horizontal,
        Vertical
    },
    Border,
    border::Radius,
    Color,
    ContentFit,
    Element,
    Length,
    Padding,
    Shadow,
    Theme,
    Vector,
    widget::{
        center,
        Column,
        column,
        Button,
        button,
        pick_list,
        row,
        Space,
        Svg,
        svg,
        scrollable,
        scrollable::{
            Direction,
            Scrollbar,
        },
        text,
    }
};

const TITLE_SIZE : f32 = 60.0;
const HEADING_SIZE : f32 = 45.0;
const SUBHEADING_SIZE : f32 = 30.0;
const TEXT_SIZE : f32 = 20.0;
const INDEX_SIZE : f32 = 15.0;
const HEX_SIZE : f32 = 40.0;
const COL_WIDTH : f32 = 135.0;
const ROW_SPACING : f32 = 20.0;
const BUTTON_SIZE_HEADING : f32 = 58.0;
const BUTTON_SIZE_SUBHEADING : f32 = 37.0;
const BUTTON_SIZE_SMALL : f32 = 15.0;
const BUTTON_PADDING_HEADING : [f32; 2] = [12.0, 12.0];
const BUTTON_PADDING_SUBHEADING : [f32; 2] = [8.0, 8.0];
const BUTTON_STYLE : button::Style = button::Style {
    background : None,
    text_color : Color::BLACK,
    border : Border {
        color : Color::TRANSPARENT,
        width : 0.0,
        radius : Radius {
            top_left : 0.0,
            top_right : 0.0,
            bottom_left : 0.0,
            bottom_right : 0.0
        }
    },
    shadow : Shadow {
        color : Color::TRANSPARENT,
        offset : Vector::ZERO,
        blur_radius : 0.0
    },
    snap : true
};
const _DEBUG_STYLE : button::Style = button::Style {
    background : Some(iced::Background::Color(iced::Color {
        r : 0.5,
        g : 0.5,
        b : 0.5,
        a : 1.0
    })),
    text_color : Color::BLACK,
    border : Border {
        color : Color::TRANSPARENT,
        width : 0.0,
        radius : Radius {
            top_left : 0.0,
            top_right : 0.0,
            bottom_left : 0.0,
            bottom_right : 0.0
        }
    },
    shadow : Shadow {
        color : Color::TRANSPARENT,
        offset : Vector::ZERO,
        blur_radius : 0.0
    },
    snap : true
};
// Which parts of the hexagram to display by default, from the preface on.
const DISPLAY_DEFAULT : [bool; 17] = [false, true, false, true, false, false,
                                      false, false, false, false, false, false,
                                      false, false, false, false, false];

// Copy cannot be derived for iced::Theme because one variant is Arc, so this
// proxy enum is necessary
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Themes {
    Light,
    Dark,
    Dracula,
    Nord,
    SolarizedLight,
    SolarizedDark,
    GruvboxLight,
    GruvboxDark,
    CatppuccinLatte,
    CatppuccinFrappe,
    CatppuccinMacchiato,
    CatppuccinMocha,
    TokyoNight,
    TokyoNightStorm,
    TokyoNightLight,
    KanagawaWave,
    KanagawaDragon,
    KanagawaLotus,
    Moonfly,
    Nightfly,
    Oxocarbon,
    Ferra
}

impl Themes {
    const ALL :[Self;22] = [Themes::Light, Themes::Dark, Themes::Dracula,
                            Themes::Nord, Themes::SolarizedLight,
                            Themes::SolarizedDark, Themes::GruvboxLight,
                            Themes::GruvboxDark, Themes::CatppuccinLatte,
                            Themes::CatppuccinFrappe,Themes::CatppuccinMacchiato,
                            Themes::CatppuccinMocha, Themes::TokyoNight,
                            Themes::TokyoNightStorm, Themes::TokyoNightLight,
                            Themes::KanagawaWave, Themes::KanagawaDragon,
                            Themes::KanagawaLotus, Themes::Moonfly,
                            Themes::Nightfly, Themes::Oxocarbon, Themes::Ferra];
}

impl std::fmt::Display for Themes {
    fn fmt (&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Light => {write!(f, "Light")}
            Self::Dark => {write!(f, "Dark")}
            Self::Dracula => {write!(f, "Dracula")}
            Self::Nord => {write!(f, "Nord")}
            Self::SolarizedLight => {write!(f, "SolarizedLight")}
            Self::SolarizedDark => {write!(f, "SolarizedDark")}
            Self::GruvboxLight => {write!(f, "GruvboxLight")}
            Self::GruvboxDark => {write!(f, "GruvboxDark")}
            Self::CatppuccinLatte => {write!(f, "CatppuccinLatte")}
            Self::CatppuccinFrappe => {write!(f, "CatppuccinFrappe")}
            Self::CatppuccinMacchiato => {write!(f, "CatppuccinMacchiato")}
            Self::CatppuccinMocha => {write!(f, "CatppuccinMocha")}
            Self::TokyoNight => {write!(f, "TokyoNight")}
            Self::TokyoNightStorm => {write!(f, "TokyoNightStorm")}
            Self::TokyoNightLight => {write!(f, "TokyoNightLight")}
            Self::KanagawaWave => {write!(f, "KanagawaWave")}
            Self::KanagawaDragon => {write!(f, "KanagawaDragon")}
            Self::KanagawaLotus => {write!(f, "KanagawaLotus")}
            Self::Moonfly => {write!(f, "Moonfly")}
            Self::Nightfly => {write!(f, "Nightfly")}
            Self::Oxocarbon => {write!(f, "Oxocarbon")}
            Self::Ferra => {write!(f, "Ferra")}
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum State {
    Display(Hex),
    Index,
    Start
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Back,
    FoldSec(Fold),
    ThemeSelect(Themes),
    ToDraw,
    ToHex(u8),
    ToIndex,
}

#[derive(Debug, Clone, Copy)]
pub enum Fold {
    Preface,
    Judgment,
    JudgmentComm,
    Image,
    ImageComm,
    Line1,
    Line1Comm,
    Line2,
    Line2Comm,
    Line3,
    Line3Comm,
    Line4,
    Line4Comm,
    Line5,
    Line5Comm,
    Line6,
    Line6Comm
}

impl Hex {
    pub fn view<'a>(&'a self, display: &[bool; 17], color: &'a Color)
        -> Column<'a, Message> {
        let mut col = Column::with_capacity(37);
        // Title section
        col = col.push(Self::hex_svg(self.number, 150_f32, color));
        col = col.push(text(format!("{}: {} ({})", self.number, self.name,
                                    self.pinyin))
                       .size(TITLE_SIZE).wrapping(Word));

        // Preface section
        col = col.push(row![button(svg("./images/button.svg"))
                                .on_press(Message::FoldSec(Fold::Preface))
                                .style_heading(),
                            text("Preface")
                                .size(HEADING_SIZE)
                                .wrapping(Word)
                           ]
        );
        if display[0] {
            col = col.push(text(&self.preface).size(TEXT_SIZE).wrapping(Word));
        }

        // Judgment section
        col = col.push(row![button(svg("./images/button.svg"))
                                .on_press(Message::FoldSec(Fold::Judgment))
                                .style_heading(),
                            text("Judgment")
                                .size(HEADING_SIZE)
                                .wrapping(Word)
                           ]
        );
        if display[1] {
            col = col.push(text(&self.judgment).size(TEXT_SIZE).wrapping(Word));
            col = col.push(row![button(svg("./images/button.svg"))
                                    .on_press(Message::FoldSec(Fold::JudgmentComm))
                                    .style_subheading(),
                                text("Judgment Commentary")
                                    .size(SUBHEADING_SIZE)
                                    .wrapping(Word)
                               ]
            );
            if display[2] {
                col = col.push(text(&self.judgment_comm).size(TEXT_SIZE)
                                                        .wrapping(Word));
            }
        }

        // Image section
        col = col.push(row![button(svg("./images/button.svg"))
                                .on_press(Message::FoldSec(Fold::Image))
                                .style_heading(),
                            text("Image")
                                .size(HEADING_SIZE)
                                .wrapping(Word)
                           ]
        );
        if display[3] {
            col = col.push(text(&self.image).size(TEXT_SIZE).wrapping(Word));
            col = col.push(row![button(svg("./images/button.svg"))
                                    .on_press(Message::FoldSec(Fold::ImageComm))
                                    .style_subheading(),
                                text("Image Commentary")
                                    .size(SUBHEADING_SIZE)
                                    .wrapping(Word)
                               ]
            );
            if display[4] {
                col = col.push(text(&self.image_comm).size(TEXT_SIZE)
                                                     .wrapping(Word));
            }
        }

        // Line 6
        col = col.push(row![button(svg("./images/button.svg"))
                                .on_press(Message::FoldSec(Fold::Line6))
                                .style_heading(),
                            text("Line 6")
                                .size(HEADING_SIZE)
                                .wrapping(Word)
                           ]
        );
        if display[15] {
            col = col.push(text(&self.line_6).size(TEXT_SIZE).wrapping(Word));
            col = col.push(row![button(svg("./images/button.svg"))
                                    .on_press(Message::FoldSec(Fold::Line6Comm))
                                    .style_subheading(),
                                text("Line 6 Commentary")
                                    .size(SUBHEADING_SIZE)
                                    .wrapping(Word)
                               ]
            );
            if display[16] {
                col = col.push(text(&self.line_6_comm).size(TEXT_SIZE)
                                                     .wrapping(Word));
            }
        }

        // Line 5
        col = col.push(row![button(svg("./images/button.svg"))
                                .on_press(Message::FoldSec(Fold::Line5))
                                .style_heading(),
                            text("Line 5")
                                .size(HEADING_SIZE)
                                .wrapping(Word)
                           ]
        );
        if display[13] {
            col = col.push(text(&self.line_5).size(TEXT_SIZE).wrapping(Word));
            col = col.push(row![button(svg("./images/button.svg"))
                                    .on_press(Message::FoldSec(Fold::Line5Comm))
                                    .style_subheading(),
                                text("Line 5 Commentary")
                                    .size(SUBHEADING_SIZE)
                                    .wrapping(Word)
                               ]
            );
            if display[14] {
                col = col.push(text(&self.line_5_comm).size(TEXT_SIZE)
                                                     .wrapping(Word));
            }
        }

        // Line 4
        col = col.push(row![button(svg("./images/button.svg"))
                                .on_press(Message::FoldSec(Fold::Line4))
                                .style_heading(),
                            text("Line 4")
                                .size(HEADING_SIZE)
                                .wrapping(Word)
                           ]
        );
        if display[11] {
            col = col.push(text(&self.line_4).size(TEXT_SIZE).wrapping(Word));
            col = col.push(row![button(svg("./images/button.svg"))
                                    .on_press(Message::FoldSec(Fold::Line4Comm))
                                    .style_subheading(),
                                text("Line 4 Commentary")
                                    .size(SUBHEADING_SIZE)
                                    .wrapping(Word)
                               ]
            );
            if display[12] {
                col = col.push(text(&self.line_4_comm).size(TEXT_SIZE)
                                                     .wrapping(Word));
            }
        }

        // Line 3
        col = col.push(row![button(svg("./images/button.svg"))
                                .on_press(Message::FoldSec(Fold::Line3))
                                .style_heading(),
                            text("Line 3")
                                .size(HEADING_SIZE)
                                .wrapping(Word)
                           ]
        );
        if display[9] {
            col = col.push(text(&self.line_3).size(TEXT_SIZE).wrapping(Word));
            col = col.push(row![button(svg("./images/button.svg"))
                                    .on_press(Message::FoldSec(Fold::Line3Comm))
                                    .style_subheading(),
                                text("Line 3 Commentary")
                                    .size(SUBHEADING_SIZE)
                                    .wrapping(Word)
                               ]
            );
            if display[10] {
                col = col.push(text(&self.line_3_comm).size(TEXT_SIZE)
                                                     .wrapping(Word));
            }
        }

        // Line 2
        col = col.push(row![button(svg("./images/button.svg"))
                                .on_press(Message::FoldSec(Fold::Line2))
                                .style_heading(),
                            text("Line 2")
                                .size(HEADING_SIZE)
                                .wrapping(Word)
                           ]
        );
        if display[7] {
            col = col.push(text(&self.line_2).size(TEXT_SIZE).wrapping(Word));
            col = col.push(row![button(svg("./images/button.svg"))
                                    .on_press(Message::FoldSec(Fold::Line2Comm))
                                    .style_subheading(),
                                text("Line 2 Commentary")
                                    .size(SUBHEADING_SIZE)
                                    .wrapping(Word)
                               ]
            );
            if display[8] {
                col = col.push(text(&self.line_2_comm).size(TEXT_SIZE)
                                                     .wrapping(Word));
            }
        }

        // Line 1
        col = col.push(row![button(svg("./images/button.svg"))
                                .on_press(Message::FoldSec(Fold::Line1))
                                .style_heading(),
                            text("Line 1")
                                .size(HEADING_SIZE)
                                .wrapping(Word)
                           ]
        );
        if display[5] {
            col = col.push(text(&self.line_1).size(TEXT_SIZE).wrapping(Word));
            col = col.push(row![button(svg("./images/button.svg"))
                                    .on_press(Message::FoldSec(Fold::Line1Comm))
                                    .style_subheading(),
                                text("Line 1 Commentary")
                                    .size(SUBHEADING_SIZE)
                                    .wrapping(Word)
                               ]
            );
            if display[6] {
                col = col.push(text(&self.line_1_comm).size(TEXT_SIZE)
                                                     .wrapping(Word));
            }
        }

        col = col.push(button(text("Back").size(BUTTON_SIZE_SUBHEADING))
                        .on_press(Message::Back));

        col.width(800)
    }

    fn trigram_svg<'a>(name: &'a str, color: &'a Color) -> Column<'a, Message> {
        column![svg(format!("./images/{}.svg", name))
                    .style(|_,_| svg::Style {color: Some(*color)})
                    .width(HEX_SIZE)
                    .height(HEX_SIZE)
                    .content_fit(ContentFit::Cover),
                text(name).size(INDEX_SIZE).wrapping(Word),
                Space::new().height(Length::Fixed(10.0))// :Y_
               ].align_x(Horizontal::Center).width(Length::Fixed(COL_WIDTH))
    }

    fn hex_button<'a>(num : u8, text_color : &'a Color, book : &Changes)
        -> Column<'a, Message> {
        let button_style = button::Style{
            background : None,
            text_color : *text_color,
            border : Border {
                color : Color::TRANSPARENT,
                width : 0.0,
                radius : Radius {
                    top_left : 0.0,
                    top_right : 0.0,
                    bottom_left : 0.0,
                    bottom_right : 0.0
                }
            },
            shadow : Shadow {
                    color : Color::TRANSPARENT,
                    offset : Vector::ZERO,
                    blur_radius : 0.0
                },
            snap : true
        };
        let button_style_clone = button_style.clone();
        let name = book.find_by_num(num).name;
        column![
                button(Hex::hex_svg(num, HEX_SIZE as f32,
                                    text_color))
                    .style(move |_,_| {button_style_clone})
                    .on_press(Message::ToHex(num)),
                button(text(format!("{}: {}", num, name))
                        .size(INDEX_SIZE)
                        .wrapping(Word))
                    .style(move |_,_| {button_style_clone})
                    .on_press(Message::ToHex(num)),
                Space::new().height(Length::Fixed(10.0))// :Y_
               ].align_x(Horizontal::Center).width(Length::Fixed(COL_WIDTH))
    }

    fn hex_svg<'a>(num : u8, size : f32, color : &'a Color) -> Svg<'a> {
        svg(format!("./images/{}.svg", num))
            .style(|_,_| svg::Style {color : Some(*color)})
            .width(Length::Fixed(size))
            .height(Length::Fixed(size))
            .content_fit(ContentFit::Cover)
    }
}

pub struct ChangesGUI {
    book : Changes,
    state : State,
    prev_state : State,
    display : [bool; 17],
    theme : Themes,
    text_color : Color
}

impl ChangesGUI {
    pub fn new(book : Changes) -> Self {
        Self {
            book : book,
            state : State::Start,
            prev_state : State::Start,
            display : DISPLAY_DEFAULT,
            theme : Themes::Dark,
            text_color : Theme::Dark.palette().text
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::FoldSec(sec) => {
                match sec {
                    Fold::Preface => { self.display[0].toggle(); }
                    Fold::Judgment => { self.display[1].toggle(); }
                    Fold::JudgmentComm => { self.display[2].toggle(); }
                    Fold::Image => { self.display[3].toggle(); }
                    Fold::ImageComm => { self.display[4].toggle(); }
                    Fold::Line1 => { self.display[5].toggle(); }
                    Fold::Line1Comm => { self.display[6].toggle(); }
                    Fold::Line2 => { self.display[7].toggle(); }
                    Fold::Line2Comm => { self.display[8].toggle(); }
                    Fold::Line3 => { self.display[9].toggle(); }
                    Fold::Line3Comm => { self.display[10].toggle(); }
                    Fold::Line4 => { self.display[11].toggle(); }
                    Fold::Line4Comm => { self.display[12].toggle(); }
                    Fold::Line5 => { self.display[13].toggle(); }
                    Fold::Line5Comm => { self.display[14].toggle(); }
                    Fold::Line6 => { self.display[15].toggle(); }
                    Fold::Line6Comm => { self.display[16].toggle(); }
                }
            }
            Message::ThemeSelect(theme) => {
                self.theme = theme;
                self.text_color = self.theme().palette().text;
            }
            Message::ToDraw => {
                // Draw a hexagram
                let (lines, moving, _) = Changes::make_hex();
                let hex = self.book.find_by_lines(&lines);
                for (i, line) in moving.into_iter().enumerate() {
                    if line == 1 {
                        self.display[5 + 2*i] = true;
                    }
                }
                self.prev_state = self.state.clone();
                self.state = State::Display(hex);
            }
            Message::ToHex(num) => {
                self.prev_state = self.state.clone();
                self.state = State::Display(self.book.find_by_num(num));
            }
            Message::ToIndex => {
                // Display hexagram index
                self.prev_state = self.state.clone();
                self.state = State::Index;
            }
            Message::Back => {
                if let State::Display(_) = self.state {
                    self.display = DISPLAY_DEFAULT;
                }
                if let State::Display(_) = self.prev_state
                       && self.state == State::Index {
                    self.prev_state = State::Index;
                    self.state = State::Start;
                }
                else {
                    let temp_state = self.prev_state.clone();
                    self.prev_state = self.state.clone();
                    self.state = temp_state;
                }
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        center(scrollable(match &self.state {
            State::Display(hex) => {
                hex.view(&self.display, &self.text_color)
            }
            State::Index => {
                column![
                    // Row 1
                    row![
                        column![
                                button(text("Back").size(BUTTON_SIZE_SMALL))
                                    .on_press(Message::Back),
                                Space::new().height(Length::Fixed(5.0)),// :Y_
                                text("Upper ->").size(INDEX_SIZE),
                                text("Lower v").size(INDEX_SIZE),
                                Space::new().height(Length::Fixed(10.0))// :Y_
                               ]
                            .align_x(Horizontal::Center)
                            .width(Length::Fixed(COL_WIDTH)),
                        Hex::trigram_svg("The Creative", &self.text_color),
                        Hex::trigram_svg("The Joyful", &self.text_color),
                        Hex::trigram_svg("The Clinging", &self.text_color),
                        Hex::trigram_svg("The Arousing", &self.text_color),
                        Hex::trigram_svg("The Penetrating", &self.text_color),
                        Hex::trigram_svg("The Abyssmal", &self.text_color),
                        Hex::trigram_svg("Standing Still", &self.text_color),
                        Hex::trigram_svg("The Receptive", &self.text_color),
                    ].spacing(ROW_SPACING).align_y(Vertical::Center),
                    // Row 2
                    row![
                        Hex::trigram_svg("The Creative", &self.text_color),
                        Hex::hex_button(1, &self.text_color, &self.book),
                        Hex::hex_button(43, &self.text_color, &self.book),
                        Hex::hex_button(14, &self.text_color, &self.book),
                        Hex::hex_button(34, &self.text_color, &self.book),
                        Hex::hex_button(9, &self.text_color, &self.book),
                        Hex::hex_button(5, &self.text_color, &self.book),
                        Hex::hex_button(26, &self.text_color, &self.book),
                        Hex::hex_button(11, &self.text_color, &self.book),
                    ].spacing(ROW_SPACING).align_y(Vertical::Center),
                    // Row 3
                    row![
                        Hex::trigram_svg("The Joyful", &self.text_color),
                        Hex::hex_button(10, &self.text_color, &self.book),
                        Hex::hex_button(58, &self.text_color, &self.book),
                        Hex::hex_button(38, &self.text_color, &self.book),
                        Hex::hex_button(54, &self.text_color, &self.book),
                        Hex::hex_button(61, &self.text_color, &self.book),
                        Hex::hex_button(60, &self.text_color, &self.book),
                        Hex::hex_button(41, &self.text_color, &self.book),
                        Hex::hex_button(19, &self.text_color, &self.book),
                    ].spacing(ROW_SPACING).align_y(Vertical::Center),
                    // Row 4
                    row![
                        Hex::trigram_svg("The Clinging", &self.text_color),
                        Hex::hex_button(13, &self.text_color, &self.book),
                        Hex::hex_button(49, &self.text_color, &self.book),
                        Hex::hex_button(30, &self.text_color, &self.book),
                        Hex::hex_button(55, &self.text_color, &self.book),
                        Hex::hex_button(37, &self.text_color, &self.book),
                        Hex::hex_button(63, &self.text_color, &self.book),
                        Hex::hex_button(22, &self.text_color, &self.book),
                        Hex::hex_button(36, &self.text_color, &self.book),
                    ].spacing(ROW_SPACING).align_y(Vertical::Center),
                    // Row 5
                    row![
                        Hex::trigram_svg("The Arousing", &self.text_color),
                        Hex::hex_button(25, &self.text_color, &self.book),
                        Hex::hex_button(17, &self.text_color, &self.book),
                        Hex::hex_button(21, &self.text_color, &self.book),
                        Hex::hex_button(51, &self.text_color, &self.book),
                        Hex::hex_button(42, &self.text_color, &self.book),
                        Hex::hex_button(3, &self.text_color, &self.book),
                        Hex::hex_button(27, &self.text_color, &self.book),
                        Hex::hex_button(24, &self.text_color, &self.book),
                    ].spacing(ROW_SPACING).align_y(Vertical::Center),
                    // Row 6
                    row![
                        Hex::trigram_svg("The Penetrating", &self.text_color),
                        Hex::hex_button(44, &self.text_color, &self.book),
                        Hex::hex_button(28, &self.text_color, &self.book),
                        Hex::hex_button(50, &self.text_color, &self.book),
                        Hex::hex_button(32, &self.text_color, &self.book),
                        Hex::hex_button(57, &self.text_color, &self.book),
                        Hex::hex_button(48, &self.text_color, &self.book),
                        Hex::hex_button(18, &self.text_color, &self.book),
                        Hex::hex_button(46, &self.text_color, &self.book),
                    ].spacing(ROW_SPACING).align_y(Vertical::Center),
                    // Row 7
                    row![
                        Hex::trigram_svg("The Abyssmal", &self.text_color),
                        Hex::hex_button(6, &self.text_color, &self.book),
                        Hex::hex_button(47, &self.text_color, &self.book),
                        Hex::hex_button(64, &self.text_color, &self.book),
                        Hex::hex_button(40, &self.text_color, &self.book),
                        Hex::hex_button(59, &self.text_color, &self.book),
                        Hex::hex_button(29, &self.text_color, &self.book),
                        Hex::hex_button(4, &self.text_color, &self.book),
                        Hex::hex_button(7, &self.text_color, &self.book),
                    ].spacing(ROW_SPACING).align_y(Vertical::Center),
                    // Row 8
                    row![
                        Hex::trigram_svg("Standing Still", &self.text_color),
                        Hex::hex_button(33, &self.text_color, &self.book),
                        Hex::hex_button(31, &self.text_color, &self.book),
                        Hex::hex_button(56, &self.text_color, &self.book),
                        Hex::hex_button(62, &self.text_color, &self.book),
                        Hex::hex_button(53, &self.text_color, &self.book),
                        Hex::hex_button(39, &self.text_color, &self.book),
                        Hex::hex_button(52, &self.text_color, &self.book),
                        Hex::hex_button(15, &self.text_color, &self.book),
                    ].spacing(ROW_SPACING).align_y(Vertical::Center),
                    // Row 9
                    row![
                        Hex::trigram_svg("The Receptive", &self.text_color),
                        Hex::hex_button(12, &self.text_color, &self.book),
                        Hex::hex_button(45, &self.text_color, &self.book),
                        Hex::hex_button(35, &self.text_color, &self.book),
                        Hex::hex_button(16, &self.text_color, &self.book),
                        Hex::hex_button(20, &self.text_color, &self.book),
                        Hex::hex_button(8, &self.text_color, &self.book),
                        Hex::hex_button(23, &self.text_color, &self.book),
                        Hex::hex_button(2, &self.text_color, &self.book),
                    ].spacing(ROW_SPACING).align_y(Vertical::Center),
                ]
                .spacing(15)
            }
            State::Start => {
                column![
                    text("The Book of Changes").size(TITLE_SIZE),
                    row![
                        button(text("Draw").size(BUTTON_SIZE_SUBHEADING))
                            .on_press(Message::ToDraw),
                        button(text("Index").size(BUTTON_SIZE_SUBHEADING))
                            .on_press(Message::ToIndex)
                    ].spacing(15),
                    pick_list(Themes::ALL, Some(self.theme),
                         Message::ThemeSelect)
                ]
                .spacing(25)
            }
        }
        .padding(Padding::from([50, 50]))
        .align_x(Horizontal::Center)
        ).direction(Direction::Both{vertical: Scrollbar::default(),
                                    horizontal: Scrollbar::default()})
        )
        .into()
    }

    pub fn theme(&self) -> Theme {
        match self.theme {
            Themes::Light => {Theme::Light}
            Themes::Dark => {Theme::Dark}
            Themes::Dracula => {Theme::Dracula}
            Themes::Nord => {Theme::Nord}
            Themes::SolarizedLight => {Theme::SolarizedLight}
            Themes::SolarizedDark => {Theme::SolarizedDark}
            Themes::GruvboxLight => {Theme::GruvboxLight}
            Themes::GruvboxDark => {Theme::GruvboxDark}
            Themes::CatppuccinLatte => {Theme::CatppuccinLatte}
            Themes::CatppuccinFrappe => {Theme::CatppuccinFrappe}
            Themes::CatppuccinMacchiato => {Theme::CatppuccinMacchiato}
            Themes::CatppuccinMocha => {Theme::CatppuccinMocha}
            Themes::TokyoNight => {Theme::TokyoNight}
            Themes::TokyoNightStorm => {Theme::TokyoNightStorm}
            Themes::TokyoNightLight => {Theme::TokyoNightLight}
            Themes::KanagawaWave => {Theme::KanagawaWave}
            Themes::KanagawaDragon => {Theme::KanagawaDragon}
            Themes::KanagawaLotus => {Theme::KanagawaLotus}
            Themes::Moonfly => {Theme::Moonfly}
            Themes::Nightfly => {Theme::Nightfly}
            Themes::Oxocarbon => {Theme::Oxocarbon}
            Themes::Ferra => {Theme::Ferra}
        }
    }
}

impl std::default::Default for ChangesGUI {
    fn default() -> Self {
        Self::new(Changes::new())
    }
}

/// A trait for simple functions to consistently style buttons
trait ButtonStyling {
    fn style_heading(self) -> Self;
    fn style_subheading(self) -> Self;
}

/// Simple functions to consistently style buttons
impl ButtonStyling for Button<'_, Message> {
    fn style_heading(mut self) -> Self {
        self = self.style(|_, _| { BUTTON_STYLE })
                   .padding(BUTTON_PADDING_HEADING)
                   .height(BUTTON_SIZE_HEADING)
                   .width(BUTTON_SIZE_HEADING);
        self
    }

    fn style_subheading(mut self) -> Self {
        self = self.style(|_, _| { BUTTON_STYLE })
                   .padding(BUTTON_PADDING_SUBHEADING)
                   .height(BUTTON_SIZE_SUBHEADING)
                   .width(BUTTON_SIZE_SUBHEADING);
        self
    }
}

/// A trait for a simple convenience function to toggle a bool
trait Toggle {
    fn toggle(&mut self);
}

/// A simple convenience function to toggle a bool
impl Toggle for bool {
    fn toggle(&mut self) {
        *self = !*self;
    }
}
