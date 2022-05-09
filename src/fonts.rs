use iced::Font;

pub const WQY_CN: Font = Font::External {
    name: "WenQuanYi Micro Hei Regular",
    bytes: include_bytes!("../assets/fonts/wqy-microhei.ttc"),
};

pub const CINZEL_DECORATIVE: Font = Font::External {
    name: "Cinzel Decorative",
    bytes: include_bytes!("../assets/fonts/CinzelDecorative-Regular.ttf"),
};
