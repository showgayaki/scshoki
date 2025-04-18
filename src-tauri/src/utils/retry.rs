pub fn retry<F, T, E>(mut f: F, max_attempts: u8, delay_ms: u64) -> Result<T, E>
where
    F: FnMut() -> Result<T, E>,
{
    for attempt in 0..max_attempts {
        match f() {
            Ok(value) => return Ok(value),
            Err(_) if attempt + 1 < max_attempts => {
                std::thread::sleep(std::time::Duration::from_millis(delay_ms));
            }
            Err(e) => return Err(e),
        }
    }
    unreachable!()
}
