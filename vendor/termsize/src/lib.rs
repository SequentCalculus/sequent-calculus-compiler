#![deny(missing_docs)]

//! Termsize is a tiny crate that provides a simple
//! interface for retrieving the current
//! [terminal interface](http://www.manpagez.com/man/4/tty/) size
//!
//! ```rust
//! extern crate termsize;
//!
//! termsize::get().map(|size| println!("rows {} cols {}", size.rows, size.cols));
//! ```

/// Container for number of rows and columns
#[derive(Debug)]
pub struct Size {
    /// number of rows
    pub rows: u16,
    /// number of columns
    pub cols: u16,
}

#[cfg(unix)]
#[path = "nix.rs"]
mod imp;

#[cfg(windows)]
#[path = "win.rs"]
mod imp;

#[cfg(not(any(unix, windows)))]
#[path = "other.rs"]
mod imp;

pub use imp::get;

#[cfg(test)]
mod tests {
    use super::get;
    #[test]
    fn test_get() {
        assert!(get().is_some())
    }
}
