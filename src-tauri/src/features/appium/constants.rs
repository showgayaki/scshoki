use std::sync::LazyLock;
use std::sync::Mutex;
use tokio_util::sync::CancellationToken;

pub static CANCEL_TOKEN: LazyLock<Mutex<Option<CancellationToken>>> =
    LazyLock::new(|| Mutex::new(None));
