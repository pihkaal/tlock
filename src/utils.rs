use std::time;

pub fn format_duration(duration: time::Duration) -> String {
    let seconds = duration.as_secs();
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;

    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

#[macro_export]
macro_rules! eprintln_quit {
    ($($arg:tt)*) => ({
        use std::io::Write;
        write!(&mut std::io::stderr(), "ERROR: ").unwrap();
        writeln!(&mut std::io::stderr(), $($arg)*).unwrap();
        std::process::exit(1)
    })
}
