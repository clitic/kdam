#[derive(Debug)]
pub struct BarInternal {
    pub started: bool,
    pub elapsed_time: f64,
    pub its_per: f64,
    pub bar_length: i16,
    pub user_ncols: i16,
    pub charset: String,
    pub charset_len: u64,
    pub timer: std::time::Instant,
    pub force_refresh: bool,
}

impl Default for BarInternal {
    fn default() -> BarInternal {
        BarInternal {
            started: false,
            elapsed_time: 0.0,
            its_per: 0.0,
            bar_length: 0,
            user_ncols: -1,
            charset: crate::styles::TQDMCHARSET.join(""),
            charset_len: 8,
            timer: std::time::Instant::now(),
            force_refresh: false,
        }
    }
}
