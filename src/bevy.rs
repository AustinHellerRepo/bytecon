use bevy::{input::{keyboard::NativeKeyCode, mouse::MouseScrollUnit}, math::Affine3, prelude::*};
use crate::ByteConverter;
use std::error::Error;

impl ByteConverter for KeyCode {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        match self {
            Self::Abort => 0u8.append_to_bytes(bytes)?,
            Self::Again => 1u8.append_to_bytes(bytes)?,
            Self::AltLeft => 2u8.append_to_bytes(bytes)?,
            Self::AltRight => 3u8.append_to_bytes(bytes)?,
            Self::ArrowDown => 4u8.append_to_bytes(bytes)?,
            Self::ArrowLeft => 5u8.append_to_bytes(bytes)?,
            Self::ArrowRight => 6u8.append_to_bytes(bytes)?,
            Self::ArrowUp => 7u8.append_to_bytes(bytes)?,
            Self::AudioVolumeDown => 8u8.append_to_bytes(bytes)?,
            Self::AudioVolumeMute => 9u8.append_to_bytes(bytes)?,
            Self::AudioVolumeUp => 10u8.append_to_bytes(bytes)?,
            Self::Backquote => 11u8.append_to_bytes(bytes)?,
            Self::Backslash => 12u8.append_to_bytes(bytes)?,
            Self::Backspace => 13u8.append_to_bytes(bytes)?,
            Self::BracketLeft => 14u8.append_to_bytes(bytes)?,
            Self::BracketRight => 15u8.append_to_bytes(bytes)?,
            Self::BrowserBack => 16u8.append_to_bytes(bytes)?,
            Self::BrowserFavorites => 17u8.append_to_bytes(bytes)?,
            Self::BrowserForward => 18u8.append_to_bytes(bytes)?,
            Self::BrowserHome => 19u8.append_to_bytes(bytes)?,
            Self::BrowserRefresh => 20u8.append_to_bytes(bytes)?,
            Self::BrowserSearch => 21u8.append_to_bytes(bytes)?,
            Self::BrowserStop => 22u8.append_to_bytes(bytes)?,
            Self::CapsLock => 23u8.append_to_bytes(bytes)?,
            Self::Comma => 24u8.append_to_bytes(bytes)?,
            Self::ContextMenu => 25u8.append_to_bytes(bytes)?,
            Self::ControlLeft => 26u8.append_to_bytes(bytes)?,
            Self::ControlRight => 27u8.append_to_bytes(bytes)?,
            Self::Convert => 28u8.append_to_bytes(bytes)?,
            Self::Copy => 29u8.append_to_bytes(bytes)?,
            Self::Cut => 30u8.append_to_bytes(bytes)?,
            Self::Delete => 31u8.append_to_bytes(bytes)?,
            Self::Digit0 => 32u8.append_to_bytes(bytes)?,
            Self::Digit1 => 33u8.append_to_bytes(bytes)?,
            Self::Digit2 => 34u8.append_to_bytes(bytes)?,
            Self::Digit3 => 35u8.append_to_bytes(bytes)?,
            Self::Digit4 => 36u8.append_to_bytes(bytes)?,
            Self::Digit5 => 37u8.append_to_bytes(bytes)?,
            Self::Digit6 => 38u8.append_to_bytes(bytes)?,
            Self::Digit7 => 39u8.append_to_bytes(bytes)?,
            Self::Digit8 => 40u8.append_to_bytes(bytes)?,
            Self::Digit9 => 41u8.append_to_bytes(bytes)?,
            Self::Eject => 42u8.append_to_bytes(bytes)?,
            Self::End => 43u8.append_to_bytes(bytes)?,
            Self::Enter => 44u8.append_to_bytes(bytes)?,
            Self::Equal => 45u8.append_to_bytes(bytes)?,
            Self::Escape => 46u8.append_to_bytes(bytes)?,
            Self::F1 => 47u8.append_to_bytes(bytes)?,
            Self::F10 => 48u8.append_to_bytes(bytes)?,
            Self::F11 => 49u8.append_to_bytes(bytes)?,
            Self::F12 => 50u8.append_to_bytes(bytes)?,
            Self::F13 => 51u8.append_to_bytes(bytes)?,
            Self::F14 => 52u8.append_to_bytes(bytes)?,
            Self::F15 => 53u8.append_to_bytes(bytes)?,
            Self::F16 => 54u8.append_to_bytes(bytes)?,
            Self::F17 => 55u8.append_to_bytes(bytes)?,
            Self::F18 => 56u8.append_to_bytes(bytes)?,
            Self::F19 => 57u8.append_to_bytes(bytes)?,
            Self::F2 => 58u8.append_to_bytes(bytes)?,
            Self::F20 => 59u8.append_to_bytes(bytes)?,
            Self::F21 => 60u8.append_to_bytes(bytes)?,
            Self::F22 => 61u8.append_to_bytes(bytes)?,
            Self::F23 => 62u8.append_to_bytes(bytes)?,
            Self::F24 => 63u8.append_to_bytes(bytes)?,
            Self::F25 => 64u8.append_to_bytes(bytes)?,
            Self::F26 => 65u8.append_to_bytes(bytes)?,
            Self::F27 => 66u8.append_to_bytes(bytes)?,
            Self::F28 => 67u8.append_to_bytes(bytes)?,
            Self::F29 => 68u8.append_to_bytes(bytes)?,
            Self::F3 => 69u8.append_to_bytes(bytes)?,
            Self::F30 => 70u8.append_to_bytes(bytes)?,
            Self::F31 => 71u8.append_to_bytes(bytes)?,
            Self::F32 => 72u8.append_to_bytes(bytes)?,
            Self::F33 => 73u8.append_to_bytes(bytes)?,
            Self::F34 => 74u8.append_to_bytes(bytes)?,
            Self::F35 => 75u8.append_to_bytes(bytes)?,
            Self::F4 => 76u8.append_to_bytes(bytes)?,
            Self::F5 => 77u8.append_to_bytes(bytes)?,
            Self::F6 => 78u8.append_to_bytes(bytes)?,
            Self::F7 => 79u8.append_to_bytes(bytes)?,
            Self::F8 => 80u8.append_to_bytes(bytes)?,
            Self::F9 => 81u8.append_to_bytes(bytes)?,
            Self::Find => 82u8.append_to_bytes(bytes)?,
            Self::Fn => 83u8.append_to_bytes(bytes)?,
            Self::FnLock => 84u8.append_to_bytes(bytes)?,
            Self::Help => 85u8.append_to_bytes(bytes)?,
            Self::Hiragana => 86u8.append_to_bytes(bytes)?,
            Self::Home => 87u8.append_to_bytes(bytes)?,
            Self::Hyper => 88u8.append_to_bytes(bytes)?,
            Self::Insert => 89u8.append_to_bytes(bytes)?,
            Self::IntlBackslash => 90u8.append_to_bytes(bytes)?,
            Self::IntlRo => 91u8.append_to_bytes(bytes)?,
            Self::IntlYen => 92u8.append_to_bytes(bytes)?,
            Self::KanaMode => 93u8.append_to_bytes(bytes)?,
            Self::Katakana => 94u8.append_to_bytes(bytes)?,
            Self::KeyA => 95u8.append_to_bytes(bytes)?,
            Self::KeyB => 96u8.append_to_bytes(bytes)?,
            Self::KeyC => 97u8.append_to_bytes(bytes)?,
            Self::KeyD => 98u8.append_to_bytes(bytes)?,
            Self::KeyE => 99u8.append_to_bytes(bytes)?,
            Self::KeyF => 100u8.append_to_bytes(bytes)?,
            Self::KeyG => 101u8.append_to_bytes(bytes)?,
            Self::KeyH => 102u8.append_to_bytes(bytes)?,
            Self::KeyI => 103u8.append_to_bytes(bytes)?,
            Self::KeyJ => 104u8.append_to_bytes(bytes)?,
            Self::KeyK => 105u8.append_to_bytes(bytes)?,
            Self::KeyL => 106u8.append_to_bytes(bytes)?,
            Self::KeyM => 107u8.append_to_bytes(bytes)?,
            Self::KeyN => 108u8.append_to_bytes(bytes)?,
            Self::KeyO => 109u8.append_to_bytes(bytes)?,
            Self::KeyP => 110u8.append_to_bytes(bytes)?,
            Self::KeyQ => 111u8.append_to_bytes(bytes)?,
            Self::KeyR => 112u8.append_to_bytes(bytes)?,
            Self::KeyS => 113u8.append_to_bytes(bytes)?,
            Self::KeyT => 114u8.append_to_bytes(bytes)?,
            Self::KeyU => 115u8.append_to_bytes(bytes)?,
            Self::KeyV => 116u8.append_to_bytes(bytes)?,
            Self::KeyW => 117u8.append_to_bytes(bytes)?,
            Self::KeyX => 118u8.append_to_bytes(bytes)?,
            Self::KeyY => 119u8.append_to_bytes(bytes)?,
            Self::KeyZ => 120u8.append_to_bytes(bytes)?,
            Self::Lang1 => 121u8.append_to_bytes(bytes)?,
            Self::Lang2 => 122u8.append_to_bytes(bytes)?,
            Self::Lang3 => 123u8.append_to_bytes(bytes)?,
            Self::Lang4 => 124u8.append_to_bytes(bytes)?,
            Self::Lang5 => 125u8.append_to_bytes(bytes)?,
            Self::LaunchApp1 => 126u8.append_to_bytes(bytes)?,
            Self::LaunchApp2 => 127u8.append_to_bytes(bytes)?,
            Self::LaunchMail => 128u8.append_to_bytes(bytes)?,
            Self::MediaPlayPause => 129u8.append_to_bytes(bytes)?,
            Self::MediaSelect => 130u8.append_to_bytes(bytes)?,
            Self::MediaStop => 131u8.append_to_bytes(bytes)?,
            Self::MediaTrackNext => 132u8.append_to_bytes(bytes)?,
            Self::MediaTrackPrevious => 133u8.append_to_bytes(bytes)?,
            Self::Meta => 134u8.append_to_bytes(bytes)?,
            Self::Minus => 135u8.append_to_bytes(bytes)?,
            Self::NonConvert => 136u8.append_to_bytes(bytes)?,
            Self::NumLock => 137u8.append_to_bytes(bytes)?,
            Self::Numpad0 => 138u8.append_to_bytes(bytes)?,
            Self::Numpad1 => 139u8.append_to_bytes(bytes)?,
            Self::Numpad2 => 140u8.append_to_bytes(bytes)?,
            Self::Numpad3 => 141u8.append_to_bytes(bytes)?,
            Self::Numpad4 => 142u8.append_to_bytes(bytes)?,
            Self::Numpad5 => 143u8.append_to_bytes(bytes)?,
            Self::Numpad6 => 144u8.append_to_bytes(bytes)?,
            Self::Numpad7 => 145u8.append_to_bytes(bytes)?,
            Self::Numpad8 => 146u8.append_to_bytes(bytes)?,
            Self::Numpad9 => 147u8.append_to_bytes(bytes)?,
            Self::NumpadAdd => 148u8.append_to_bytes(bytes)?,
            Self::NumpadBackspace => 149u8.append_to_bytes(bytes)?,
            Self::NumpadClear => 150u8.append_to_bytes(bytes)?,
            Self::NumpadClearEntry => 151u8.append_to_bytes(bytes)?,
            Self::NumpadComma => 152u8.append_to_bytes(bytes)?,
            Self::NumpadDecimal => 153u8.append_to_bytes(bytes)?,
            Self::NumpadDivide => 154u8.append_to_bytes(bytes)?,
            Self::NumpadEnter => 155u8.append_to_bytes(bytes)?,
            Self::NumpadEqual => 156u8.append_to_bytes(bytes)?,
            Self::NumpadHash => 157u8.append_to_bytes(bytes)?,
            Self::NumpadMemoryAdd => 158u8.append_to_bytes(bytes)?,
            Self::NumpadMemoryClear => 159u8.append_to_bytes(bytes)?,
            Self::NumpadMemoryRecall => 160u8.append_to_bytes(bytes)?,
            Self::NumpadMemoryStore => 161u8.append_to_bytes(bytes)?,
            Self::NumpadMemorySubtract => 162u8.append_to_bytes(bytes)?,
            Self::NumpadMultiply => 163u8.append_to_bytes(bytes)?,
            Self::NumpadParenLeft => 164u8.append_to_bytes(bytes)?,
            Self::NumpadParenRight => 165u8.append_to_bytes(bytes)?,
            Self::NumpadStar => 166u8.append_to_bytes(bytes)?,
            Self::NumpadSubtract => 167u8.append_to_bytes(bytes)?,
            Self::Open => 168u8.append_to_bytes(bytes)?,
            Self::PageDown => 169u8.append_to_bytes(bytes)?,
            Self::PageUp => 170u8.append_to_bytes(bytes)?,
            Self::Paste => 171u8.append_to_bytes(bytes)?,
            Self::Pause => 172u8.append_to_bytes(bytes)?,
            Self::Period => 173u8.append_to_bytes(bytes)?,
            Self::Power => 174u8.append_to_bytes(bytes)?,
            Self::PrintScreen => 175u8.append_to_bytes(bytes)?,
            Self::Props => 176u8.append_to_bytes(bytes)?,
            Self::Quote => 177u8.append_to_bytes(bytes)?,
            Self::Resume => 178u8.append_to_bytes(bytes)?,
            Self::ScrollLock => 179u8.append_to_bytes(bytes)?,
            Self::Select => 180u8.append_to_bytes(bytes)?,
            Self::Semicolon => 181u8.append_to_bytes(bytes)?,
            Self::ShiftLeft => 182u8.append_to_bytes(bytes)?,
            Self::ShiftRight => 183u8.append_to_bytes(bytes)?,
            Self::Slash => 184u8.append_to_bytes(bytes)?,
            Self::Sleep => 185u8.append_to_bytes(bytes)?,
            Self::Space => 186u8.append_to_bytes(bytes)?,
            Self::SuperLeft => 187u8.append_to_bytes(bytes)?,
            Self::SuperRight => 188u8.append_to_bytes(bytes)?,
            Self::Suspend => 189u8.append_to_bytes(bytes)?,
            Self::Tab => 190u8.append_to_bytes(bytes)?,
            Self::Turbo => 191u8.append_to_bytes(bytes)?,
            Self::Undo => 192u8.append_to_bytes(bytes)?,
            Self::Unidentified(native_key_code) => {
                193u8.append_to_bytes(bytes)?;
                native_key_code.append_to_bytes(bytes)?;
            },
            Self::WakeUp => 194u8.append_to_bytes(bytes)?,
        }
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        let enum_variant_byte = u8::extract_from_bytes(bytes, index)?;
        match enum_variant_byte {
            0u8 => Ok(Self::Abort),
            1u8 => Ok(Self::Again),
            2u8 => Ok(Self::AltLeft),
            3u8 => Ok(Self::AltRight),
            4u8 => Ok(Self::ArrowDown),
            5u8 => Ok(Self::ArrowLeft),
            6u8 => Ok(Self::ArrowRight),
            7u8 => Ok(Self::ArrowUp),
            8u8 => Ok(Self::AudioVolumeDown),
            9u8 => Ok(Self::AudioVolumeMute),
            10u8 => Ok(Self::AudioVolumeUp),
            11u8 => Ok(Self::Backquote),
            12u8 => Ok(Self::Backslash),
            13u8 => Ok(Self::Backspace),
            14u8 => Ok(Self::BracketLeft),
            15u8 => Ok(Self::BracketRight),
            16u8 => Ok(Self::BrowserBack),
            17u8 => Ok(Self::BrowserFavorites),
            18u8 => Ok(Self::BrowserForward),
            19u8 => Ok(Self::BrowserHome),
            20u8 => Ok(Self::BrowserRefresh),
            21u8 => Ok(Self::BrowserSearch),
            22u8 => Ok(Self::BrowserStop),
            23u8 => Ok(Self::CapsLock),
            24u8 => Ok(Self::Comma),
            25u8 => Ok(Self::ContextMenu),
            26u8 => Ok(Self::ControlLeft),
            27u8 => Ok(Self::ControlRight),
            28u8 => Ok(Self::Convert),
            29u8 => Ok(Self::Copy),
            30u8 => Ok(Self::Cut),
            31u8 => Ok(Self::Delete),
            32u8 => Ok(Self::Digit0),
            33u8 => Ok(Self::Digit1),
            34u8 => Ok(Self::Digit2),
            35u8 => Ok(Self::Digit3),
            36u8 => Ok(Self::Digit4),
            37u8 => Ok(Self::Digit5),
            38u8 => Ok(Self::Digit6),
            39u8 => Ok(Self::Digit7),
            40u8 => Ok(Self::Digit8),
            41u8 => Ok(Self::Digit9),
            42u8 => Ok(Self::Eject),
            43u8 => Ok(Self::End),
            44u8 => Ok(Self::Enter),
            45u8 => Ok(Self::Equal),
            46u8 => Ok(Self::Escape),
            47u8 => Ok(Self::F1),
            48u8 => Ok(Self::F10),
            49u8 => Ok(Self::F11),
            50u8 => Ok(Self::F12),
            51u8 => Ok(Self::F13),
            52u8 => Ok(Self::F14),
            53u8 => Ok(Self::F15),
            54u8 => Ok(Self::F16),
            55u8 => Ok(Self::F17),
            56u8 => Ok(Self::F18),
            57u8 => Ok(Self::F19),
            58u8 => Ok(Self::F2),
            59u8 => Ok(Self::F20),
            60u8 => Ok(Self::F21),
            61u8 => Ok(Self::F22),
            62u8 => Ok(Self::F23),
            63u8 => Ok(Self::F24),
            64u8 => Ok(Self::F25),
            65u8 => Ok(Self::F26),
            66u8 => Ok(Self::F27),
            67u8 => Ok(Self::F28),
            68u8 => Ok(Self::F29),
            69u8 => Ok(Self::F3),
            70u8 => Ok(Self::F30),
            71u8 => Ok(Self::F31),
            72u8 => Ok(Self::F32),
            73u8 => Ok(Self::F33),
            74u8 => Ok(Self::F34),
            75u8 => Ok(Self::F35),
            76u8 => Ok(Self::F4),
            77u8 => Ok(Self::F5),
            78u8 => Ok(Self::F6),
            79u8 => Ok(Self::F7),
            80u8 => Ok(Self::F8),
            81u8 => Ok(Self::F9),
            82u8 => Ok(Self::Find),
            83u8 => Ok(Self::Fn),
            84u8 => Ok(Self::FnLock),
            85u8 => Ok(Self::Help),
            86u8 => Ok(Self::Hiragana),
            87u8 => Ok(Self::Home),
            88u8 => Ok(Self::Hyper),
            89u8 => Ok(Self::Insert),
            90u8 => Ok(Self::IntlBackslash),
            91u8 => Ok(Self::IntlRo),
            92u8 => Ok(Self::IntlYen),
            93u8 => Ok(Self::KanaMode),
            94u8 => Ok(Self::Katakana),
            95u8 => Ok(Self::KeyA),
            96u8 => Ok(Self::KeyB),
            97u8 => Ok(Self::KeyC),
            98u8 => Ok(Self::KeyD),
            99u8 => Ok(Self::KeyE),
            100u8 => Ok(Self::KeyF),
            101u8 => Ok(Self::KeyG),
            102u8 => Ok(Self::KeyH),
            103u8 => Ok(Self::KeyI),
            104u8 => Ok(Self::KeyJ),
            105u8 => Ok(Self::KeyK),
            106u8 => Ok(Self::KeyL),
            107u8 => Ok(Self::KeyM),
            108u8 => Ok(Self::KeyN),
            109u8 => Ok(Self::KeyO),
            110u8 => Ok(Self::KeyP),
            111u8 => Ok(Self::KeyQ),
            112u8 => Ok(Self::KeyR),
            113u8 => Ok(Self::KeyS),
            114u8 => Ok(Self::KeyT),
            115u8 => Ok(Self::KeyU),
            116u8 => Ok(Self::KeyV),
            117u8 => Ok(Self::KeyW),
            118u8 => Ok(Self::KeyX),
            119u8 => Ok(Self::KeyY),
            120u8 => Ok(Self::KeyZ),
            121u8 => Ok(Self::Lang1),
            122u8 => Ok(Self::Lang2),
            123u8 => Ok(Self::Lang3),
            124u8 => Ok(Self::Lang4),
            125u8 => Ok(Self::Lang5),
            126u8 => Ok(Self::LaunchApp1),
            127u8 => Ok(Self::LaunchApp2),
            128u8 => Ok(Self::LaunchMail),
            129u8 => Ok(Self::MediaPlayPause),
            130u8 => Ok(Self::MediaSelect),
            131u8 => Ok(Self::MediaStop),
            132u8 => Ok(Self::MediaTrackNext),
            133u8 => Ok(Self::MediaTrackPrevious),
            134u8 => Ok(Self::Meta),
            135u8 => Ok(Self::Minus),
            136u8 => Ok(Self::NonConvert),
            137u8 => Ok(Self::NumLock),
            138u8 => Ok(Self::Numpad0),
            139u8 => Ok(Self::Numpad1),
            140u8 => Ok(Self::Numpad2),
            141u8 => Ok(Self::Numpad3),
            142u8 => Ok(Self::Numpad4),
            143u8 => Ok(Self::Numpad5),
            144u8 => Ok(Self::Numpad6),
            145u8 => Ok(Self::Numpad7),
            146u8 => Ok(Self::Numpad8),
            147u8 => Ok(Self::Numpad9),
            148u8 => Ok(Self::NumpadAdd),
            149u8 => Ok(Self::NumpadBackspace),
            150u8 => Ok(Self::NumpadClear),
            151u8 => Ok(Self::NumpadClearEntry),
            152u8 => Ok(Self::NumpadComma),
            153u8 => Ok(Self::NumpadDecimal),
            154u8 => Ok(Self::NumpadDivide),
            155u8 => Ok(Self::NumpadEnter),
            156u8 => Ok(Self::NumpadEqual),
            157u8 => Ok(Self::NumpadHash),
            158u8 => Ok(Self::NumpadMemoryAdd),
            159u8 => Ok(Self::NumpadMemoryClear),
            160u8 => Ok(Self::NumpadMemoryRecall),
            161u8 => Ok(Self::NumpadMemoryStore),
            162u8 => Ok(Self::NumpadMemorySubtract),
            163u8 => Ok(Self::NumpadMultiply),
            164u8 => Ok(Self::NumpadParenLeft),
            165u8 => Ok(Self::NumpadParenRight),
            166u8 => Ok(Self::NumpadStar),
            167u8 => Ok(Self::NumpadSubtract),
            168u8 => Ok(Self::Open),
            169u8 => Ok(Self::PageDown),
            170u8 => Ok(Self::PageUp),
            171u8 => Ok(Self::Paste),
            172u8 => Ok(Self::Pause),
            173u8 => Ok(Self::Period),
            174u8 => Ok(Self::Power),
            175u8 => Ok(Self::PrintScreen),
            176u8 => Ok(Self::Props),
            177u8 => Ok(Self::Quote),
            178u8 => Ok(Self::Resume),
            179u8 => Ok(Self::ScrollLock),
            180u8 => Ok(Self::Select),
            181u8 => Ok(Self::Semicolon),
            182u8 => Ok(Self::ShiftLeft),
            183u8 => Ok(Self::ShiftRight),
            184u8 => Ok(Self::Slash),
            185u8 => Ok(Self::Sleep),
            186u8 => Ok(Self::Space),
            187u8 => Ok(Self::SuperLeft),
            188u8 => Ok(Self::SuperRight),
            189u8 => Ok(Self::Suspend),
            190u8 => Ok(Self::Tab),
            191u8 => Ok(Self::Turbo),
            192u8 => Ok(Self::Undo),
            193u8 => Ok(Self::Unidentified(NativeKeyCode::extract_from_bytes(bytes, index)?)),
            194u8 => Ok(Self::WakeUp),
            _ => Err("Unexpected enum variant".into()),
        }
    }
}

