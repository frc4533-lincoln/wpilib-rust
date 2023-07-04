//setup some clippy lints
#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![warn(
    missing_copy_implementations,
    single_use_lifetimes,
    variant_size_differences,
    clippy::many_single_char_names,
    clippy::get_unwrap,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::panicking_unwrap,
    arithmetic_overflow,
    missing_debug_implementations
)]
#![forbid(
    clippy::missing_safety_doc,
    while_true,
    absolute_paths_not_starting_with_crate,
    bare_trait_objects,
    semicolon_in_expressions_from_macros,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_import_braces,
    unused_lifetimes,
    redundant_semicolons
)]

extern crate wpilib_macros;

use robots::UserRobot;

#[cfg(feature = "command")]
pub mod command;
pub mod math;
pub mod robots;
#[macro_use]
pub mod macros;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventTypes {
    Init,
    Periodic,
    Overrun,
    End,
}

#[no_panic::no_panic]
pub fn wpilib_main(_robot: Box<dyn UserRobot>) {}
