extern crate x11_clipboard;

use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::os::unix::fs::FileTypeExt;
use std::os::unix::net::UnixStream;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use x11_clipboard::Clipboard;

fn main() {
    let prevstr = Arc::new(Mutex::new(String::new()));
    if fs::metadata(std::env::args().nth(1).unwrap())
        .unwrap()
        .file_type()
        .is_char_device()
    {
        // guest
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(std::env::args().nth(1).unwrap())
            .unwrap();

        // reader
        let mut rdfile = file.try_clone().unwrap();
        let rdprev = Arc::clone(&prevstr);
        thread::spawn(move || {
            let clipboard = Clipboard::new().unwrap();
            let mut buffer = [0; 65535];
            loop {
                let n = rdfile.read(&mut buffer[..]).unwrap();
                if n == 0 {
                    thread::sleep(Duration::from_secs(1));
                    continue;
                }
                let str = std::str::from_utf8(&buffer[..n]).unwrap().to_string();
                let mut prev = rdprev.lock().unwrap();
                if str != *prev {
                    *prev = str.to_owned().to_string();
                    clipboard
                        .store(
                            clipboard.getter.atoms.primary,
                            clipboard.getter.atoms.utf8_string,
                            str,
                        )
                        .unwrap();
                }
            }
        });

        //writer
        let clipboard = Clipboard::new().unwrap();
        let mut wrfile = file.try_clone().unwrap();
        loop {
            if let Ok(str) = clipboard.load_wait(
                clipboard.getter.atoms.primary,
                clipboard.getter.atoms.utf8_string,
                clipboard.getter.atoms.property,
            ) {
                let str = String::from_utf8_lossy(&str);
                let mut prev = prevstr.lock().unwrap();
                if str != *prev {
                    *prev = str.to_owned().to_string();
                    wrfile.write_all(str.as_bytes()).unwrap();
                }
            }
        }
    } else {
        // host
        let file = UnixStream::connect(std::env::args().nth(1).unwrap()).unwrap();

        // reader
        let mut rdfile = file.try_clone().unwrap();
        let rdprev = Arc::clone(&prevstr);
        thread::spawn(move || {
            let clipboard = Clipboard::new().unwrap();
            let mut buffer = [0; 65535];
            loop {
                let n = rdfile.read(&mut buffer[..]).unwrap();
                if n == 0 {
                    thread::sleep(Duration::from_secs(1));
                    continue;
                }
                let str = std::str::from_utf8(&buffer[..n]).unwrap().to_string();
                let mut prev = rdprev.lock().unwrap();
                if str != *prev {
                    *prev = str.to_owned().to_string();
                    clipboard
                        .store(
                            clipboard.getter.atoms.primary,
                            clipboard.getter.atoms.utf8_string,
                            str,
                        )
                        .unwrap();
                }
            }
        });

        //writer
        let clipboard = Clipboard::new().unwrap();
        let mut wrfile = file.try_clone().unwrap();
        loop {
            if let Ok(str) = clipboard.load_wait(
                clipboard.getter.atoms.primary,
                clipboard.getter.atoms.utf8_string,
                clipboard.getter.atoms.property,
            ) {
                let str = String::from_utf8_lossy(&str);
                let mut prev = prevstr.lock().unwrap();
                if str != *prev {
                    *prev = str.to_owned().to_string();
                    wrfile.write_all(str.as_bytes()).unwrap();
                }
            }
        }
    }
}