impl ByteConverter for NativeKeyCode {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        match self {
            Self::Android(scan_code) => {
                0u8.append_to_bytes(bytes)?;
                scan_code.append_to_bytes(bytes)?;
            },
            Self::MacOS(scan_code) => {
                1u8.append_to_bytes(bytes)?;
                scan_code.append_to_bytes(bytes)?;
            },
            Self::Unidentified => 2u8.append_to_bytes(bytes)?,
            Self::Windows(scan_code) => {
                3u8.append_to_bytes(bytes)?;
                scan_code.append_to_bytes(bytes)?;
            },
            Self::Xkb(key_code) => {
                4u8.append_to_bytes(bytes)?;
                key_code.append_to_bytes(bytes)?;
            }
        }
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        let enum_variant_byte = u8::extract_from_bytes(bytes, index)?;
        match enum_variant_byte {
            0u8 => {
                Ok(Self::Android(u32::extract_from_bytes(bytes, index)?))
            },
            1u8 => {
                Ok(Self::MacOS(u16::extract_from_bytes(bytes, index)?))
            },
            2u8 => {
                Ok(Self::Unidentified)
            },
            3u8 => {
                Ok(Self::Windows(u16::extract_from_bytes(bytes, index)?))
            },
            4u8 => {
                Ok(Self::Xkb(u32::extract_from_bytes(bytes, index)?))
            },
            _ => {
                Err("Unexpected enum variant byte".into())
            },
        }
    }
}

