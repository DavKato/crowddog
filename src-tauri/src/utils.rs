use crate::api::ReqError;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub fn today() -> String {
    chrono::Local::now().format("%Y-%m-%d").to_string()
}

pub fn log_if_error<T>(res: Result<T, reqwest::Error>) -> Result<T, ReqError> {
    match res {
        Ok(data) => Ok(data),
        Err(e) => {
            eprintln!("{:?}", e);
            Err(ReqError::from(&e))
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

pub fn encrypt(str: &str) -> String {
    let vec: Vec<u8> = str.bytes().map(|x| x.wrapping_add(1)).collect();
    String::from_utf8(vec).unwrap()
}

pub fn decrypt(str: &str) -> String {
    let vec: Vec<u8> = str.bytes().map(|x| x.wrapping_sub(1)).collect();
    String::from_utf8(vec).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_str(str: &str) {
        let encoded = encrypt(str);
        let decoded = decrypt(encoded.as_str());
        assert_eq!(str, decoded);
    }

    #[test]
    fn encode_decode_utf8() {
        test_str("password");
        test_str("Hy3u45!ZRdLz*Kp#2B^DyRMs$irXCZtD");
        test_str("🎉emoji_is_not_supported!!");
    }
}
