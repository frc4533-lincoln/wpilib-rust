# P I D Control
A PID controller is a controller that calculates an error value as the difference between a measured input and a desired
set point. The controller attempts to minimize the error by adjusting the process control inputs. The PID controller 
calculation (algorithm) involves three separate constant parameters, and is accordingly sometimes called three-term 
control: the proportional, the integral and derivative values, denoted P, I, and D. Heuristically, these values can be 
interpreted in terms of time: P depends on the present error, I on the accumulation of past errors, and D is a 
prediction of future errors, based on current rate of change. The weighted sum of these three actions is used to adjust 
the process via a control element such as the position of a wheel, speed of a motor, or velocity of a drivetrain. 

## Implementation
```rust
use wpilib::math::controllers::pid::PIDController;

let mut controller = PIDController::new(0.1, 0.01, 0.0);

controller.set_limits(-1.0, 1.0, -1.0, 1.0);

controller.set_set_point(0.32);

let output = controller.calculate(0.5, 20);
```

## Tuning
For any PID controller, the exact impact of the three terms (proportional, integral, and derivative) must be assessed 
for each implementation. Each term must be multiplied by a constant which is generally only determined by 
experimentation with the system at hand to determine the best response. Constant choice must balance several factors:

Settling time - For many choices of constants, the system will oscillate one or more times around the target. If the 
oscillations stop quickly, this is usually not an issue, but sustained oscillation is seldom intended.
Overshoot - PID controllers will usually overshoot the target slightly and come back to the desired value. While such 
behavior is often mechanically useful to remove gear or chain backlash, too much overshoot will lead to oscillations.
Steady-state error - Some small offset from the target will usually remain due to a variety of possible reasons.
Rise time - The rate at which the controller reaches the target must be carefully monitored. Setting a hard limit on the
output rate of change is often acceptable, but some constant choices may limit it implicitly.
Generally speaking, you want to first tune proportion (P), then derivative (D), and then finally integral (I). Some of 
these values might not be used if the values that come before it make the controller smooth enough. Most of the time 
spent coding PID will most likely not be spent on the algorithm itself, but rather refining these constants.

*~ Tuning section from* [BLRS-Wiki](https://github.com/purduesigbots/BLRS-Wiki/blob/master/software/control-algorithms/pid-controller.md)