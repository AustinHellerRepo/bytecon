#[cfg(test)]
mod byte_converter_tests {
    use std::path::PathBuf;

    use bevy::{input::keyboard::NativeKeyCode, prelude::{KeyCode, MouseButton}};
    use bytecon::ByteConverter;
    use rand::SeedableRng;
    use rand_chacha::{ChaCha20Rng, ChaCha8Rng};


    #[test]
    fn test_e8v5_string() {
        let string = String::from("test value");
        let cloned_string = string.clone_via_bytes().unwrap();
        assert_eq!(string, cloned_string);
    }

    #[test]
    fn test_t3b1_pathbuf() {
        let obj = PathBuf::from("/home/path/here");
        let cloned_obj = obj.clone_via_bytes().unwrap();
        assert_eq!(obj, cloned_obj);
    }
    
    #[test]
    fn test_p2b7_array() {
        let obj = [1, 2, 3];
        let cloned_obj = obj.clone_via_bytes().unwrap();
        assert_eq!(obj, cloned_obj);
    }

    #[test]
    fn test_t4l8_rand_chacha_8_entropy() {
        let obj = ChaCha8Rng::from_entropy();
        let cloned_obj = obj.clone_via_bytes().unwrap();
        assert_eq!(obj, cloned_obj);
    }

    #[test]
    fn test_e5p1_rand_chacha_8_seeded() {
        let obj = ChaCha8Rng::from_seed([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32]);
        let cloned_obj = obj.clone_via_bytes().unwrap();
        assert_eq!(obj, cloned_obj);
    }

    #[test]
    fn test_u4x2_rand_chacha_20_entropy() {
        let obj = ChaCha20Rng::from_entropy();
        let cloned_obj = obj.clone_via_bytes().unwrap();
        assert_eq!(obj, cloned_obj);
    }

    #[test]
    fn test_k9q3_rand_chacha_20_seeded() {
        let obj = ChaCha20Rng::from_seed([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32]);
        let cloned_obj = obj.clone_via_bytes().unwrap();
        assert_eq!(obj, cloned_obj);
    }
    
