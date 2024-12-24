#![no_std] // 標準ライブラリを使わない

// allocクレートを利用
extern crate alloc;

pub mod error;
pub mod http;
pub mod renderer;
pub mod url;