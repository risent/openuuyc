//! Windows virtual keys translated to macOS key codes.
//!
//! The UU protocol carries a bare integer for every keystroke, and each host
//! reads it in its own key space. A Windows host reads the Windows virtual key
//! this client already produces; a macOS host instead feeds the number straight
//! to `CGEventCreateKeyboardEvent`, so it must be a macOS key code.
//!
//! The pairs below come from the macOS host binary's own `NEMUKeyCodes` table,
//! which stores `(text, scanCode, virtualKey, keyCode)` per key. The host only
//! ever reads the `keyCode` column, so that column is the wire format a macOS
//! peer expects. Sending the virtual key instead lands on unrelated keys --
//! `A` (0x41) becomes the numeric-keypad period (65) -- which is the "right
//! finger, wrong character" symptom this table exists to fix.
//!
//! Keys the table does not name are dropped rather than guessed at: a wrong
//! character is worse than none.

/// Translates a Windows virtual key to the macOS key code a macOS host wants.
///
/// Returns `None` for keys the host table does not cover, including the
/// browser/media/IME keys.
pub(crate) const fn from_virtual_key(key: u16) -> Option<u16> {
    Some(match key {
        // Letters.
        65 => 0,   // A
        66 => 11,  // B
        67 => 8,   // C
        68 => 2,   // D
        69 => 14,  // E
        70 => 3,   // F
        71 => 5,   // G
        72 => 4,   // H
        73 => 34,  // I
        74 => 38,  // J
        75 => 40,  // K
        76 => 37,  // L
        77 => 46,  // M
        78 => 45,  // N
        79 => 31,  // O
        80 => 35,  // P
        81 => 12,  // Q
        82 => 15,  // R
        83 => 1,   // S
        84 => 17,  // T
        85 => 32,  // U
        86 => 9,   // V
        87 => 13,  // W
        88 => 7,   // X
        89 => 16,  // Y
        90 => 6,   // Z
        // Digits.
        48 => 29, // 0
        49 => 18, // 1
        50 => 19, // 2
        51 => 20, // 3
        52 => 21, // 4
        53 => 23, // 5
        54 => 22, // 6
        55 => 26, // 7
        56 => 28, // 8
        57 => 25, // 9
        // Punctuation, keyed by the US position the virtual key names.
        192 => 50, // `
        189 => 27, // -
        187 => 24, // =
        219 => 33, // [
        221 => 30, // ]
        220 => 42, // \
        186 => 41, // ;
        222 => 39, // '
        188 => 43, // ,
        190 => 47, // .
        191 => 44, // /
        // Editing and whitespace.
        27 => 53, // Escape
        8 => 51,  // Backspace
        9 => 48,  // Tab
        13 => 36, // Enter
        32 => 49, // Space
        46 => 117, // Delete (forward)
        // Modifiers: the host table lists left before right for each pair.
        16 => 56,  // Shift
        160 => 56, // Left Shift
        161 => 60, // Right Shift
        17 => 59,  // Control
        162 => 59, // Left Control
        163 => 62, // Right Control
        18 => 58,  // Alt / Option
        164 => 58, // Left Alt
        165 => 61, // Right Alt
        91 => 55,  // Super / Command
        92 => 54,  // Right Command
        // Locks and navigation.
        20 => 57,   // Caps Lock
        144 => 71,  // Num Lock
        45 => 114,  // Insert
        36 => 115,  // Home
        35 => 119,  // End
        33 => 116,  // Page Up
        34 => 121,  // Page Down
        37 => 123,  // Arrow Left
        39 => 124,  // Arrow Right
        40 => 125,  // Arrow Down
        38 => 126,  // Arrow Up
        93 => 110,  // Context Menu
        // Numeric keypad.
        96 => 82,   // Numpad 0
        97 => 83,   // Numpad 1
        98 => 84,   // Numpad 2
        99 => 85,   // Numpad 3
        100 => 86,  // Numpad 4
        101 => 87,  // Numpad 5
        102 => 88,  // Numpad 6
        103 => 89,  // Numpad 7
        104 => 91,  // Numpad 8
        105 => 92,  // Numpad 9
        106 => 67,  // Numpad *
        107 => 69,  // Numpad +
        109 => 78,  // Numpad -
        110 => 65,  // Numpad .
        111 => 75,  // Numpad /
        // Function keys.
        112 => 122, // F1
        113 => 120, // F2
        114 => 99,  // F3
        115 => 118, // F4
        116 => 96,  // F5
        117 => 97,  // F6
        118 => 98,  // F7
        119 => 100, // F8
        120 => 101, // F9
        121 => 109, // F10
        122 => 103, // F11
        123 => 111, // F12
        124 => 105, // F13
        125 => 107, // F14
        126 => 113, // F15
        127 => 106, // F16
        128 => 64,  // F17
        129 => 79,  // F18
        130 => 80,  // F19
        131 => 90,  // F20
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::from_virtual_key;

    #[test]
    fn letters_and_digits_land_on_their_macos_codes() {
        assert_eq!(from_virtual_key(65), Some(0)); // A
        assert_eq!(from_virtual_key(83), Some(1)); // S
        assert_eq!(from_virtual_key(90), Some(6)); // Z
        assert_eq!(from_virtual_key(48), Some(29)); // 0
        assert_eq!(from_virtual_key(57), Some(25)); // 9
    }

    #[test]
    fn modifiers_keep_their_side() {
        assert_eq!(from_virtual_key(160), Some(56)); // Left Shift
        assert_eq!(from_virtual_key(161), Some(60)); // Right Shift
        assert_eq!(from_virtual_key(162), Some(59)); // Left Control
        assert_eq!(from_virtual_key(164), Some(58)); // Left Alt
        assert_eq!(from_virtual_key(91), Some(55)); // Command
    }

    #[test]
    fn unmapped_keys_are_rejected() {
        // Browser and media keys have no macOS key code in the host table.
        assert_eq!(from_virtual_key(166), None);
        assert_eq!(from_virtual_key(0), None);
    }
}
