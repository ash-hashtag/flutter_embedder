use winit::keyboard::KeyCode;

pub fn translate_physical_key(scancode: KeyCode) -> Option<u64> {
    Some(match scancode {
        KeyCode::Hyper => 0x00000010,
        KeyCode::SuperLeft | KeyCode::SuperRight => 0x00000011,
        KeyCode::Fn => 0x00000012,
        KeyCode::FnLock => 0x00000013,
        KeyCode::Suspend => 0x00000014,
        KeyCode::Resume => 0x00000015,
        KeyCode::Turbo => 0x00000016,
        // KeyCode::Lock => 0x00000017,
        // KeyCode::MicrophoneVolumeMute => 0x00000018,
        KeyCode::Sleep => 0x00010082,
        KeyCode::WakeUp => 0x00010083,
        // KeyCode::DisplayToggleIntExt => 0x000100b5,
        // KeyCode::GameButton1 => 0x0005ff01,
        // KeyCode::GameButton2 => 0x0005ff02,
        // KeyCode::GameButton3 => 0x0005ff03,
        // KeyCode::GameButton4 => 0x0005ff04,
        // KeyCode::GameButton5 => 0x0005ff05,
        // KeyCode::GameButton6 => 0x0005ff06,
        // KeyCode::GameButton7 => 0x0005ff07,
        // KeyCode::GameButton8 => 0x0005ff08,
        // KeyCode::GameButton9 => 0x0005ff09,
        // KeyCode::GameButton10 => 0x0005ff0a,
        // KeyCode::GameButton11 => 0x0005ff0b,
        // KeyCode::GameButton12 => 0x0005ff0c,
        // KeyCode::GameButton13 => 0x0005ff0d,
        // KeyCode::GameButton14 => 0x0005ff0e,
        // KeyCode::GameButton15 => 0x0005ff0f,
        // KeyCode::GameButton16 => 0x0005ff10,
        // KeyCode::GameButtonA => 0x0005ff11,
        // KeyCode::GameButtonB => 0x0005ff12,
        // KeyCode::GameButtonC => 0x0005ff13,
        // KeyCode::GameButtonLeft1 => 0x0005ff14,
        // KeyCode::GameButtonLeft2 => 0x0005ff15,
        // KeyCode::GameButtonMode => 0x0005ff16,
        // KeyCode::GameButtonRight1 => 0x0005ff17,
        // KeyCode::GameButtonRight2 => 0x0005ff18,
        // KeyCode::GameButtonSelect => 0x0005ff19,
        // KeyCode::GameButtonStart => 0x0005ff1a,
        // KeyCode::GameButtonThumbLeft => 0x0005ff1b,
        // KeyCode::GameButtonThumbRight => 0x0005ff1c,
        // KeyCode::GameButtonX => 0x0005ff1d,
        // KeyCode::GameButtonY => 0x0005ff1e,
        // KeyCode::GameButtonZ => 0x0005ff1f,
        // KeyCode::UsbReserved => 0x00070000,
        // KeyCode::UsbErrorRollOver => 0x00070001,
        // KeyCode::UsbPostFail => 0x00070002,
        // KeyCode::UsbErrorUndefined => 0x00070003,
        KeyCode::KeyA => 0x00070004,
        KeyCode::KeyB => 0x00070005,
        KeyCode::KeyC => 0x00070006,
        KeyCode::KeyD => 0x00070007,
        KeyCode::KeyE => 0x00070008,
        KeyCode::KeyF => 0x00070009,
        KeyCode::KeyG => 0x0007000a,
        KeyCode::KeyH => 0x0007000b,
        KeyCode::KeyI => 0x0007000c,
        KeyCode::KeyJ => 0x0007000d,
        KeyCode::KeyK => 0x0007000e,
        KeyCode::KeyL => 0x0007000f,
        KeyCode::KeyM => 0x00070010,
        KeyCode::KeyN => 0x00070011,
        KeyCode::KeyO => 0x00070012,
        KeyCode::KeyP => 0x00070013,
        KeyCode::KeyQ => 0x00070014,
        KeyCode::KeyR => 0x00070015,
        KeyCode::KeyS => 0x00070016,
        KeyCode::KeyT => 0x00070017,
        KeyCode::KeyU => 0x00070018,
        KeyCode::KeyV => 0x00070019,
        KeyCode::KeyW => 0x0007001a,
        KeyCode::KeyX => 0x0007001b,
        KeyCode::KeyY => 0x0007001c,
        KeyCode::KeyZ => 0x0007001d,
        KeyCode::Digit1 => 0x0007001e,
        KeyCode::Digit2 => 0x0007001f,
        KeyCode::Digit3 => 0x00070020,
        KeyCode::Digit4 => 0x00070021,
        KeyCode::Digit5 => 0x00070022,
        KeyCode::Digit6 => 0x00070023,
        KeyCode::Digit7 => 0x00070024,
        KeyCode::Digit8 => 0x00070025,
        KeyCode::Digit9 => 0x00070026,
        KeyCode::Digit0 => 0x00070027,
        KeyCode::Enter => 0x00070028,
        KeyCode::Escape => 0x00070029,
        KeyCode::Backspace => 0x0007002a,
        KeyCode::Tab => 0x0007002b,
        KeyCode::Space => 0x0007002c,
        KeyCode::Minus => 0x0007002d,
        KeyCode::Equal => 0x0007002e,
        KeyCode::BracketLeft => 0x0007002f,
        KeyCode::BracketRight => 0x00070030,
        KeyCode::Backslash => 0x00070031,
        KeyCode::Semicolon => 0x00070033,
        KeyCode::Quote => 0x00070034,
        KeyCode::Backquote => 0x00070035,
        KeyCode::Comma => 0x00070036,
        KeyCode::Period => 0x00070037,
        KeyCode::Slash => 0x00070038,
        KeyCode::CapsLock => 0x00070039,
        KeyCode::F1 => 0x0007003a,
        KeyCode::F2 => 0x0007003b,
        KeyCode::F3 => 0x0007003c,
        KeyCode::F4 => 0x0007003d,
        KeyCode::F5 => 0x0007003e,
        KeyCode::F6 => 0x0007003f,
        KeyCode::F7 => 0x00070040,
        KeyCode::F8 => 0x00070041,
        KeyCode::F9 => 0x00070042,
        KeyCode::F10 => 0x00070043,
        KeyCode::F11 => 0x00070044,
        KeyCode::F12 => 0x00070045,
        KeyCode::PrintScreen => 0x00070046,
        KeyCode::ScrollLock => 0x00070047,
        KeyCode::Pause => 0x00070048,
        KeyCode::Insert => 0x00070049,
        KeyCode::Home => 0x0007004a,
        KeyCode::PageUp => 0x0007004b,
        KeyCode::Delete => 0x0007004c,
        KeyCode::End => 0x0007004d,
        KeyCode::PageDown => 0x0007004e,
        KeyCode::ArrowRight => 0x0007004f,
        KeyCode::ArrowLeft => 0x00070050,
        KeyCode::ArrowDown => 0x00070051,
        KeyCode::ArrowUp => 0x00070052,
        KeyCode::NumLock => 0x00070053,
        KeyCode::NumpadDivide => 0x00070054,
        KeyCode::NumpadMultiply => 0x00070055,
        KeyCode::NumpadSubtract => 0x00070056,
        KeyCode::NumpadAdd => 0x00070057,
        KeyCode::NumpadEnter => 0x00070058,
        KeyCode::Numpad1 => 0x00070059,
        KeyCode::Numpad2 => 0x0007005a,
        KeyCode::Numpad3 => 0x0007005b,
        KeyCode::Numpad4 => 0x0007005c,
        KeyCode::Numpad5 => 0x0007005d,
        KeyCode::Numpad6 => 0x0007005e,
        KeyCode::Numpad7 => 0x0007005f,
        KeyCode::Numpad8 => 0x00070060,
        KeyCode::Numpad9 => 0x00070061,
        KeyCode::Numpad0 => 0x00070062,
        KeyCode::NumpadDecimal => 0x00070063,
        KeyCode::IntlBackslash => 0x00070064,
        KeyCode::ContextMenu => 0x00070065,
        KeyCode::Power => 0x00070066,
        KeyCode::NumpadEqual => 0x00070067,
        KeyCode::F13 => 0x00070068,
        KeyCode::F14 => 0x00070069,
        KeyCode::F15 => 0x0007006a,
        KeyCode::F16 => 0x0007006b,
        KeyCode::F17 => 0x0007006c,
        KeyCode::F18 => 0x0007006d,
        KeyCode::F19 => 0x0007006e,
        KeyCode::F20 => 0x0007006f,
        KeyCode::F21 => 0x00070070,
        KeyCode::F22 => 0x00070071,
        KeyCode::F23 => 0x00070072,
        KeyCode::F24 => 0x00070073,
        KeyCode::Open => 0x00070074,
        KeyCode::Help => 0x00070075,
        KeyCode::Select => 0x00070077,
        KeyCode::Again => 0x00070079,
        KeyCode::Undo => 0x0007007a,
        KeyCode::Cut => 0x0007007b,
        KeyCode::Copy => 0x0007007c,
        KeyCode::Paste => 0x0007007d,
        KeyCode::Find => 0x0007007e,
        KeyCode::AudioVolumeMute => 0x0007007f,
        KeyCode::AudioVolumeUp => 0x00070080,
        KeyCode::AudioVolumeDown => 0x00070081,
        KeyCode::NumpadComma => 0x00070085,
        KeyCode::IntlRo => 0x00070087,
        KeyCode::KanaMode => 0x00070088,
        KeyCode::IntlYen => 0x00070089,
        KeyCode::Convert => 0x0007008a,
        KeyCode::NonConvert => 0x0007008b,
        KeyCode::Lang1 => 0x00070090,
        KeyCode::Lang2 => 0x00070091,
        KeyCode::Lang3 => 0x00070092,
        KeyCode::Lang4 => 0x00070093,
        KeyCode::Lang5 => 0x00070094,
        KeyCode::Abort => 0x0007009b,
        KeyCode::Props => 0x000700a3,
        KeyCode::NumpadParenLeft => 0x000700b6,
        KeyCode::NumpadParenRight => 0x000700b7,
        KeyCode::NumpadBackspace => 0x000700bb,
        KeyCode::NumpadMemoryStore => 0x000700d0,
        KeyCode::NumpadMemoryRecall => 0x000700d1,
        KeyCode::NumpadMemoryClear => 0x000700d2,
        KeyCode::NumpadMemoryAdd => 0x000700d3,
        KeyCode::NumpadMemorySubtract => 0x000700d4,
        // KeyCode::NumpadSignChange => 0x000700d7,
        KeyCode::NumpadClear => 0x000700d8,
        KeyCode::NumpadClearEntry => 0x000700d9,
        KeyCode::ControlLeft => 0x000700e0,
        KeyCode::ShiftLeft => 0x000700e1,
        KeyCode::AltLeft => 0x000700e2,
        KeyCode::Meta => 0x000700e3,
        KeyCode::ControlRight => 0x000700e4,
        KeyCode::ShiftRight => 0x000700e5,
        KeyCode::AltRight => 0x000700e6,
        // KeyCode::Info => 0x000c0060,
        // KeyCode::ClosedCaptionToggle => 0x000c0061,
        // KeyCode::BrightnessUp => 0x000c006f,
        // KeyCode::BrightnessDown => 0x000c0070,
        // KeyCode::BrightnessToggle => 0x000c0072,
        // KeyCode::BrightnessMinimum => 0x000c0073,
        // KeyCode::BrightnessMaximum => 0x000c0074,
        // KeyCode::BrightnessAuto => 0x000c0075,
        // KeyCode::KbdIllumUp => 0x000c0079,
        // KeyCode::KbdIllumDown => 0x000c007a,
        // KeyCode::MediaLast => 0x000c0083,
        // KeyCode::LaunchPhone => 0x000c008c,
        // KeyCode::ProgramGuide => 0x000c008d,
        // KeyCode::Exit => 0x000c0094,
        // KeyCode::ChannelUp => 0x000c009c,
        // KeyCode::ChannelDown => 0x000c009d,
        // KeyCode::MediaPlay => 0x000c00b0,
        // KeyCode::MediaPause => 0x000c00b1,
        // KeyCode::MediaRecord => 0x000c00b2,
        // KeyCode::MediaFastForward => 0x000c00b3,
        // KeyCode::MediaRewind => 0x000c00b4,
        KeyCode::MediaTrackNext => 0x000c00b5,
        KeyCode::MediaTrackPrevious => 0x000c00b6,
        KeyCode::MediaStop => 0x000c00b7,
        KeyCode::Eject => 0x000c00b8,
        KeyCode::MediaPlayPause => 0x000c00cd,
        // KeyCode::SpeechInputToggle => 0x000c00cf,
        // KeyCode::BassBoost => 0x000c00e5,
        KeyCode::MediaSelect => 0x000c0183,
        // KeyCode::LaunchWordProcessor => 0x000c0184,
        // KeyCode::LaunchSpreadsheet => 0x000c0186,
        KeyCode::LaunchMail => 0x000c018a,
        // KeyCode::LaunchContacts => 0x000c018d,
        // KeyCode::LaunchCalendar => 0x000c018e,
        KeyCode::LaunchApp2 => 0x000c0192,
        KeyCode::LaunchApp1 => 0x000c0194,
        // KeyCode::LaunchInternetBrowser => 0x000c0196,
        // KeyCode::LogOff => 0x000c019c,
        // KeyCode::LockScreen => 0x000c019e,
        // KeyCode::LaunchControlPanel => 0x000c019f,
        // KeyCode::SelectTask => 0x000c01a2,
        // KeyCode::LaunchDocuments => 0x000c01a7,
        // KeyCode::SpellCheck => 0x000c01ab,
        // KeyCode::LaunchKeyboardLayout => 0x000c01ae,
        // KeyCode::LaunchScreenSaver => 0x000c01b1,
        // KeyCode::LaunchAudioBrowser => 0x000c01b7,
        // KeyCode::LaunchAssistant => 0x000c01cb,
        // KeyCode::NewKey => 0x000c0201,
        // KeyCode::Close => 0x000c0203,
        // KeyCode::Save => 0x000c0207,
        // KeyCode::Print => 0x000c0208,
        KeyCode::BrowserSearch => 0x000c0221,
        KeyCode::BrowserHome => 0x000c0223,
        KeyCode::BrowserBack => 0x000c0224,
        KeyCode::BrowserForward => 0x000c0225,
        KeyCode::BrowserStop => 0x000c0226,
        KeyCode::BrowserRefresh => 0x000c0227,
        KeyCode::BrowserFavorites => 0x000c022a,
        // KeyCode::ZoomIn => 0x000c022d,
        // KeyCode::ZoomOut => 0x000c022e,
        // KeyCode::ZoomToggle => 0x000c0232,
        // KeyCode::Redo => 0x000c0279,
        // KeyCode::MailReply => 0x000c0289,
        // KeyCode::MailForward => 0x000c028b,
        // KeyCode::MailSend => 0x000c028c,
        // KeyCode::KeyboardLayoutSelect => 0x000c029d,
        // KeyCode::ShowAllWindows => 0x000c029f,
        // KeyCode::NumpadHash => return None,
        // KeyCode::NumpadStar => return None,
        // KeyCode::Hiragana => return None,
        // KeyCode::Katakana => return None,
        // KeyCode::F25 => return None,
        // KeyCode::F26 => return None,
        // KeyCode::F27 => return None,
        // KeyCode::F28 => return None,
        // KeyCode::F29 => return None,
        // KeyCode::F30 => return None,
        // KeyCode::F31 => return None,
        // KeyCode::F32 => return None,
        // KeyCode::F33 => return None,
        // KeyCode::F34 => return None,
        // KeyCode::F35 => return None,
        _ => return None,
    })
}

// TODO: all other platforms
