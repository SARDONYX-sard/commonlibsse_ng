//! Skyrim console log print

/// Prints to the in-game console log (Skyrim) without a newline.
///
/// This is analogous to [`print!`](https://doc.rust-lang.org/std/macro.print.html),
/// but targets the Skyrim in-game console log instead of stdout.
///
/// # Examples
///
/// ```
/// use commonlibsse_ng::console_print;
///
/// console_print!("Hello, {}!", "Dragonborn");
/// ```
#[macro_export]
macro_rules! console_print {
    ($($arg:tt)*) => {{
        use ::core::fmt::Write as _;
        $crate::re::ConsoleLog::print_fmt(format_args!($($arg)*));
    }};
}

/// Prints to the in-game console log (Skyrim) with a newline.
///
/// This is analogous to [`println!`](https://doc.rust-lang.org/std/macro.println.html),
/// but targets the Skyrim in-game console log instead of stdout.
///
/// # Examples
///
/// ```no_run
/// use commonlibsse_ng::console_println;
///
/// console_println!("Level up! You are now level {}", 42);
/// console_println!(); // just a newline
/// ```
///
/// # Allocate Heap Memory
/// This uses the [`Write`] trait internally, and since [`str`] is a [`CString`], heap allocation occurs each time it is used.
///
/// If the string is fixed at compile time, it is better to use `c""` and `ConsoleLog::print` method to save memory and speed up the process.
#[macro_export]
macro_rules! console_println {
    () => {unsafe {
        $crate::re::ConsoleLog::ConsoleLog::get_singleton_mut()
            .map(|console| console.print(c"\n".as_ptr()));
    }};
    ($($arg:tt)*) => {{
        $crate::re::ConsoleLog::print_fmt(format_args!("{}\n", format_args!($($arg)*)));
    }};
}
