#![warn(
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction
)]
#![allow(
    dead_code,
    clippy::blanket_clippy_restriction_lints,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::default_numeric_fallback,
    clippy::else_if_without_else,
    clippy::exhaustive_enums,
    clippy::exhaustive_structs,
    clippy::float_arithmetic,
    clippy::implicit_return,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::integer_division,
    clippy::missing_docs_in_private_items,
    clippy::missing_panics_doc,
    clippy::mod_module_files,
    clippy::module_name_repetitions,
    clippy::modulo_arithmetic,
    clippy::multiple_crate_versions,
    clippy::multiple_inherent_impl,
    clippy::panic_in_result_fn,
    clippy::panic,
    clippy::print_stdout,
    clippy::pub_use,
    clippy::shadow_reuse,
    clippy::shadow_unrelated,
    clippy::unreachable,
    clippy::unwrap_used
)]

mod audio;
mod game;
mod generation;
mod loading;
mod menu;
pub mod settings;

pub use game::GamePlugin;