impl ByteConverter for MouseButton {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        match self {
            Self::Back => 0u8.append_to_bytes(bytes)?,
            Self::Forward => 1u8.append_to_bytes(bytes)?,
            Self::Left => 2u8.append_to_bytes(bytes)?,
            Self::Middle => 3u8.append_to_bytes(bytes)?,
            Self::Other(mouse_button_id) => {
                4u8.append_to_bytes(bytes)?;
                mouse_button_id.append_to_bytes(bytes)?;
            },
            Self::Right => 5u8.append_to_bytes(bytes)?,
        }
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        let enum_variant_byte = u8::extract_from_bytes(bytes, index)?;
        match enum_variant_byte {
            0u8 => Ok(Self::Back),
            1u8 => Ok(Self::Forward),
            2u8 => Ok(Self::Left),
            3u8 => Ok(Self::Middle),
            4u8 => Ok(Self::Other(u16::extract_from_bytes(bytes, index)?)),
            5u8 => Ok(Self::Right),
            _ => Err("Unexpected enum variant byte".into()),
        }
    }
}

impl ByteConverter for MouseScrollUnit {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        match self {
            Self::Line => 0u8.append_to_bytes(bytes)?,
            Self::Pixel => 1u8.append_to_bytes(bytes)?,
        }
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        let enum_variant_byte = u8::extract_from_bytes(bytes, index)?;
        match enum_variant_byte {
            0u8 => Ok(Self::Line),
            1u8 => Ok(Self::Pixel),
            _ => Err("Unexpected enum variant byte".into()),
        }
    }
}

