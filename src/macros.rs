#[macro_export]
macro_rules! verbose {
    ($matches:expr, $($arg:tt)*) => {
        if $matches.verbose {
            println!($($arg)*);
        }
    }
}

#[macro_export]
macro_rules! err {
    ($($arg:tt)*) => {
        eprintln!("ERR: {}", format!($($arg)*));
    }
}
