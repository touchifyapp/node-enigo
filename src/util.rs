use crate::error::{JsError, Result};
use napi_derive::napi;

#[napi(string_enum, js_name = "Button")]
#[allow(non_camel_case_types)]
pub enum JsButton {
    Left,
    left,
    Middle,
    middle,
    Center,
    center,
    Right,
    right,
    Back,
    back,
    Forward,
    forward,
}

impl From<&JsButton> for enigo::Button {
    fn from(value: &JsButton) -> Self {
        match value {
            JsButton::Left | JsButton::left => Self::Left,
            JsButton::Middle | JsButton::middle | JsButton::Center | JsButton::center => {
                Self::Middle
            }
            JsButton::Right | JsButton::right => Self::Right,
            JsButton::Back | JsButton::back => Self::Back,
            JsButton::Forward | JsButton::forward => Self::Forward,
        }
    }
}

#[napi(string_enum, js_name = "Direction")]
#[allow(non_camel_case_types)]
pub enum JsDirection {
    Press,
    press,
    Down,
    down,
    Release,
    release,
    Up,
    up,
    Click,
    click,
}

impl From<&JsDirection> for enigo::Direction {
    fn from(value: &JsDirection) -> Self {
        match value {
            JsDirection::Press | JsDirection::Down | JsDirection::press | JsDirection::down => {
                enigo::Direction::Press
            }
            JsDirection::Release | JsDirection::Up | JsDirection::release | JsDirection::up => {
                enigo::Direction::Release
            }
            JsDirection::Click | JsDirection::click => enigo::Direction::Click,
        }
    }
}

#[napi(string_enum, js_name = "Axis")]
#[allow(non_camel_case_types)]
pub enum JsAxis {
    Horizontal,
    horizontal,
    Vertical,
    vertical,
}

impl From<&JsAxis> for enigo::Axis {
    fn from(value: &JsAxis) -> Self {
        match value {
            JsAxis::Horizontal | JsAxis::horizontal => enigo::Axis::Horizontal,
            JsAxis::Vertical | JsAxis::vertical => enigo::Axis::Vertical,
        }
    }
}

