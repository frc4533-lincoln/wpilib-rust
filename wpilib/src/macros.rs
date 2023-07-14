#[macro_export]
macro_rules! if_sim {
    ($($t:tt)*) => {
        #[cfg(feature = "simulation")]
        $($t)*
    }
}

#[macro_export]
macro_rules! if_not_athena {
    ($($t:tt)*) => {
        #[cfg(feature = "other-target")]
        $($t)*
    }
}

#[macro_export]
macro_rules! if_athena {
    ($($t:tt)*) => {
        #[cfg(feature = "athena")]
        $($t)*
    }
}

#[macro_export]
///allows macros to use the wpilib namespace as if they're being called by users who have imported wpilib
macro_rules! crate_namespace {
    () => {
        use crate as wpilib;
    };
}