    #[test]
    fn test_l2c6_bevy_keys() {
        let key_codes = vec![
            KeyCode::Abort,
            KeyCode::Again,
            KeyCode::AltLeft,
            KeyCode::AltRight,
            KeyCode::ArrowDown,
            KeyCode::ArrowLeft,
            KeyCode::ArrowRight,
            KeyCode::ArrowUp,
            KeyCode::AudioVolumeDown,
            KeyCode::AudioVolumeMute,
            KeyCode::AudioVolumeUp,
            KeyCode::Backquote,
            KeyCode::Backslash,
            KeyCode::Backspace,
            KeyCode::BracketLeft,
            KeyCode::BracketRight,
            KeyCode::BrowserBack,
            KeyCode::BrowserFavorites,
            KeyCode::BrowserForward,
            KeyCode::BrowserHome,
            KeyCode::BrowserRefresh,
            KeyCode::BrowserSearch,
            KeyCode::BrowserStop,
            KeyCode::CapsLock,
            KeyCode::Comma,
            KeyCode::ContextMenu,
            KeyCode::ControlLeft,
            KeyCode::ControlRight,
            KeyCode::Convert,
            KeyCode::Copy,
            KeyCode::Cut,
            KeyCode::Delete,
            KeyCode::Digit0,
            KeyCode::Digit1,
            KeyCode::Digit2,
            KeyCode::Digit3,
            KeyCode::Digit4,
            KeyCode::Digit5,
            KeyCode::Digit6,
            KeyCode::Digit7,
            KeyCode::Digit8,
            KeyCode::Digit9,
            KeyCode::Eject,
            KeyCode::End,
            KeyCode::Enter,
            KeyCode::Equal,
            KeyCode::Escape,
            KeyCode::F1,
            KeyCode::F10,
            KeyCode::F11,
            KeyCode::F12,
            KeyCode::F13,
            KeyCode::F14,
            KeyCode::F15,
            KeyCode::F16,
            KeyCode::F17,
            KeyCode::F18,
            KeyCode::F19,
            KeyCode::F2,
            KeyCode::F20,
            KeyCode::F21,
            KeyCode::F22,
            KeyCode::F23,
            KeyCode::F24,
            KeyCode::F25,
            KeyCode::F26,
            KeyCode::F27,
            KeyCode::F28,
            KeyCode::F29,
            KeyCode::F3,
            KeyCode::F30,
            KeyCode::F31,
            KeyCode::F32,
            KeyCode::F33,
            KeyCode::F34,
            KeyCode::F35,
            KeyCode::F4,
            KeyCode::F5,
            KeyCode::F6,
            KeyCode::F7,
            KeyCode::F8,
            KeyCode::F9,
            KeyCode::Find,
            KeyCode::Fn,
            KeyCode::FnLock,
            KeyCode::Help,
            KeyCode::Hiragana,
            KeyCode::Home,
            KeyCode::Hyper,
            KeyCode::Insert,
            KeyCode::IntlBackslash,
            KeyCode::IntlRo,
            KeyCode::IntlYen,
            KeyCode::KanaMode,
            KeyCode::Katakana,
            KeyCode::KeyA,
            KeyCode::KeyB,
            KeyCode::KeyC,
            KeyCode::KeyD,
            KeyCode::KeyE,
            KeyCode::KeyF,
            KeyCode::KeyG,
            KeyCode::KeyH,
            KeyCode::KeyI,
            KeyCode::KeyJ,
            KeyCode::KeyK,
            KeyCode::KeyL,
            KeyCode::KeyM,
            KeyCode::KeyN,
            KeyCode::KeyO,
            KeyCode::KeyP,
            KeyCode::KeyQ,
            KeyCode::KeyR,
            KeyCode::KeyS,
            KeyCode::KeyT,
            KeyCode::KeyU,
            KeyCode::KeyV,
            KeyCode::KeyW,
            KeyCode::KeyX,
            KeyCode::KeyY,
            KeyCode::KeyZ,
            KeyCode::Lang1,
            KeyCode::Lang2,
            KeyCode::Lang3,
            KeyCode::Lang4,
            KeyCode::Lang5,
            KeyCode::LaunchApp1,
            KeyCode::LaunchApp2,
            KeyCode::LaunchMail,
            KeyCode::MediaPlayPause,
            KeyCode::MediaSelect,
            KeyCode::MediaStop,
            KeyCode::MediaTrackNext,
            KeyCode::MediaTrackPrevious,
            KeyCode::Meta,
            KeyCode::Minus,
            KeyCode::NonConvert,
            KeyCode::NumLock,
            KeyCode::Numpad0,
            KeyCode::Numpad1,
            KeyCode::Numpad2,
            KeyCode::Numpad3,
            KeyCode::Numpad4,
            KeyCode::Numpad5,
            KeyCode::Numpad6,
            KeyCode::Numpad7,
            KeyCode::Numpad8,
            KeyCode::Numpad9,
            KeyCode::NumpadAdd,
            KeyCode::NumpadBackspace,
            KeyCode::NumpadClear,
            KeyCode::NumpadClearEntry,
            KeyCode::NumpadComma,
            KeyCode::NumpadDecimal,
            KeyCode::NumpadDivide,
            KeyCode::NumpadEnter,
            KeyCode::NumpadEqual,
            KeyCode::NumpadHash,
            KeyCode::NumpadMemoryAdd,
            KeyCode::NumpadMemoryClear,
            KeyCode::NumpadMemoryRecall,
            KeyCode::NumpadMemoryStore,
            KeyCode::NumpadMemorySubtract,
            KeyCode::NumpadMultiply,
            KeyCode::NumpadParenLeft,
            KeyCode::NumpadParenRight,
            KeyCode::NumpadStar,
            KeyCode::NumpadSubtract,
            KeyCode::Open,
            KeyCode::PageDown,
            KeyCode::PageUp,
            KeyCode::Paste,
            KeyCode::Pause,
            KeyCode::Period,
            KeyCode::Power,
            KeyCode::PrintScreen,
            KeyCode::Props,
            KeyCode::Quote,
            KeyCode::Resume,
            KeyCode::ScrollLock,
            KeyCode::Select,
            KeyCode::Semicolon,
            KeyCode::ShiftLeft,
            KeyCode::ShiftRight,
            KeyCode::Slash,
            KeyCode::Sleep,
            KeyCode::Space,
            KeyCode::SuperLeft,
            KeyCode::SuperRight,
            KeyCode::Suspend,
            KeyCode::Tab,
            KeyCode::Turbo,
            KeyCode::Undo,
            KeyCode::Unidentified(NativeKeyCode::Android(u32::MIN)),
            KeyCode::Unidentified(NativeKeyCode::Android(123)),
            KeyCode::Unidentified(NativeKeyCode::Android(u32::MAX)),
            KeyCode::Unidentified(NativeKeyCode::MacOS(u16::MIN)),
            KeyCode::Unidentified(NativeKeyCode::MacOS(234)),
            KeyCode::Unidentified(NativeKeyCode::MacOS(u16::MAX)),
            KeyCode::Unidentified(NativeKeyCode::Unidentified),
            KeyCode::Unidentified(NativeKeyCode::Windows(u16::MIN)),
            KeyCode::Unidentified(NativeKeyCode::Windows(345)),
            KeyCode::Unidentified(NativeKeyCode::Windows(u16::MAX)),
            KeyCode::Unidentified(NativeKeyCode::Xkb(u32::MIN)),
            KeyCode::Unidentified(NativeKeyCode::Xkb(456)),
            KeyCode::Unidentified(NativeKeyCode::Xkb(u32::MAX)),
            KeyCode::WakeUp,
        ];

        for key_code in key_codes {
            assert_eq!(key_code, key_code.clone_via_bytes().unwrap());
        }
    }

    #[test]
    fn test_w8n4_bevy_mouse() {
        let mouse_buttons = vec![
            MouseButton::Back,
            MouseButton::Forward,
            MouseButton::Left,
            MouseButton::Middle,
            MouseButton::Other(u16::MIN),
            MouseButton::Other(123),
            MouseButton::Other(u16::MAX),
            MouseButton::Right,
        ];


        for mouse_button in mouse_buttons {
            assert_eq!(mouse_button, mouse_button.clone_via_bytes().unwrap());
        }
    }
}