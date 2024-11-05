use crate::{Char, Nil};

macro_rules! declare_char_types {
    ($(
        $(#[$meta:meta])*
        $name:ident => $char:literal
    ),* $(,)?) => {
        $(
            $(#[$meta])*
            pub type $name<T = Nil> = Char<$char, T>;
        )*
    }
}

declare_char_types!(
    Null => '\0',
    StartOfHeading => '\u{0001}',
    StartOfText => '\u{0002}',
    EndOfText => '\u{0003}',
    EndOfTransmission => '\u{0004}',
    Enquiry => '\u{0005}',
    Acknowledge => '\u{0006}',
    Bell => '\u{0007}',
    Backspace => '\u{0008}',
    // \t
    #[doc(alias = "CharacterTabulation")]
    Tab => '\u{0009}',
    // \n
    #[doc(alias = "LineFeed", alias = "NewLine")]
    Nl => '\u{000A}',
    LineTabulation => '\u{000B}',
    FormFeed => '\u{000C}',
    CarriageReturn => '\u{000D}',
    ShiftOut => '\u{000E}',
    ShiftIn => '\u{000F}',
    DataLinkEscape => '\u{0010}',
    DeviceControlOne => '\u{0011}',
    DeviceControlTwo => '\u{0012}',
    DeviceControlThree => '\u{0013}',
    DeviceControlFour => '\u{0014}',
    NegativeAcknowledge => '\u{0015}',
    SynchronousIdle => '\u{0016}',
    EndOfTransmissionBlock => '\u{0017}',
    Cancel => '\u{0018}',
    EndOfMedium => '\u{0019}',
    Substitute => '\u{001A}',
    Escape => '\u{001B}',
    InformationSeparatorFour => '\u{001C}',
    InformationSeparatorThree => '\u{001D}',
    InformationSeparatorTwo => '\u{001E}',
    InformationSeparatorOne => '\u{001F}',
    Space => ' ',
    ExclamationMark => '!',
    QuotationMark => '"',
    #[doc(alias = "Hashtag")]
    NumberSign => '#',
    #[doc(alias = "DollarSign")]
    Dollar => '$',
    #[doc(alias = "PercentSign")]
    Percent => '%',
    Ampersand => '&',
    Apostrophe => '\'',
    LeftParenthesis => '(',
    RightParenthesis => ')',
    Asterisk => '*',
    #[doc(alias = "PlusSign")]
    Plus => '+',
    Comma => ',',
    #[doc(alias = "HyphenMinus")]
    Minus => '-',
    #[doc(alias = "FullStop")]
    Dot => '.',
    #[doc(alias = "Solidus")]
    Slash => '/',
);

declare_char_types! {
    _0 => '0',
    _1 => '1',
    _2 => '2',
    _3 => '3',
    _4 => '4',
    _5 => '5',
    _6 => '6',
    _7 => '7',
    _8 => '8',
    _9 => '9',
}

declare_char_types!(
    Colon => ':',
    Semicolon => ';',
    #[doc(alias = "LessThanSign")]
    LeftArrow => '<',
    EqualsSign => '=',
    #[doc(alias = "GreaterThanSign")]
    RightArrow => '>',
    QuestionMark => '?',
    CommercialAt => '@',
);

declare_char_types! {
    A => 'A',
    B => 'B',
    C => 'C',
    D => 'D',
    E => 'E',
    F => 'F',
    G => 'G',
    H => 'H',
    I => 'I',
    J => 'J',
    K => 'K',
    L => 'L',
    M => 'M',
    N => 'N',
    O => 'O',
    P => 'P',
    Q => 'Q',
    R => 'R',
    S => 'S',
    T => 'T',
    U => 'U',
    V => 'V',
    W => 'W',
    X => 'X',
    Y => 'Y',
    Z => 'Z',
}

declare_char_types!(
    LeftSquareBracket => '[',
    ReverseSolidus => '\\',
    RightSquareBracket => ']',
    #[doc(alias = "CircumflexAccent")]
    Caret => '^',
    #[doc(alias = "LowLine")]
    Underscore => '_',
    #[doc(alias = "GraveAccent")]
    Backtick => '`',
);

declare_char_types! {
    a => 'a',
    b => 'b',
    c => 'c',
    d => 'd',
    e => 'e',
    f => 'f',
    g => 'g',
    h => 'h',
    i => 'i',
    j => 'j',
    k => 'k',
    l => 'l',
    m => 'm',
    n => 'n',
    o => 'o',
    p => 'p',
    q => 'q',
    r => 'r',
    s => 's',
    t => 't',
    u => 'u',
    v => 'v',
    w => 'w',
    x => 'x',
    y => 'y',
    z => 'z',
}

declare_char_types!(
    LeftCurlyBracket => '{',
    #[doc(alias = "VerticalLine")]
    Pipe => '|',
    RightCurlyBracket => '}',
    Tilde => '~',
    Delete => '\u{007F}',
);

#[cfg(feature = "ascii-extended")]
pub mod extended {
    //! ASCII extended characters

    use super::*;

    declare_char_types!(
        Ç => '\u{00C7}',
        ü => '\u{00FC}',
        é => '\u{00E9}',
        â => '\u{00E2}',
        ä => '\u{00E4}',
        à => '\u{00E0}',
        å => '\u{00E5}',
        ç => '\u{00E7}',
        ê => '\u{00EA}',
        ë => '\u{00EB}',
        è => '\u{00E8}',
        ï => '\u{00EF}',
        î => '\u{00EE}',
        ì => '\u{00EC}',
        Ä => '\u{00C4}',
        Å => '\u{00C5}',
        É => '\u{00C9}',
        æ => '\u{00E6}',
        Æ => '\u{00C6}',
        ô => '\u{00F4}',
        ö => '\u{00F6}',
        ò => '\u{00F2}',
        û => '\u{00FB}',
        ù => '\u{00F9}',
        ÿ => '\u{00FF}',
        Ö => '\u{00D6}',
        Ü => '\u{00DC}',
        ø => '\u{00F8}',
        /// £
        Pound => '\u{00A3}',
        Ø => '\u{00D8}',
        /// ×
        Times => '\u{00D7}',
        ƒ => '\u{0192}',
        á => '\u{00E1}',
        í => '\u{00ED}',
        ó => '\u{00F3}',
        ú => '\u{00FA}',
        ñ => '\u{00F1}',
        Ñ => '\u{00D1}',
        ª => '\u{00AA}',
        /// º
        º => '\u{00BA}',
        /// ¿
        InvertedQuestionMark => '\u{00BF}',
        /// ®
        Registered => '\u{00AE}',
        /// ¬
        NotSign => '\u{00AC}',
        /// ½
        OneHalf => '\u{00BD}',
        /// ¼
        OneQuarter => '\u{00BC}',
        /// ¡
        InvertedExclamationMark => '\u{00A1}',
        /// «
        LeftDoubleAngleQuote => '\u{00AB}',
        /// »
        RightDoubleAngleQuote => '\u{00BB}',
        /// ░
        LightShade => '\u{2591}',
        /// ▒
        MediumShade => '\u{2592}',
        /// ▓
        DarkShade => '\u{2593}',
        /// │
        VerticalBar => '\u{2502}',
        /// ┤
        RightT => '\u{2524}',
        Aacute => '\u{00C1}',
        Acircumflex => '\u{00C2}',
        Agrave => '\u{00C0}',
        /// ©
        Copyright => '\u{00A9}',
        /// ╣
        RightDoubleLine => '\u{2563}',
        /// ║
        DoubleVerticalLine => '\u{2551}',
        /// ╗
        TopRightDoubleLine => '\u{2557}',
        /// ╝
        BottomRightDoubleLine => '\u{255D}',
        /// ¢
        Cent => '\u{00A2}',
        /// ¥
        Yen => '\u{00A5}',
        /// ┐
        TopRight => '\u{2510}',
        /// └
        BottomLeft => '\u{2514}',
        /// ┴
        BottomT => '\u{2534}',
        /// ┬
        TopT => '\u{252C}',
        /// ├
        LeftT => '\u{251C}',
        /// ─
        HorizontalBar => '\u{2500}',
        /// ┼
        Cross => '\u{253C}',
        ã => '\u{00E3}',
        Ã => '\u{00C3}',
        /// ╚
        BottomLeftDoubleLine => '\u{255A}',
        /// ╔
        TopLeftDoubleLine => '\u{2554}',
        /// ╩
        BottomDoubleT => '\u{2569}',
        /// ╦
        TopDoubleT => '\u{2566}',
        /// ╠
        LeftDoubleT => '\u{2560}',
        /// ═
        DoubleHorizontalLine => '\u{2550}',
        /// ╬
        DoubleCross => '\u{256C}',
        /// ¤
        CurrencySign => '\u{00A4}',
        ð => '\u{00F0}',
        Ð => '\u{00D0}',
        Ê => '\u{00CA}',
        Ë => '\u{00CB}',
        È => '\u{00C8}',
        ı => '\u{0131}',
        Í => '\u{00CD}',
        Î => '\u{00CE}',
        Ï => '\u{00CF}',
        /// ┘
        RightCorner => '\u{2518}',
        /// ┌
        LeftCorner => '\u{250C}',
        /// █
        FullBlock => '\u{2588}',
        /// ▄
        LowerHalfBlock => '\u{2584}',
        /// ¦
        BrokenBar => '\u{00A6}',
        /// Ì
        IGrave => '\u{00CC}',
        /// ▀
        UpperHalfBlock => '\u{2580}',
        Ó => '\u{00D3}',
        ß => '\u{00DF}',
        Ô => '\u{00D4}',
        Ò => '\u{00D2}',
        õ => '\u{00F5}',
        Õ => '\u{00D5}',
        µ => '\u{00B5}',
        þ => '\u{00FE}',
        Þ => '\u{00DE}',
        Ú => '\u{00DA}',
        Û => '\u{00DB}',
        Ù => '\u{00D9}',
        ý => '\u{00FD}',
        Ý => '\u{00DD}',
        /// ¯
        Macron => '\u{00AF}',
        /// ´
        AcuteAccent => '\u{00B4}',
        /// ­­
        SoftHyphen => '\u{00AD}',
        /// ±
        PlusMinus => '\u{00B1}',
        /// ‗
        DoubleLowLine => '\u{2017}',
        /// ¾
        ThreeQuarters => '\u{00BE}',
        /// ¶
        Paragraph => '\u{00B6}',
        /// §
        Section => '\u{00A7}',
        /// ÷
        Division => '\u{00F7}',
        /// ¸
        Cedilla => '\u{00B8}',
        /// °
        Degree => '\u{00B0}',
        /// ¨
        Diaeresis => '\u{00A8}',
        /// ·
        MiddleDot => '\u{00B7}',
        /// ¹
        SuperscriptOne => '\u{00B9}',
        /// ³
        SuperscriptThree => '\u{00B3}',
        /// ²
        SuperscriptTwo => '\u{00B2}',
        /// ■
        BlackSquare => '\u{25A0}',
        nbsp => '\u{00A0}',
    );
}
