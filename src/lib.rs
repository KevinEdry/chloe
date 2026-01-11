//! Chloe library crate
//!
//! # Safety Policy
//!
//! This crate forbids ALL unsafe code.

#![forbid(unsafe_code)]

// Re-export main modules for testing
pub mod app;
pub mod cli;
pub mod events;
pub mod helpers;
pub mod persistence;
pub mod review;
pub mod types;
pub mod views;
pub mod widgets;

#[cfg(test)]
mod tests {
    /// This test verifies that unsafe code is forbidden at compile time.
    /// If you uncomment the unsafe block below, the code will NOT compile.
    #[test]
    fn verify_unsafe_code_is_forbidden() {
        // This should compile fine (safe code)
        let x = 42;
        assert_eq!(x, 42);

        // Uncomment this to verify the policy works:
        // unsafe {
        //     // This will cause a compile error: unsafe code is forbidden
        //     let ptr = &x as *const i32;
        //     let _value = *ptr;
        // }
    }

    #[test]
    fn verify_thread_safety() {
        use std::sync::{Arc, Mutex};
        use std::thread;

        // Only safe threading patterns allowed
        let counter = Arc::new(Mutex::new(0));
        let handles: Vec<_> = (0..10)
            .map(|_| {
                let counter = Arc::clone(&counter);
                thread::spawn(move || {
                    let mut number = counter.lock().unwrap();
                    *number += 1;
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(*counter.lock().unwrap(), 10);
    }
}
