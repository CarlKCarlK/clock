/// An empty struct useful for defining constants related to 7-segment LED displays.
pub struct Leds;

/// Constants of interest for a 7-segment LED display.
impl Leds {
    /// Segment A of the 7-segment display.
    pub const SEG_A: u8 = 0b_0000_0001;
    /// Segment B of the 7-segment display.
    pub const SEG_B: u8 = 0b_0000_0010;
    /// Segment C of the 7-segment display.
    pub const SEG_C: u8 = 0b_0000_0100;
    /// Segment D of the 7-segment display.
    pub const SEG_D: u8 = 0b_0000_1000;
    /// Segment E of the 7-segment display.
    pub const SEG_E: u8 = 0b_0001_0000;
    /// Segment F of the 7-segment display.
    pub const SEG_F: u8 = 0b_0010_0000;
    /// Segment G of the 7-segment display.
    pub const SEG_G: u8 = 0b_0100_0000;
    /// Decimal point of the 7-segment display.
    pub const DECIMAL: u8 = 0b_1000_0000;

    /// Array representing the segments for digits 0-9 on a 7-segment display.
    pub const DIGITS: [u8; 10] = [
        0b_0011_1111, // Digit 0
        0b_0000_0110, // Digit 1
        0b_0101_1011, // Digit 2
        0b_0100_1111, // Digit 3
        0b_0110_0110, // Digit 4
        0b_0110_1101, // Digit 5
        0b_0111_1101, // Digit 6
        0b_0000_0111, // Digit 7
        0b_0111_1111, // Digit 8
        0b_0110_1111, // Digit 9
    ];

    /// Representation of a blank space on a 7-segment display.
    pub const SPACE: u8 = 0b_0000_0000;

    /// ASCII table mapping characters to their 7-segment display representations.
    /// Control characters (0-31) and delete (127) are represented as blank spaces.
    /// Uppercase and lowercase letters are mostly mapped to the same segments for simplicity.
    pub const ASCII_TABLE: [u8; 128] = [
        // Control characters (0-31) + space (32)
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_0000, // 0-4
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_0000, // 5-9
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_0000, // 10-14
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_0000, // 15-19
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_0000, //  20-24
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_0000, //  25-29
        0b_0000_0000,
        0b_0000_0000,
        0b_0000_0000, // 30-32
        // Symbols (33-47)
        0b_1000_0110, // !
        0b_0000_0000, // "
        0b_0000_0000, // #
        0b_0000_0000, // $
        0b_0000_0000, // %
        0b_0000_0000, // &
        0b_0000_0000, // '
        0b_0000_0000, // (
        0b_0000_0000, // )
        0b_0000_0000, // *
        0b_0000_0000, // +
        0b_0000_0000, // ,
        0b_0100_0000, // -
        0b_1000_0000, // .
        0b_0000_0000, // /
        // Numbers (48-57)
        0b_0011_1111, // 0
        0b_0000_0110, // 1
        0b_0101_1011, // 2
        0b_0100_1111, // 3
        0b_0110_0110, // 4
        0b_0110_1101, // 5
        0b_0111_1101, // 6
        0b_0000_0111, // 7
        0b_0111_1111, // 8
        0b_0110_1111, // 9
        // Symbols (58-64)
        0b_0000_0000, // :
        0b_0000_0000, // ;
        0b_0000_0000, // <
        0b_0000_0000, // =
        0b_0000_0000, // >
        0b_0000_0000, // ?
        0b_0000_0000, // @
        // Uppercase letters (65-90)
        0b_0111_0111, // A
        0b_0111_1100, // B (same as b)
        0b_0011_1001, // C
        0b_0101_1110, // D (same as d)
        0b_0111_1001, // E
        0b_0111_0001, // F
        0b_0011_1101, // G (same as 9)
        0b_0111_0110, // H
        0b_0000_0110, // I (same as 1)
        0b_0001_1110, // J
        0b_0111_0110, // K (approximation)
        0b_0011_1000, // L
        0b_0001_0101, // M (arbitrary, no good match)
        0b_0101_0100, // N
        0b_0011_1111, // O (same as 0)
        0b_0111_0011, // P
        0b_0110_0111, // Q
        0b_0101_0000, // R
        0b_0110_1101, // S (same as 5)
        0b_0111_1000, // T
        0b_0011_1110, // U
        0b_0010_1010, // V (arbitrary, no good match)
        0b_0001_1101, // W (arbitrary, no good match)
        0b_0111_0110, // X (same as H)
        0b_0110_1110, // Y
        0b_0101_1011, // Z (same as 2)
        // Symbols (91-96)
        0b_0011_1001, // [
        0b_0000_0000, // \
        0b_0000_1111, // ]
        0b_0000_0000, // ^
        0b_0000_1000, // _
        0b_0000_0000, // `
        // Lowercase letters (97-122), mostly reusing uppercase for simplicity
        0b_0111_0111, // A
        0b_0111_1100, // B (same as b)
        0b_0011_1001, // C
        0b_0101_1110, // D (same as d)
        0b_0111_1001, // E
        0b_0111_0001, // F
        0b_0011_1101, // G (same as 9)
        0b_0111_0100, // H
        0b_0000_0110, // I (same as 1)
        0b_0001_1110, // J
        0b_0111_0110, // K (approximation)
        0b_0011_1000, // L
        0b_0001_0101, // M (arbitrary, no good match)
        0b_0101_0100, // N
        0b_0011_1111, // O (same as 0)
        0b_0111_0011, // P
        0b_0110_0111, // Q
        0b_0101_0000, // R
        0b_0110_1101, // S (same as 5)
        0b_0111_1000, // T
        0b_0011_1110, // U
        0b_0010_1010, // V (arbitrary, no good match)
        0b_0001_1101, // W (arbitrary, no good match)
        0b_0111_0110, // X (same as H)
        0b_0110_1110, // Y
        0b_0101_1011, // Z (same as 2)
        // Placeholder for simplicity
        0b_0011_1001, // '{' (123)
        0b_0000_0110, // '|' (124)
        0b_0000_1111, // '}' (125)
        0b_0100_0000, // '~' (126)
        0b_0000_0000, // delete (127)
    ];
}