pub fn key_from_string(key: String) -> Result<enigo::Key> {
    match key.as_str() {
        #[cfg(target_os = "windows")]
        "Num0" | "num0" => Ok(enigo::Key::Num0),
        #[cfg(target_os = "windows")]
        "Num1" | "num1" => Ok(enigo::Key::Num1),
        #[cfg(target_os = "windows")]
        "Num2" | "num2" => Ok(enigo::Key::Num2),
        #[cfg(target_os = "windows")]
        "Num3" | "num3" => Ok(enigo::Key::Num3),
        #[cfg(target_os = "windows")]
        "Num4" | "num4" => Ok(enigo::Key::Num4),
        #[cfg(target_os = "windows")]
        "Num5" | "num5" => Ok(enigo::Key::Num5),
        #[cfg(target_os = "windows")]
        "Num6" | "num6" => Ok(enigo::Key::Num6),
        #[cfg(target_os = "windows")]
        "Num7" | "num7" => Ok(enigo::Key::Num7),
        #[cfg(target_os = "windows")]
        "Num8" | "num8" => Ok(enigo::Key::Num8),
        #[cfg(target_os = "windows")]
        "Num9" | "num9" => Ok(enigo::Key::Num9),
        #[cfg(target_os = "windows")]
        "A" => Ok(enigo::Key::A),
        #[cfg(target_os = "windows")]
        "B" => Ok(enigo::Key::B),
        #[cfg(target_os = "windows")]
        "C" => Ok(enigo::Key::C),
        #[cfg(target_os = "windows")]
        "D" => Ok(enigo::Key::D),
        #[cfg(target_os = "windows")]
        "E" => Ok(enigo::Key::E),
        #[cfg(target_os = "windows")]
        "F" => Ok(enigo::Key::F),
        #[cfg(target_os = "windows")]
        "G" => Ok(enigo::Key::G),
        #[cfg(target_os = "windows")]
        "H" => Ok(enigo::Key::H),
        #[cfg(target_os = "windows")]
        "I" => Ok(enigo::Key::I),
        #[cfg(target_os = "windows")]
        "J" => Ok(enigo::Key::J),
        #[cfg(target_os = "windows")]
        "K" => Ok(enigo::Key::K),
        #[cfg(target_os = "windows")]
        "L" => Ok(enigo::Key::L),
        #[cfg(target_os = "windows")]
        "M" => Ok(enigo::Key::M),
        #[cfg(target_os = "windows")]
        "N" => Ok(enigo::Key::N),
        #[cfg(target_os = "windows")]
        "O" => Ok(enigo::Key::O),
        #[cfg(target_os = "windows")]
        "P" => Ok(enigo::Key::P),
        #[cfg(target_os = "windows")]
        "Q" => Ok(enigo::Key::Q),
        #[cfg(target_os = "windows")]
        "R" => Ok(enigo::Key::R),
        #[cfg(target_os = "windows")]
        "S" => Ok(enigo::Key::S),
        #[cfg(target_os = "windows")]
        "T" => Ok(enigo::Key::T),
        #[cfg(target_os = "windows")]
        "U" => Ok(enigo::Key::U),
        #[cfg(target_os = "windows")]
        "V" => Ok(enigo::Key::V),
        #[cfg(target_os = "windows")]
        "W" => Ok(enigo::Key::W),
        #[cfg(target_os = "windows")]
        "X" => Ok(enigo::Key::X),
        #[cfg(target_os = "windows")]
        "Y" => Ok(enigo::Key::Y),
        #[cfg(target_os = "windows")]
        "Z" => Ok(enigo::Key::Z),
        #[cfg(target_os = "windows")]
        "AbntC1" | "abntc1" => Ok(enigo::Key::AbntC1),
        #[cfg(target_os = "windows")]
        "AbntC2" | "abntc2" => Ok(enigo::Key::AbntC2),
        #[cfg(target_os = "windows")]
        "Accept" | "accept" => Ok(enigo::Key::Accept),
        "Add" | "add" => Ok(enigo::Key::Add),
        // alt key on Linux and Windows (option key on macOS)
        "Alt" | "alt" => Ok(enigo::Key::Alt),
        #[cfg(target_os = "windows")]
        "Apps" | "apps" => Ok(enigo::Key::Apps),
        #[cfg(target_os = "windows")]
        "Attn" | "attn" => Ok(enigo::Key::Attn),
        "Backspace" | "backspace" => Ok(enigo::Key::Backspace),
        #[cfg(all(unix, not(target_os = "macos")))]
        "Break" | "break" => Ok(enigo::Key::Break),
        #[cfg(all(unix, not(target_os = "macos")))]
        "Begin" | "begin" => Ok(enigo::Key::Begin),
        #[cfg(target_os = "macos")]
        "BrightnessDown" | "brightnessdown" => Ok(enigo::Key::BrightnessDown),
        #[cfg(target_os = "macos")]
        "BrightnessUp" | "brightnessup" => Ok(enigo::Key::BrightnessUp),
        #[cfg(target_os = "windows")]
        "BrowserBack" | "browserback" => Ok(enigo::Key::BrowserBack),
        #[cfg(target_os = "windows")]
        "BrowserFavorites" | "browserfavorites" => Ok(enigo::Key::BrowserFavorites),
        #[cfg(target_os = "windows")]
        "BrowserForward" | "browserforward" => Ok(enigo::Key::BrowserForward),
        #[cfg(target_os = "windows")]
        "BrowserHome" | "browserhome" => Ok(enigo::Key::BrowserHome),
        #[cfg(target_os = "windows")]
        "BrowserRefresh" | "browserrefresh" => Ok(enigo::Key::BrowserRefresh),
        #[cfg(target_os = "windows")]
        "BrowserSearch" | "browsersearch" => Ok(enigo::Key::BrowserSearch),
        #[cfg(target_os = "windows")]
        "BrowserStop" | "browserstop" => Ok(enigo::Key::BrowserStop),
        #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
        "Cancel" | "cancel" => Ok(enigo::Key::Cancel),
        "CapsLock" | "capslock" => Ok(enigo::Key::CapsLock),
        #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
        "Clear" | "clear" => Ok(enigo::Key::Clear),
        #[cfg(target_os = "macos")]
        "ContrastUp" | "contrastup" => Ok(enigo::Key::ContrastUp),
        #[cfg(target_os = "macos")]
        "ContrastDown" | "contrastdown" => Ok(enigo::Key::ContrastDown),
        "Control" | "control" | "Ctrl" | "ctrl" => Ok(enigo::Key::Control),
        #[cfg(target_os = "windows")]
        "Convert" | "convert" => Ok(enigo::Key::Convert),
        #[cfg(target_os = "windows")]
        "Crsel" | "crsel" => Ok(enigo::Key::Crsel),
        #[cfg(target_os = "windows")]
        "DBEAlphanumeric" | "dbealphanumeric" => Ok(enigo::Key::DBEAlphanumeric),
        #[cfg(target_os = "windows")]
        "DBECodeinput" | "dbecodeinput" => Ok(enigo::Key::DBECodeinput),
        #[cfg(target_os = "windows")]
        "DBEDetermineString" | "dbedeterminestring" => Ok(enigo::Key::DBEDetermineString),
        #[cfg(target_os = "windows")]
        "DBEEnterDLGConversionMode" | "dbeenterdlgconversionmode" => {
            Ok(enigo::Key::DBEEnterDLGConversionMode)
        }
        #[cfg(target_os = "windows")]
        "DBEEnterIMEConfigMode" | "dbeenterimeconfigmode" => Ok(enigo::Key::DBEEnterIMEConfigMode),
        #[cfg(target_os = "windows")]
        "DBEEnterWordRegisterMode" | "dbeenterwordregistermode" => {
            Ok(enigo::Key::DBEEnterWordRegisterMode)
        }
        #[cfg(target_os = "windows")]
        "DBEFlushString" | "dbeflushstring" => Ok(enigo::Key::DBEFlushString),
        #[cfg(target_os = "windows")]
        "DBEHiragana" | "dbehiragana" => Ok(enigo::Key::DBEHiragana),
        #[cfg(target_os = "windows")]
        "DBEKatakana" | "dbekatakana" => Ok(enigo::Key::DBEKatakana),
        #[cfg(target_os = "windows")]
        "DBENoCodepoint" | "dbenocodepoint" => Ok(enigo::Key::DBENoCodepoint),
        #[cfg(target_os = "windows")]
        "DBENoRoman" | "dbenoroman" => Ok(enigo::Key::DBENoRoman),
        #[cfg(target_os = "windows")]
        "DBERoman" | "dberoman" => Ok(enigo::Key::DBERoman),
        #[cfg(target_os = "windows")]
        "DBESBCSChar" | "dbesbcschar" => Ok(enigo::Key::DBESBCSChar),
        #[cfg(target_os = "windows")]
        "DBESChar" | "dbeschar" => Ok(enigo::Key::DBESChar),
        "Decimal" | "decimal" => Ok(enigo::Key::Decimal),
        "Delete" | "delete" => Ok(enigo::Key::Delete),
        "Divide" | "divide" => Ok(enigo::Key::Divide),
        "DownArrow" | "downarrow" | "down" => Ok(enigo::Key::DownArrow),
        #[cfg(target_os = "macos")]
        "Eject" | "eject" => Ok(enigo::Key::Eject),
        "End" | "end" => Ok(enigo::Key::End),
        #[cfg(target_os = "windows")]
        "Ereof" | "ereof" => Ok(enigo::Key::Ereof),
        "Escape" | "escape" | "esc" => Ok(enigo::Key::Escape),
        #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
        "Execute" | "execute" => Ok(enigo::Key::Execute),
        #[cfg(target_os = "windows")]
        "Exsel" | "exsel" => Ok(enigo::Key::Exsel),
        "F1" | "f1" => Ok(enigo::Key::F1),
        "F2" | "f2" => Ok(enigo::Key::F2),
        "F3" | "f3" => Ok(enigo::Key::F3),
        "F4" | "f4" => Ok(enigo::Key::F4),
        "F5" | "f5" => Ok(enigo::Key::F5),
        "F6" | "f6" => Ok(enigo::Key::F6),
        "F7" | "f7" => Ok(enigo::Key::F7),
        "F8" | "f8" => Ok(enigo::Key::F8),
        "F9" | "f9" => Ok(enigo::Key::F9),
        "F10" | "f10" => Ok(enigo::Key::F10),
        "F11" | "f11" => Ok(enigo::Key::F11),
        "F12" | "f12" => Ok(enigo::Key::F12),
        "F13" | "f13" => Ok(enigo::Key::F13),
        "F14" | "f14" => Ok(enigo::Key::F14),
        "F15" | "f15" => Ok(enigo::Key::F15),
        "F16" | "f16" => Ok(enigo::Key::F16),
        "F17" | "f17" => Ok(enigo::Key::F17),
        "F18" | "f18" => Ok(enigo::Key::F18),
        "F19" | "f19" => Ok(enigo::Key::F19),
        "F20" | "f20" => Ok(enigo::Key::F20),
        #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
        "F21" | "f21" => Ok(enigo::Key::F21),
        #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
        "F22" | "f22" => Ok(enigo::Key::F22),
        #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
        "F23" | "f23" => Ok(enigo::Key::F23),
        #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
        "F24" | "f24" => Ok(enigo::Key::F24),
        #[cfg(all(unix, not(target_os = "macos")))]
        "F25" | "f25" => Ok(enigo::Key::F25),
        #[cfg(all(unix, not(target_os = "macos")))]
        "F26" | "f26" => Ok(enigo::Key::F26),
        #[cfg(all(unix, not(target_os = "macos")))]
        "F27" | "f27" => Ok(enigo::Key::F27),
        #[cfg(all(unix, not(target_os = "macos")))]
        "F28" | "f28" => Ok(enigo::Key::F28),
        #[cfg(all(unix, not(target_os = "macos")))]
        "F29" | "f29" => Ok(enigo::Key::F29),
        #[cfg(all(unix, not(target_os = "macos")))]
        "F30" | "f30" => Ok(enigo::Key::F30),
        #[cfg(all(unix, not(target_os = "macos")))]
        "F31" | "f31" => Ok(enigo::Key::F31),
        #[cfg(all(unix, not(target_os = "macos")))]
        "F32" | "f32" => Ok(enigo::Key::F32),
        #[cfg(all(unix, not(target_os = "macos")))]
        "F33" | "f33" => Ok(enigo::Key::F33),
        #[cfg(all(unix, not(target_os = "macos")))]
        "F34" | "f34" => Ok(enigo::Key::F34),
        #[cfg(all(unix, not(target_os = "macos")))]
        "F35" | "f35" => Ok(enigo::Key::F35),
        #[cfg(target_os = "macos")]
        "Function" | "function" => Ok(enigo::Key::Function),
        #[cfg(target_os = "windows")]
        "Final" | "final" => Ok(enigo::Key::Final),
        #[cfg(all(unix, not(target_os = "macos")))]
        "Find" | "find" => Ok(enigo::Key::Find),
        #[cfg(target_os = "windows")]
        "GamepadA" | "gamepada" => Ok(enigo::Key::GamepadA),
        #[cfg(target_os = "windows")]
        "GamepadB" | "gamepadb" => Ok(enigo::Key::GamepadB),
        #[cfg(target_os = "windows")]
        "GamepadDPadDown" | "gamepaddpaddown" => Ok(enigo::Key::GamepadDPadDown),
        #[cfg(target_os = "windows")]
        "GamepadDPadLeft" | "gamepaddpadleft" => Ok(enigo::Key::GamepadDPadLeft),
        #[cfg(target_os = "windows")]
        "GamepadDPadRight" | "gamepaddpadright" => Ok(enigo::Key::GamepadDPadRight),
        #[cfg(target_os = "windows")]
        "GamepadDPadUp" | "gamepaddpadup" => Ok(enigo::Key::GamepadDPadUp),
        #[cfg(target_os = "windows")]
        "GamepadLeftShoulder" | "gamepadleftshoulder" => Ok(enigo::Key::GamepadLeftShoulder),
        #[cfg(target_os = "windows")]
        "GamepadLeftThumbstickButton" | "gamepadleftthumbstickbutton" => {
            Ok(enigo::Key::GamepadLeftThumbstickButton)
        }
        #[cfg(target_os = "windows")]
        "GamepadLeftThumbstickDown" | "gamepadleftthumbstickdown" => {
            Ok(enigo::Key::GamepadLeftThumbstickDown)
        }
        #[cfg(target_os = "windows")]
        "GamepadLeftThumbstickLeft" | "gamepadleftthumbstickleft" => {
            Ok(enigo::Key::GamepadLeftThumbstickLeft)
        }
        #[cfg(target_os = "windows")]
        "GamepadLeftThumbstickRight" | "gamepadleftthumbstickright" => {
            Ok(enigo::Key::GamepadLeftThumbstickRight)
        }
        #[cfg(target_os = "windows")]
        "GamepadLeftThumbstickUp" | "gamepadleftthumbstickup" => {
            Ok(enigo::Key::GamepadLeftThumbstickUp)
        }
        #[cfg(target_os = "windows")]
        "GamepadLeftTrigger" | "gamepadlefttrigger" => Ok(enigo::Key::GamepadLeftTrigger),
        #[cfg(target_os = "windows")]
        "GamepadMenu" | "gamepadmenu" => Ok(enigo::Key::GamepadMenu),
        #[cfg(target_os = "windows")]
        "GamepadRightShoulder" | "gamepadrightshoulder" => Ok(enigo::Key::GamepadRightShoulder),
        #[cfg(target_os = "windows")]
        "GamepadRightThumbstickButton" | "gamepadrightthumbstickbutton" => {
            Ok(enigo::Key::GamepadRightThumbstickButton)
        }
        #[cfg(target_os = "windows")]
        "GamepadRightThumbstickDown" | "gamepadrightthumbstickdown" => {
            Ok(enigo::Key::GamepadRightThumbstickDown)
        }
        #[cfg(target_os = "windows")]
        "GamepadRightThumbstickLeft" | "gamepadrightthumbstickleft" => {
            Ok(enigo::Key::GamepadRightThumbstickLeft)
        }
        #[cfg(target_os = "windows")]
        "GamepadRightThumbstickRight" | "gamepadrightthumbstickright" => {
            Ok(enigo::Key::GamepadRightThumbstickRight)
        }
        #[cfg(target_os = "windows")]
        "GamepadRightThumbstickUp" | "gamepadrightthumbstickup" => {
            Ok(enigo::Key::GamepadRightThumbstickUp)
        }
        #[cfg(target_os = "windows")]
        "GamepadRightTrigger" | "gamepadrighttrigger" => Ok(enigo::Key::GamepadRightTrigger),
        #[cfg(target_os = "windows")]
        "GamepadView" | "gamepadview" => Ok(enigo::Key::GamepadView),
        #[cfg(target_os = "windows")]
        "GamepadX" | "gamepadx" => Ok(enigo::Key::GamepadX),
        #[cfg(target_os = "windows")]
        "GamepadY" | "gamepady" => Ok(enigo::Key::GamepadY),
        #[cfg(target_os = "windows")]
        "Hangeul" | "hangeul" => Ok(enigo::Key::Hangeul),
        #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
        "Hangul" | "hangul" => Ok(enigo::Key::Hangul),
        #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
        "Hanja" | "hanja" => Ok(enigo::Key::Hanja),
        "Help" | "help" => Ok(enigo::Key::Help),
        "Home" | "home" => Ok(enigo::Key::Home),
        #[cfg(target_os = "windows")]
        "Ico00" | "ico00" => Ok(enigo::Key::Ico00),
        #[cfg(target_os = "windows")]
        "IcoClear" | "icoclear" => Ok(enigo::Key::IcoClear),
        #[cfg(target_os = "windows")]
        "IcoHelp" | "icohelp" => Ok(enigo::Key::IcoHelp),
        #[cfg(target_os = "macos")]
        "IlluminationDown" | "illuminationdown" => Ok(enigo::Key::IlluminationDown),
        #[cfg(target_os = "macos")]
        "IlluminationUp" | "illuminationup" => Ok(enigo::Key::IlluminationUp),
        #[cfg(target_os = "macos")]
        "IlluminationToggle" | "illuminationtoggle" => Ok(enigo::Key::IlluminationToggle),
        #[cfg(target_os = "windows")]
        "IMEOff" | "imeoff" => Ok(enigo::Key::IMEOff),
        #[cfg(target_os = "windows")]
        "IMEOn" | "imeon" => Ok(enigo::Key::IMEOn),
        #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
        "Insert" | "insert" => Ok(enigo::Key::Insert),
        #[cfg(target_os = "windows")]
        "Junja" | "junja" => Ok(enigo::Key::Junja),
        #[cfg(target_os = "windows")]
        "Kana" | "kana" => Ok(enigo::Key::Kana),
        #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
        "Kanji" | "kanji" => Ok(enigo::Key::Kanji),
        #[cfg(target_os = "windows")]
        "LaunchApp1" | "launchapp1" => Ok(enigo::Key::LaunchApp1),
        #[cfg(target_os = "windows")]
        "LaunchApp2" | "launchapp2" => Ok(enigo::Key::LaunchApp2),
        #[cfg(target_os = "windows")]
        "LaunchMail" | "launchmail" => Ok(enigo::Key::LaunchMail),
        #[cfg(target_os = "windows")]
        "LaunchMediaSelect" | "launchmediaselect" => Ok(enigo::Key::LaunchMediaSelect),
        #[cfg(target_os = "macos")]
        "Launchpad" | "launchpad" => Ok(enigo::Key::Launchpad),
        #[cfg(target_os = "macos")]
        "LaunchPanel" | "launchpanel" => Ok(enigo::Key::LaunchPanel),
        #[cfg(target_os = "windows")]
        "LButton" | "lbutton" => Ok(enigo::Key::LButton),
        "LControl" | "lcontrol" => Ok(enigo::Key::LControl),
        "LeftArrow" | "leftarrow" => Ok(enigo::Key::LeftArrow),
        #[cfg(all(unix, not(target_os = "macos")))]
        "Linefeed" | "linefeed" => Ok(enigo::Key::Linefeed),
        #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
        "LMenu" | "lmenu" => Ok(enigo::Key::LMenu),
        "LShift" | "lshift" => Ok(enigo::Key::LShift),
        #[cfg(target_os = "windows")]
        "LWin" | "lwin" => Ok(enigo::Key::LWin),
        #[cfg(target_os = "windows")]
        "MButton" | "mbutton" => Ok(enigo::Key::MButton),
        #[cfg(target_os = "macos")]
        "MediaFast" | "mediafast" => Ok(enigo::Key::MediaFast),
        "MediaNextTrack" | "medianexttrack" => Ok(enigo::Key::MediaNextTrack),
        "MediaPlayPause" | "mediaplaypause" => Ok(enigo::Key::MediaPlayPause),
        "MediaPrevTrack" | "mediaprevtrack" => Ok(enigo::Key::MediaPrevTrack),
        #[cfg(target_os = "macos")]
        "MediaRewind" | "mediarewind" => Ok(enigo::Key::MediaRewind),
        #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
        "MediaStop" | "mediastop" => Ok(enigo::Key::MediaStop),
        "Meta" | "meta" | "Command" | "command" | "cmd" | "Super" | "super" | "Windows"
        | "windows" | "win" => Ok(enigo::Key::Meta),
        #[cfg(target_os = "macos")]
        "MissionControl" | "missioncontrol" => Ok(enigo::Key::MissionControl),
        #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
        "ModeChange" | "modechange" => Ok(enigo::Key::ModeChange),
        "Multiply" | "multiply" => Ok(enigo::Key::Multiply),
        #[cfg(target_os = "windows")]
        "NavigationAccept" | "navigationaccept" => Ok(enigo::Key::NavigationAccept),
        #[cfg(target_os = "windows")]
        "NavigationCancel" | "navigationcancel" => Ok(enigo::Key::NavigationCancel),
        #[cfg(target_os = "windows")]
        "NavigationDown" | "navigationdown" => Ok(enigo::Key::NavigationDown),
        #[cfg(target_os = "windows")]
        "NavigationLeft" | "navigationleft" => Ok(enigo::Key::NavigationLeft),
        #[cfg(target_os = "windows")]
        "NavigationMenu" | "navigationmenu" => Ok(enigo::Key::NavigationMenu),
        #[cfg(target_os = "windows")]
        "NavigationRight" | "navigationright" => Ok(enigo::Key::NavigationRight),
        #[cfg(target_os = "windows")]
        "NavigationUp" | "navigationup" => Ok(enigo::Key::NavigationUp),
        #[cfg(target_os = "windows")]
        "NavigationView" | "navigationview" => Ok(enigo::Key::NavigationView),
        #[cfg(target_os = "windows")]
        "NoName" | "noname" => Ok(enigo::Key::NoName),
        #[cfg(target_os = "windows")]
        "NonConvert" | "nonconvert" => Ok(enigo::Key::NonConvert),
        #[cfg(target_os = "windows")]
        "None" | "none" => Ok(enigo::Key::None),
        #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
        "Numlock" | "numlock" => Ok(enigo::Key::Numlock),
        "Numpad0" | "numpad0" => Ok(enigo::Key::Numpad0),
        "Numpad1" | "numpad1" => Ok(enigo::Key::Numpad1),
        "Numpad2" | "numpad2" => Ok(enigo::Key::Numpad2),
        "Numpad3" | "numpad3" => Ok(enigo::Key::Numpad3),
        "Numpad4" | "numpad4" => Ok(enigo::Key::Numpad4),
        "Numpad5" | "numpad5" => Ok(enigo::Key::Numpad5),
        "Numpad6" | "numpad6" => Ok(enigo::Key::Numpad6),
        "Numpad7" | "numpad7" => Ok(enigo::Key::Numpad7),
        "Numpad8" | "numpad8" => Ok(enigo::Key::Numpad8),
        "Numpad9" | "numpad9" => Ok(enigo::Key::Numpad9),
        #[cfg(target_os = "windows")]
        "OEM1" | "oem1" => Ok(enigo::Key::OEM1),
        #[cfg(target_os = "windows")]
        "OEM102" | "oem102" => Ok(enigo::Key::OEM102),
        #[cfg(target_os = "windows")]
        "OEM2" | "oem2" => Ok(enigo::Key::OEM2),
        #[cfg(target_os = "windows")]
        "OEM3" | "oem3" => Ok(enigo::Key::OEM3),
        #[cfg(target_os = "windows")]
        "OEM4" | "oem4" => Ok(enigo::Key::OEM4),
        #[cfg(target_os = "windows")]
        "OEM5" | "oem5" => Ok(enigo::Key::OEM5),
        #[cfg(target_os = "windows")]
        "OEM6" | "oem6" => Ok(enigo::Key::OEM6),
        #[cfg(target_os = "windows")]
        "OEM7" | "oem7" => Ok(enigo::Key::OEM7),
        #[cfg(target_os = "windows")]
        "OEM8" | "oem8" => Ok(enigo::Key::OEM8),
        #[cfg(target_os = "windows")]
        "OEMAttn" | "oemattn" => Ok(enigo::Key::OEMAttn),
        #[cfg(target_os = "windows")]
        "OEMAuto" | "oemauto" => Ok(enigo::Key::OEMAuto),
        #[cfg(target_os = "windows")]
        "OEMAx" | "oemax" => Ok(enigo::Key::OEMAx),
        #[cfg(target_os = "windows")]
        "OEMBacktab" | "oembacktab" => Ok(enigo::Key::OEMBacktab),
        #[cfg(target_os = "windows")]
        "OEMClear" | "oemclear" => Ok(enigo::Key::OEMClear),
        #[cfg(target_os = "windows")]
        "OEMComma" | "oemcomma" => Ok(enigo::Key::OEMComma),
        #[cfg(target_os = "windows")]
        "OEMCopy" | "oemcopy" => Ok(enigo::Key::OEMCopy),
        #[cfg(target_os = "windows")]
        "OEMCusel" | "oemcusel" => Ok(enigo::Key::OEMCusel),
        #[cfg(target_os = "windows")]
        "OEMEnlw" | "oemenlw" => Ok(enigo::Key::OEMEnlw),
        #[cfg(target_os = "windows")]
        "OEMFinish" | "oemfinish" => Ok(enigo::Key::OEMFinish),
        #[cfg(target_os = "windows")]
        "OEMFJJisho" | "oemfjjisho" => Ok(enigo::Key::OEMFJJisho),
        #[cfg(target_os = "windows")]
        "OEMFJLoya" | "oemfjloya" => Ok(enigo::Key::OEMFJLoya),
        #[cfg(target_os = "windows")]
        "OEMFJMasshou" | "oemfjmasshou" => Ok(enigo::Key::OEMFJMasshou),
        #[cfg(target_os = "windows")]
        "OEMFJRoya" | "oemfjroya" => Ok(enigo::Key::OEMFJRoya),
        #[cfg(target_os = "windows")]
        "OEMFJTouroku" | "oemfjtouroku" => Ok(enigo::Key::OEMFJTouroku),
        #[cfg(target_os = "windows")]
        "OEMJump" | "oemjump" => Ok(enigo::Key::OEMJump),
        #[cfg(target_os = "windows")]
        "OEMMinus" | "oemminus" => Ok(enigo::Key::OEMMinus),
        #[cfg(target_os = "windows")]
        "OEMNECEqual" | "oemnecequal" => Ok(enigo::Key::OEMNECEqual),
        #[cfg(target_os = "windows")]
        "OEMPA1" | "oempa1" => Ok(enigo::Key::OEMPA1),
        #[cfg(target_os = "windows")]
        "OEMPA2" | "oempa2" => Ok(enigo::Key::OEMPA2),
        #[cfg(target_os = "windows")]
        "OEMPA3" | "oempa3" => Ok(enigo::Key::OEMPA3),
        #[cfg(target_os = "windows")]
        "OEMPeriod" | "oemperiod" => Ok(enigo::Key::OEMPeriod),
        #[cfg(target_os = "windows")]
        "OEMPlus" | "oemplus" => Ok(enigo::Key::OEMPlus),
        #[cfg(target_os = "windows")]
        "OEMReset" | "oemreset" => Ok(enigo::Key::OEMReset),
        #[cfg(target_os = "windows")]
        "OEMWsctrl" | "oemwsctrl" => Ok(enigo::Key::OEMWsctrl),
        // option key on macOS (alt key on Linux and Windows)
        "Option" | "option" => Ok(enigo::Key::Option),
        #[cfg(target_os = "windows")]
        "PA1" | "pa1" => Ok(enigo::Key::PA1),
        #[cfg(target_os = "windows")]
        "Packet" | "packet" => Ok(enigo::Key::Packet),
        "PageDown" | "pagedown" => Ok(enigo::Key::PageDown),
        "PageUp" | "pageup" => Ok(enigo::Key::PageUp),
        #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
        "Pause" | "pause" => Ok(enigo::Key::Pause),
        #[cfg(target_os = "windows")]
        "Play" | "play" => Ok(enigo::Key::Play),
        #[cfg(target_os = "macos")]
        "Power" | "power" => Ok(enigo::Key::Power),
        #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
        "PrintScr" | "printscr" | "Print" | "print" | "Snapshot" | "snapshot" => {
            Ok(enigo::Key::PrintScr)
        }
        #[cfg(target_os = "windows")]
        "Processkey" | "processkey" => Ok(enigo::Key::Processkey),
        #[cfg(target_os = "windows")]
        "RButton" | "rbutton" => Ok(enigo::Key::RButton),
        #[cfg(target_os = "macos")]
        "RCommand" | "rcommand" => Ok(enigo::Key::RCommand),
        "RControl" | "rcontrol" => Ok(enigo::Key::RControl),
        #[cfg(all(unix, not(target_os = "macos")))]
        "Redo" | "redo" => Ok(enigo::Key::Redo),
        "Return" | "return" => Ok(enigo::Key::Return),
        "RightArrow" | "rightarrow" => Ok(enigo::Key::RightArrow),
        #[cfg(target_os = "windows")]
        "RMenu" | "rmenu" => Ok(enigo::Key::RMenu),
        #[cfg(target_os = "macos")]
        "ROption" | "roption" => Ok(enigo::Key::ROption),
        "RShift" | "rshift" => Ok(enigo::Key::RShift),
        #[cfg(target_os = "windows")]
        "RWin" | "rwin" => Ok(enigo::Key::RWin),
        #[cfg(target_os = "windows")]
        "Scroll" | "scroll" => Ok(enigo::Key::Scroll),
        #[cfg(all(unix, not(target_os = "macos")))]
        "ScrollLock" | "scrolllock" => Ok(enigo::Key::ScrollLock),
        #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
        "Select" | "select" => Ok(enigo::Key::Select),
        #[cfg(all(unix, not(target_os = "macos")))]
        "ScriptSwitch" | "scriptswitch" => Ok(enigo::Key::ScriptSwitch),
        #[cfg(target_os = "windows")]
        "Separator" | "separator" => Ok(enigo::Key::Separator),
        "Shift" | "shift" => Ok(enigo::Key::Shift),
        #[cfg(all(unix, not(target_os = "macos")))]
        "ShiftLock" | "shiftlock" => Ok(enigo::Key::ShiftLock),
        #[cfg(target_os = "windows")]
        "Sleep" | "sleep" => Ok(enigo::Key::Sleep),
        "Space" | "space" => Ok(enigo::Key::Space),
        "Subtract" | "subtract" => Ok(enigo::Key::Subtract),
        #[cfg(all(unix, not(target_os = "macos")))]
        "SysReq" | "sysreq" => Ok(enigo::Key::SysReq),
        "Tab" | "tab" => Ok(enigo::Key::Tab),
        #[cfg(all(unix, not(target_os = "macos")))]
        "Undo" | "undo" => Ok(enigo::Key::Undo),
        "UpArrow" | "uparrow" => Ok(enigo::Key::UpArrow),
        #[cfg(target_os = "macos")]
        "VidMirror" | "vidmirror" => Ok(enigo::Key::VidMirror),
        "VolumeDown" | "volumedown" => Ok(enigo::Key::VolumeDown),
        "VolumeMute" | "volumemute" => Ok(enigo::Key::VolumeMute),
        "VolumeUp" | "volumeup" => Ok(enigo::Key::VolumeUp),
        #[cfg(all(unix, not(target_os = "macos")))]
        "MicMute" | "micmute" => Ok(enigo::Key::MicMute),
        #[cfg(target_os = "windows")]
        "XButton1" | "xbutton1" => Ok(enigo::Key::XButton1),
        #[cfg(target_os = "windows")]
        "XButton2" | "xbutton2" => Ok(enigo::Key::XButton2),
        #[cfg(target_os = "windows")]
        "Zoom" | "zoom" => Ok(enigo::Key::Zoom),

        key => {
            if key.len() == 1 {
                let char = char_from_string(key)?;
                Ok(enigo::Key::Unicode(char))
            } else {
                Err(JsError::invalid_input(key))
            }
        }
    }
}

fn char_from_string(key: &str) -> Result<char> {
    key.chars()
        .next()
        .ok_or_else(|| JsError::invalid_input(&key))
}
