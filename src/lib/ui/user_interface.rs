use crate::result::{OsResult, GenericOsError};
use crate::serial_println;
use alloc::boxed::Box;
use alloc::format;
use alloc::vec::Vec;
use alloc::string::String;
use pc_keyboard::{DecodedKey, Keyboard, layouts, ScancodeSet1, HandleControl};
use crate::task::executor::Executor;
use crate::task::Task;
use crate::drivers::keyboard::ps2::Ps2Keyboard;
use crate::drivers::display::vga::VgaDisplayDriver;
use futures_util::stream::{Stream, StreamExt};
use futures_util::task::AtomicWaker;
use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use core::pin::Pin;
use core::task::{Context, Poll};

#[derive(Debug, Copy, Clone)]
struct Content<'a> {
    id: &'a str,
    label: &'a str,
    can_close: bool,
    content_buffer: Vec<String>
}

impl<'a> Content<'a> {
    pub fn new(id: &'a str, label: &'a str, can_close: bool) -> Content<'a> {
        Content {
            id,
            label,
            can_close,
            content_buffer: Vec::new()
        }
    }

    pub fn draw(&mut self, display: &'a mut VgaDisplayDriver, &bounds) {

    }
}

#[derive(Debug, Copy, Clone)]
pub struct Clock {}

impl Clock {
    pub fn new() -> Clock {
        Clock {}
    }
}

const MAX_CONTENT_TABS: usize = 8;

#[derive(Debug, Copy, Clone)]
struct ContentContainer<'a> {
    pub panes: [Option<Content<'a>>; MAX_CONTENT_TABS],
    pub focused_pane: Option<usize>,
    pub next_free_index: Option<usize>,
}

impl<'a> ContentContainer<'a> {
    pub fn new() -> ContentContainer<'a> {
        ContentContainer {
            panes: [None; MAX_CONTENT_TABS],
            focused_pane: None,
            next_free_index: Some(0),
        }
    }

    pub fn app_tab(&mut self, content: Content<'a>) -> OsResult<()> {
        match self.next_free_index {
            Some(index) => {
                self.panes[index] = Some(content);
                if index == MAX_CONTENT_TABS {
                    self.next_free_index = None;
                } else {
                    self.next_free_index = Some(index + 1);
                }
                Ok(())
            }
            None => Err(Box::new(GenericOsError::new(
                format!("{:?}", self),
                "You're using the maximum number of tabs possible",
                "No free tab slots left",
            )))
        }
    }

    pub fn close_tab(&mut self, id: &str) -> OsResult<()> {
        let found = self.panes.iter().enumerate().find(|(_, pane)| {
            match pane {
                Some(pane) => pane.id == id,
                None => false
            }
        });
        if let Some((index, pane)) = found {
            if let Some(pane) = pane {
                // TODO: Cleanup pane
                self.panes[index] = None;
                return Ok(());
            }
        }
        Err(Box::new(GenericOsError::new(
            format!("id={}, self={:?}", id, self),
            "Can't find tab to close",
            "Invalid tab ID, or tab not open",
        )))
    }
}

#[derive(Debug)]
pub struct UserInterface<'a> {
    display: &'a mut VgaDisplayDriver<'a>,
    clock: Clock,
    content: ContentContainer<'a>,
}

impl<'a> UserInterface<'a> {
    pub fn new(display: &'a mut VgaDisplayDriver<'a>) -> OsResult<UserInterface> {
        let mut ui = UserInterface {
            display,
            clock: Clock::new(),
            content: ContentContainer::new(),
        };

        ui.add_tab("LOGS", "Logs", false)?;

        Ok(ui)
    }

    fn _draw(&mut self) {
        let mut display = self.display;

    }

    pub fn add_tab(&mut self, id: &'a str, label: &'a str, can_close: bool) -> OsResult<()> {
        let content = Content::new(id, label, can_close);
        self.content.app_tab(content)
    }

    pub fn inject_key_press(&self, key: DecodedKey) {
        match key {
            DecodedKey::Unicode(character) => serial_println!("{}", character),
            DecodedKey::RawKey(key) => serial_println!("{:?}", key),
        }
    }
}

impl<'a> UserInterface<'a> {
    pub async fn run(&mut self) {
        let mut keyboard_driver = Ps2Keyboard::new();
        let mut keyboard = Keyboard::new(layouts::Uk105Key, ScancodeSet1, HandleControl::Ignore);
        while let Some(scancode) = keyboard_driver.next().await {
            if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
                if let Some(key) = keyboard.process_keyevent(key_event) {
                    self.inject_key_press(key)
                }
            }
        }
    }
}
