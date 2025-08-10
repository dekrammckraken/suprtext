// This file is part of suprtext
//
// suprcen is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// suprps is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::{
    io::Write,thread, time::Duration
};
use colored::{Colorize, CustomColor};

pub struct Suprtext {
    cursor_time: u64,
    new_line_time: u64,
    data: String,
    foreground: CustomColor
}

impl Suprtext {
    pub fn init(cursor_time: u64, new_line_time: u64, data: String, foreground:CustomColor) -> Self {
        Self {
            cursor_time: cursor_time,
            new_line_time: new_line_time,
            data: data,
            foreground: foreground
        }
    }
    pub fn flush(&self) {
        std::io::stdout().flush().expect("stdout_flush_err");
    }
    pub fn hide_curor(&self) {
        print!("\x1b[?25l");
        self.flush();
    }
    pub fn restore_cursor(&self) {
        print!("\x1b[?25h");
        self.flush();
    }

    pub fn sleep(&self, d: u64) {
        thread::sleep(Duration::from_millis(d));
    }
    pub fn clear(&self) {
        print!("\x1b[2J\x1b[H");
        self.flush();
    }

    pub fn is_new_line(&self, c:char) -> bool {
          c == '\n'
    }
    pub fn animate(&self) {
        for c in self.data.chars() {
            print!("{}", format!("{}", c).custom_color(self.foreground));
            self.flush();
            self.sleep(self.cursor_time);

            if self.is_new_line(c) {
                self.sleep(self.new_line_time);
            }
        }
    }
}
