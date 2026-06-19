pub fn error(line: usize, message: &str) {
    report(line, "", message);
}

fn report(line: usize, pos: &str, message: &str) {
    let err_text = format!("[line {line}] Error {pos}: {message}");
    eprintln!("{}", err_text);
}
