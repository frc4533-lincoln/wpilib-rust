# Bang Bang Control
A bang-bang controller is a controller that switches between two discrete outputs, such as full power and zero power. It
is useful for systems that can tolerate being switched on and off rapidly, such as a motor driving a flywheel.

## Real World Example
Older household thermostats are a good example of bang-bang control. When the temperature is below the set point, the 
heater is turned on. When the temperature is above the set point, the heater is turned off. This is a simple and 
effective way to control the temperature in a house.

## Implementation
```rust
use wpilib::math::controllers::bang_bang::BangBangController;

let mut controller = BangBangController::new();

controller.set_limits(-10.0, 100.0, -1.0, 1.0);

controller.set_tolerance(0.5);

controller.set_set_point(50.0);

let output = controller.calculate(45.0, 20.0);
```

In this scenario the controller would return 1.0, as the error is greater than the tolerance, given that the set point 
is 50.0 and the error is 5.0. If the error was -2.0, the controller would return -1.0 and if it was between -0.5 and
0.5, the controller would return 0.0. as it is within the tolerance zone.