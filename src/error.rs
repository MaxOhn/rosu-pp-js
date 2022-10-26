use std::{error::Error, fmt::Write};

pub trait ErrorExt {
    fn unwind(&self, cause: &str) -> String;
}

impl<T: Error> ErrorExt for T {
    fn unwind(&self, cause: &str) -> String {
        let mut e = self as &dyn Error;
        let mut content = format!("{cause}: {e}");

        while let Some(src) = e.source() {
            let _ = writeln!(content, "  - caused by: {src}");
            e = src;
        }

        content
    }
}