impl ByteConverter for Entity {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.to_bits().append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self::from_bits(u64::extract_from_bytes(bytes, index)?))
    }
}

impl ByteConverter for Transform {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.rotation.append_to_bytes(bytes)?;
        self.scale.append_to_bytes(bytes)?;
        self.translation.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        let mut instance = Self::default();
        instance.rotation = Quat::extract_from_bytes(bytes, index)?;
        instance.scale = Vec3::extract_from_bytes(bytes, index)?;
        instance.translation = Vec3::extract_from_bytes(bytes, index)?;
        Ok(instance)
    }
}

impl ByteConverter for Text {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        self.0.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self::new(String::extract_from_bytes(bytes, index)?))
    }
}

impl ByteConverter for Text2d {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        self.0.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self::new(String::extract_from_bytes(bytes, index)?))
    }
}

impl ByteConverter for Color {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        match self {
            Self::Srgba(value) => {
                0u8.append_to_bytes(bytes)?;
                value.red.append_to_bytes(bytes)?;
                value.green.append_to_bytes(bytes)?;
                value.blue.append_to_bytes(bytes)?;
                value.alpha.append_to_bytes(bytes)?;
            },
            Self::LinearRgba(value) => {
                1u8.append_to_bytes(bytes)?;
                value.red.append_to_bytes(bytes)?;
                value.green.append_to_bytes(bytes)?;
                value.blue.append_to_bytes(bytes)?;
                value.alpha.append_to_bytes(bytes)?;
            },
            Self::Hsla(value) => {
                2u8.append_to_bytes(bytes)?;
                value.hue.append_to_bytes(bytes)?;
                value.saturation.append_to_bytes(bytes)?;
                value.lightness.append_to_bytes(bytes)?;
                value.alpha.append_to_bytes(bytes)?;
            },
            Self::Hsva(value) => {
                3u8.append_to_bytes(bytes)?;
                value.hue.append_to_bytes(bytes)?;
                value.saturation.append_to_bytes(bytes)?;
                value.value.append_to_bytes(bytes)?;
                value.alpha.append_to_bytes(bytes)?;
            },
            Self::Hwba(value) => {
                4u8.append_to_bytes(bytes)?;
                value.hue.append_to_bytes(bytes)?;
                value.whiteness.append_to_bytes(bytes)?;
                value.blackness.append_to_bytes(bytes)?;
                value.alpha.append_to_bytes(bytes)?;
            },
            Self::Laba(value) => {
                5u8.append_to_bytes(bytes)?;
                value.lightness.append_to_bytes(bytes)?;
                value.a.append_to_bytes(bytes)?;
                value.b.append_to_bytes(bytes)?;
                value.alpha.append_to_bytes(bytes)?;
            },
            Self::Lcha(value) => {
                6u8.append_to_bytes(bytes)?;
                value.lightness.append_to_bytes(bytes)?;
                value.chroma.append_to_bytes(bytes)?;
                value.hue.append_to_bytes(bytes)?;
                value.alpha.append_to_bytes(bytes)?;
            },
            Self::Oklaba(value) => {
                7u8.append_to_bytes(bytes)?;
                value.lightness.append_to_bytes(bytes)?;
                value.a.append_to_bytes(bytes)?;
                value.b.append_to_bytes(bytes)?;
                value.alpha.append_to_bytes(bytes)?;
            },
            Self::Oklcha(value) => {
                8u8.append_to_bytes(bytes)?;
                value.lightness.append_to_bytes(bytes)?;
                value.chroma.append_to_bytes(bytes)?;
                value.hue.append_to_bytes(bytes)?;
                value.alpha.append_to_bytes(bytes)?;
            },
            Self::Xyza(value) => {
                9u8.append_to_bytes(bytes)?;
                value.x.append_to_bytes(bytes)?;
                value.y.append_to_bytes(bytes)?;
                value.z.append_to_bytes(bytes)?;
                value.alpha.append_to_bytes(bytes)?;
            },
        }
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
        let enum_variant_byte = u8::extract_from_bytes(bytes, index)?;
        match enum_variant_byte {
            0u8 => Ok(Self::Srgba(Srgba {
                red: f32::extract_from_bytes(bytes, index)?,
                green: f32::extract_from_bytes(bytes, index)?,
                blue: f32::extract_from_bytes(bytes, index)?,
                alpha: f32::extract_from_bytes(bytes, index)?,
            })),
            1u8 => Ok(Self::LinearRgba(LinearRgba {
                red: f32::extract_from_bytes(bytes, index)?,
                green: f32::extract_from_bytes(bytes, index)?,
                blue: f32::extract_from_bytes(bytes, index)?,
                alpha: f32::extract_from_bytes(bytes, index)?,
            })),
            2u8 => Ok(Self::Hsla(Hsla {
                hue: f32::extract_from_bytes(bytes, index)?,
                saturation: f32::extract_from_bytes(bytes, index)?,
                lightness: f32::extract_from_bytes(bytes, index)?,
                alpha: f32::extract_from_bytes(bytes, index)?,
            })),
            3u8 => Ok(Self::Hsva(Hsva {
                hue: f32::extract_from_bytes(bytes, index)?,
                saturation: f32::extract_from_bytes(bytes, index)?,
                value: f32::extract_from_bytes(bytes, index)?,
                alpha: f32::extract_from_bytes(bytes, index)?,
            })),
            4u8 => Ok(Self::Hwba(Hwba {
                hue: f32::extract_from_bytes(bytes, index)?,
                whiteness: f32::extract_from_bytes(bytes, index)?,
                blackness: f32::extract_from_bytes(bytes, index)?,
                alpha: f32::extract_from_bytes(bytes, index)?,
            })),
            5u8 => Ok(Self::Laba(Laba {
                lightness: f32::extract_from_bytes(bytes, index)?,
                a: f32::extract_from_bytes(bytes, index)?,
                b: f32::extract_from_bytes(bytes, index)?,
                alpha: f32::extract_from_bytes(bytes, index)?,
            })),
            6u8 => Ok(Self::Lcha(Lcha {
                lightness: f32::extract_from_bytes(bytes, index)?,
                chroma: f32::extract_from_bytes(bytes, index)?,
                hue: f32::extract_from_bytes(bytes, index)?,
                alpha: f32::extract_from_bytes(bytes, index)?,
            })),
            7u8 => Ok(Self::Oklaba(Oklaba {
                lightness: f32::extract_from_bytes(bytes, index)?,
                a: f32::extract_from_bytes(bytes, index)?,
                b: f32::extract_from_bytes(bytes, index)?,
                alpha: f32::extract_from_bytes(bytes, index)?,
            })),
            8u8 => Ok(Self::Oklcha(Oklcha {
                lightness: f32::extract_from_bytes(bytes, index)?,
                chroma: f32::extract_from_bytes(bytes, index)?,
                hue: f32::extract_from_bytes(bytes, index)?,
                alpha: f32::extract_from_bytes(bytes, index)?,
            })),
            9u8 => Ok(Self::Xyza(Xyza {
                x: f32::extract_from_bytes(bytes, index)?,
                y: f32::extract_from_bytes(bytes, index)?,
                z: f32::extract_from_bytes(bytes, index)?,
                alpha: f32::extract_from_bytes(bytes, index)?,
            })),
            _ => Err("Unexpected enum variant byte.".into()),
        }
    }
}

impl ByteConverter for TextColor {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        self.0.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self(Color::extract_from_bytes(bytes, index)?))
    }
}

impl ByteConverter for Affine3 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.matrix3.append_to_bytes(bytes)?;
        self.translation.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            matrix3: Mat3::extract_from_bytes(bytes, index)?,
            translation: Vec3::extract_from_bytes(bytes, index)?,
        })
    }
}