use iced::{Font, HorizontalAlignment, Length, Text};

const ICONS: Font = Font::External {
    name: "Icons",
    bytes: include_bytes!("../../static/icons/bootstrap-icons.ttf"),
};

fn icon(unicode: char) -> Text {
    Text::new(&unicode.to_string())
        .font(ICONS)
        .width(Length::Units(20))
        .horizontal_alignment(HorizontalAlignment::Center)
        .size(20)
}

pub fn home_icon() -> Text {
    icon('\u{F3DC}')
}

pub fn send_icon() -> Text {
    icon('\u{F144}')
}

pub fn deposit_icon() -> Text {
    icon('\u{F123}')
}

#[allow(dead_code)]
pub fn withdrawal_icon() -> Text {
    icon('\u{F144}')
}

pub fn turnback_icon() -> Text {
    icon('\u{F131}')
}

pub fn history_icon() -> Text {
    icon('\u{F292}')
}

pub fn settings_icon() -> Text {
    icon('\u{F3C5}')
}

pub fn block_icon() -> Text {
    icon('\u{F1C8}')
}

pub fn network_icon() -> Text {
    icon('\u{F3ED}')
}

pub fn dot_icon() -> Text {
    icon('\u{F287}')
}

pub fn clipboard_icon() -> Text {
    icon('\u{F28E}')
}

pub fn shield_icon() -> Text {
    icon('\u{F517}')
}

pub fn shield_notif_icon() -> Text {
    icon('\u{F50A}')
}

pub fn shield_check_icon() -> Text {
    icon('\u{F509}')
}

#[allow(dead_code)]
pub fn stakeholder_icon() -> Text {
    icon('\u{F4AE}')
}

#[allow(dead_code)]
pub fn manager_icon() -> Text {
    icon('\u{F4B4}')
}
