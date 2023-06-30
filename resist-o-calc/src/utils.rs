pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// Converts an index into a float array to a concrete value,
/// where every loop of the array is 10x the value
pub fn float_to_val(data: &[f64], idx: usize) -> f64 {
    data[idx % data.len()] * (10_u32.pow((idx / data.len()) as u32)) as f64
}
