use std::io::{self, Write};

use crate::HAD_ERROR;


pub fn error(line: u32, msg: &str) {

}


fn report(line: u32, loc: &str, msg: &str) {
    let stderr = io::stderr();
    let handle = stderr.lock();
    let mut writer = io::BufWriter::new(handle);

    let _ = writer.write_fmt(format_args!("[line {}] Error {}: {}\n", line, loc, msg));
    unsafe { HAD_ERROR = true; }
    return ();
}
