/// Only look for an Ok Result otherwise continue outer loop.
#[macro_export]
macro_rules! ok_or_continue {
    ($res:expr) => {
        match $res {
            Ok(v) => v,
            Err(_) => continue,
        }
    };
}
