
//create a macro that can decorate a struct that has Subsystem as a trait
//the macro will create a static mutex that will hold the struct using lazy and create of the Subsystem trait
// #[macro_export]
// macro_rules! subsystem {
//     //make the name ident uppercase
//     (name: $name:ident, upper: $upper:ident) => {
//         static $upper: once_cell::sync::Lazy<parking_lot::Mutex<Box<$name>>> = once_cell::sync::Lazy::new(|| parking_lot::Mutex::new(Box::new($name::create())));
//     };
// }

