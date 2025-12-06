#[macro_export]
macro_rules! time {
    ($e:expr) => {{
        let s = Instant::now();
        let v = $e;
        (v, s.elapsed())
    }};
}
