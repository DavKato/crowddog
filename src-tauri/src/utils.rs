use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub fn today() -> String {
    chrono::Local::now().format("%Y-%m-%d").to_string()
}

pub fn log_if_error<T, E: core::fmt::Debug>(res: Result<T, E>) -> Result<T, String> {
    match res {
        Ok(data) => Ok(data),
        Err(e) => {
            eprintln!("{:?}", e);
            Err(format!("{:?}", e))
        }
    }
}

#[derive(Clone, Debug)]
pub struct CancellationToken {
    cancelled: Arc<AtomicBool>,
}

impl CancellationToken {
    #[inline]
    pub fn should_cancel(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }
}

#[derive(Clone, Debug)]
pub struct Canceller {
    cancelled: Arc<AtomicBool>,
}

impl Canceller {
    #[inline]
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }
}

#[inline]
pub fn cancellation_token() -> (Canceller, CancellationToken) {
    let cancelled = Arc::new(AtomicBool::new(false));
    (
        Canceller {
            cancelled: Arc::clone(&cancelled),
        },
        CancellationToken { cancelled },
    )
}
