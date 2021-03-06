//! # twilight-embed-builder
//!
//! `twilight-embed-builder` is a set of builder for the [`twilight-rs`]
//! ecosystem to create a message embed, useful when creating or updating
//! messages.
//!
//! ## Installation
//!
//! Add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! twilight-embed-builder = { branch = "trunk", git = "https://github.com/twilight-rs/twilight" }
//! ```
//!
//! ## Examples
//!
//! Build a simple embed:
//!
//! ```rust,no_run
//! use twilight_embed_builder::{EmbedBuilder, EmbedFieldBuilder};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let embed = EmbedBuilder::new()
//!     .description("Here's a list of reasons why Twilight is the best pony:")?
//!     .field(EmbedFieldBuilder::new("Wings", "She has wings.")?.inline())
//!     .field(EmbedFieldBuilder::new("Horn", "She can do magic, and she's really good at it.")?.inline())
//!     .build();
//! # Ok(()) }
//! ```
//!
//! Build an embed with an image:
//!
//! ```rust,no_run
//! use twilight_embed_builder::{EmbedBuilder, ImageSource};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let embed = EmbedBuilder::new()
//!     .description("Here's a cool image of Twilight Sparkle")?
//!     .image(ImageSource::attachment("bestpony.png")?)
//!     .build();
//!
//! # Ok(()) }
//! ```
//!
//! [`twilight-rs`]: https://github.com/twilight-rs/twilight
//! [the discord docs]: https://discord.com/developers/docs/resources/channel#create-message-using-attachments-within-embeds

#![deny(
    clippy::all,
    clippy::pedantic,
    future_incompatible,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    unsafe_code,
    unused,
    warnings
)]

pub mod author;
pub mod builder;
pub mod field;
pub mod footer;
pub mod image_source;

pub use self::{
    author::{EmbedAuthorBuilder, EmbedAuthorNameError},
    builder::{
        EmbedBuildError, EmbedBuilder, EmbedColorError, EmbedDescriptionError, EmbedTitleError,
    },
    field::{EmbedFieldBuilder, EmbedFieldError},
    footer::{EmbedFooterBuilder, EmbedFooterTextError},
    image_source::{ImageSource, ImageSourceAttachmentError, ImageSourceUrlError},
};
