use crate::math::controllers::{BangBangController, Controller, PIDController};

#[test]
fn bang_bang() {
    let mut controller = BangBangController::new();

    controller.set_set_point(0.3);

    assert_eq!(controller.calculate(0.2, 0.0), 1.0);
}

#[test]
fn pid() {
    let mut controller = PIDController::new(0.1, 0.2, 0.3);

    controller.set_set_point(0.3);

    assert_eq!(controller.calculate(0.2, 20), 0.21150000000000002);
}
