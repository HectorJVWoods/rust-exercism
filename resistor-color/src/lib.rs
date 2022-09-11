#[derive(Debug, PartialEq, Eq)]
pub enum ResistorColor {
    Black,
    Blue,
    Brown,
    Green,
    Grey,
    Orange,
    Red,
    Violet,
    White,
    Yellow,
}

pub fn color_to_value(_color: ResistorColor) -> u32 {
    match _color {
        ResistorColor::Black => 0,
        ResistorColor::Brown => 1,
        ResistorColor::Red => 2,
        ResistorColor::Orange => 3,
        ResistorColor::Yellow => 4,
        ResistorColor::Green => 5,
        ResistorColor::Blue => 6,
        ResistorColor::Violet => 7,
        ResistorColor::Grey => 8,
        ResistorColor::White => 9,
    }
}

fn color_to_string(_color: ResistorColor) -> String {
    match _color {
        ResistorColor::Black => "Black",
        ResistorColor::Brown => "Brown",
        ResistorColor::Red => "Red",
        ResistorColor::Orange => "Orange",
        ResistorColor::Yellow => "Yellow",
        ResistorColor::Green => "Green",
        ResistorColor::Blue => "Blue",
        ResistorColor::Violet => "Violet",
        ResistorColor::Grey => "Grey",
        ResistorColor::White => "White",
    }
    .to_string()
}

fn value_to_color(value : u32) -> ResistorColor {
    match value {
        0 => ResistorColor::Black,
        1 => ResistorColor::Brown,
        2 => ResistorColor::Red,
        3 => ResistorColor::Orange,
        4 => ResistorColor::Yellow,
        5 => ResistorColor::Green,
        6 => ResistorColor::Blue,
        7 => ResistorColor::Violet,
        8 => ResistorColor::Grey,
        9 => ResistorColor::White,
        _ => panic!("value out of range"),
    }
}

pub fn value_to_color_string(value: u32) -> String {
    if value > 9 {
        return "value out of range".to_string();
    }
    color_to_string(value_to_color(value))
}

pub fn colors() -> Vec<ResistorColor> {
    vec![
        ResistorColor::Black,
        ResistorColor::Brown,
        ResistorColor::Red,
        ResistorColor::Orange,
        ResistorColor::Yellow,
        ResistorColor::Green,
        ResistorColor::Blue,
        ResistorColor::Violet,
        ResistorColor::Grey,
        ResistorColor::White,
    ]
}




