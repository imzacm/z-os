mod cursor;
mod screen;

use screen::Screen;
use conquer_once::spin::OnceCell;
use spin::Mutex;

pub const WIDTH: usize = 80;
pub const HEIGHT: usize = 25;

static TTY: OnceCell<Mutex<Screen<WIDTH, HEIGHT>>> = OnceCell::uninit();

pub fn get_tty() -> &'static Mutex<Screen<WIDTH, HEIGHT>> {
    TTY.get_or_init(|| Mutex::new(Screen::new()))
}
