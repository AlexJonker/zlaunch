//! AI integration module for zlaunch.
//!
//! Provides integration with Gemini API for answering user queries.

pub mod client;
pub mod item;

pub use client::GeminiClient;
pub use item::AiItem;
