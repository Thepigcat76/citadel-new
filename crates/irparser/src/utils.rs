#[macro_export]
macro_rules! expect_tok {
    ($tok:expr,$pat:pat,$fail:expr) => {{
        if !matches!($tok, $pat) {
            $fail($tok);
            false
        } else {
            true
        }
    }};
}

#[macro_export]
macro_rules! parser_error {
    ($($arg:tt)+) => {{
        panic!("Parser Error: {}", format_args!($($arg)+));
    }};
}
