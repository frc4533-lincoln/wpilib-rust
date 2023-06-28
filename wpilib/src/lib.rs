//setup some clippy lints
#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![warn(
    unsafe_code,
    missing_copy_implementations,
    variant_size_differences,
    clippy::many_single_char_names,
    clippy::get_unwrap,
    clippy::unwrap_in_result,
    clippy::unwrap_used)]
#![forbid(
    clippy::missing_safety_doc,
    while_true,
    absolute_paths_not_starting_with_crate,
    bare_trait_objects,
    semicolon_in_expressions_from_macros,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    redundant_semicolons
    )]

extern crate wpilib_macros;

use robots::UserRobot;

#[cfg(feature = "command")]
pub mod command;
pub mod math;
pub mod robots;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventTypes {
    Init,
    Periodic,
    End,
}

pub fn wpilib_main(_robot: Box<dyn UserRobot>) {}
