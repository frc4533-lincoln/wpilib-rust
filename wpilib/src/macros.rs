
#[macro_export]
macro_rules! if_sim {
    ($($t:tt)*) => {
        #[cfg(feature = "simulation")]
        $($t)*
    }
}

#[macro_export]
macro_rules! if_real {
    ($($t:tt)*) => {
        #[cfg(not(feature = "simulation"))]
        $($t)*
    }
}