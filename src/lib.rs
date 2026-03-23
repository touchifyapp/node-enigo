#![deny(clippy::all)]

mod error;
mod util;

use enigo::Axis;
use enigo::{Button, Coordinate, Direction, Enigo, Keyboard, Mouse, Settings};
use napi_derive::napi;

use crate::error::Result;
pub use crate::util::{JsAxis, JsButton, JsDirection};

#[napi(js_name = "Enigo")]
pub struct JsEnigo {
    enigo: Enigo,
}

#[napi]
impl JsEnigo {
    #[napi(factory)]
    pub fn create() -> Result<Self> {
        Ok(Self {
            enigo: Enigo::new(&Settings::default())?,
        })
    }

    #[napi]
    /// Get the location of the mouse in pixels.
    pub fn get_mouse_position(&self) -> Result<(i32, i32)> {
        self.enigo.location().map_err(Into::into)
    }

    #[napi]
    /// Move the mouse cursor to the specified x and y coordinates.
    ///
    /// The top left corner of your monitor screen is x=0 y=0.
    /// Move the cursor down the screen by increasing the y and to the right by increasing x coordinate.
    pub fn mouse_move(&mut self, x: i32, y: i32) -> Result<()> {
        self.enigo
            .move_mouse(x, y, Coordinate::Abs)
            .map_err(Into::into)
    }

    #[napi]
    /// Sends an individual mouse button event.
    ///
    /// You can use this for example to simulate a click of the left mouse key.
    /// Some of the buttons are specific to a platform.
    pub fn mouse_button(&mut self, button: JsButton, direction: JsDirection) -> Result<()> {
        self.enigo
            .button(Button::from(&button), Direction::from(&direction))
            .map_err(Into::into)
    }

    #[napi]
    /// Sends an individual mouse button event using Direction::Press.
    pub fn mouse_down(&mut self, button: JsButton) -> Result<()> {
        self.enigo
            .button(Button::from(&button), Direction::Press)
            .map_err(Into::into)
    }

    #[napi]
    /// Sends an individual mouse button event using Direction::Release.
    pub fn mouse_up(&mut self, button: JsButton) -> Result<()> {
        self.enigo
            .button(Button::from(&button), Direction::Release)
            .map_err(Into::into)
    }

    #[napi]
    /// Sends an individual mouse button event using Direction::Click (press then release).
    pub fn mouse_click(&mut self, button: JsButton) -> Result<()> {
        self.enigo
            .button(Button::from(&button), Direction::Click)
            .map_err(Into::into)
    }

    #[napi]
    /// Send a mouse scroll event
    ///
    /// ### Arguments
    /// - `length` - Number of 15° (click) rotations of the mouse wheel to scroll. How many lines will be scrolled depends on the current setting of the operating system.
    /// - `axis` - The axis to scroll on
    ///
    /// With `Axis::Vertical`, a positive length will result in scrolling down and negative ones up.
    /// With `Axis::Horizontal`, a positive length will result in scrolling to the right and negative ones to the left
    pub fn scroll(&mut self, length: i32, axis: JsAxis) -> Result<()> {
        self.enigo
            .scroll(length, Axis::from(&axis))
            .map_err(Into::into)
    }

    #[napi]
    /// Send a mouse scroll event on vertical axis.
    /// A positive length will result in scrolling down and negative ones up.
    pub fn scroll_vertical(&mut self, length: i32) -> Result<()> {
        self.enigo
            .scroll(length, Axis::Vertical)
            .map_err(Into::into)
    }

    #[napi]
    /// Send a mouse scroll event on horizontal axis.
    /// A positive length will result in scrolling to the right and negative ones to the left.
    pub fn scroll_horizontal(&mut self, length: i32) -> Result<()> {
        self.enigo
            .scroll(length, Axis::Horizontal)
            .map_err(Into::into)
    }

    #[napi]
    /// Sends an individual key event.
    pub fn key(&mut self, key: String, direction: JsDirection) -> Result<()> {
        let key = util::key_from_string(key)?;
        self.enigo
            .key(key, Direction::from(&direction))
            .map_err(Into::into)
    }

    #[napi]
    /// Sends an individual key event with Direction::Press.
    pub fn key_down(&mut self, key: String) -> Result<()> {
        let key = util::key_from_string(key)?;
        self.enigo.key(key, Direction::Press).map_err(Into::into)
    }

    #[napi]
    /// Sends an individual key event with Direction::Release.
    pub fn key_up(&mut self, key: String) -> Result<()> {
        let key = util::key_from_string(key)?;
        self.enigo.key(key, Direction::Release).map_err(Into::into)
    }

    #[napi]
    /// Sends an individual key event with Direction::Click (press then release).
    pub fn key_press(&mut self, key: String) -> Result<()> {
        let key = util::key_from_string(key)?;
        self.enigo.key(key, Direction::Click).map_err(Into::into)
    }

    #[napi]
    /// Sends a raw keycode.
    /// The keycode may or may not be mapped on the current layout.
    /// You have to make sure of that yourself.
    pub fn key_raw(&mut self, key: u16, direction: JsDirection) -> Result<()> {
        self.enigo
            .raw(key, Direction::from(&direction))
            .map_err(Into::into)
    }

    #[napi]
    /// Sends a raw keycode with Direction::Press.
    pub fn key_down_raw(&mut self, key: u16) -> Result<()> {
        self.enigo.raw(key, Direction::Press).map_err(Into::into)
    }

    #[napi]
    /// Sends a raw keycode with Direction::Release.
    pub fn key_up_raw(&mut self, key: u16) -> Result<()> {
        self.enigo.raw(key, Direction::Release).map_err(Into::into)
    }

    #[napi]
    /// Sends a raw keycode with Direction::Click (press then release).
    pub fn key_press_raw(&mut self, key: u16) -> Result<()> {
        self.enigo.raw(key, Direction::Click).map_err(Into::into)
    }

    #[napi]
    /// Enter the text Use a fast method to enter the text, if it is available.
    /// You can use unicode here like: ❤️. This works regardless of the current keyboard layout.
    ///
    /// You cannot use this function for entering shortcuts or something similar.
    /// For shortcuts, use the key() method instead.
    pub fn type_text(&mut self, text: String) -> Result<()> {
        self.enigo.text(&text).map_err(Into::into)
    }
}
