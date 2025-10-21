use std::sync::atomic;

use super::TableOfContent;
use crate::content_manager::errors::StorageError;

pub const DEFAULT_WRITE_LOCK_ERROR_MESSAGE: &str = "Write operations are forbidden";

impl TableOfContent {
    pub fn is_write_locked(&self) -> bool {
        self.is_write_locked.load(atomic::Ordering::Relaxed)
    }

    pub fn is_read_only(&self) -> bool {
        self.read_only
    }

    pub fn get_lock_error_message(&self) -> Option<String> {
        self.lock_error_message.lock().clone()
    }

    /// Returns an error if the write lock is set
    pub fn check_write_lock(&self) -> Result<(), StorageError> {
        if self.is_write_locked.load(atomic::Ordering::Relaxed) {
            let error_message = if self.read_only {
                "Instance is running in read-only mode".to_string()
            } else {
                self.lock_error_message
                    .lock()
                    .clone()
                    .unwrap_or_else(|| DEFAULT_WRITE_LOCK_ERROR_MESSAGE.to_string())
            };
            return Err(StorageError::Forbidden { description: error_message });
        }
        Ok(())
    }

    pub fn set_locks(&self, is_write_locked: bool, error_message: Option<String>) {
        self.is_write_locked
            .store(is_write_locked, atomic::Ordering::Relaxed);
        *self.lock_error_message.lock() = error_message;
    }
}
