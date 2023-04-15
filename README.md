# sb-rotary-encoder

`no_std` Rust crate for processing rotary encoder signals.

## Features

- Platform independent with minimum dependencies.
- Pulse division for handling of encoders with detents.
- Velocity calculation via timestamps for optional acceleration.
- Integer processing for maximum performance.
- Automatic realignment for encoders with detents (see explanation below).

## Encoders with Detents

There are two types of rotary encoders available on the market: ones with detents and ones without.
Having detents gives the user an experience of a stepped control, but has some implications that need to be taken care of:

### Number of Pulses

The resolution of the detents is often lower than the resolution of pulses generated. So several pulses have to occur before a single step is reached and a corresponding event has to be emitted. In practice, these encoder models typically use 2 or 4 pulses per detent.

### Switching Point

For best user experience, the encoder should emit an event when the next step is hit, i.e. when the number of pulses per detent has been reached. Unfortunately, detection of pulses can sometimes be missed either to turning the encoder very fast or due to natural wearout of the component itself. In this case, the noticable switching point will shift, something that is noticeable when doing further movements.

## Usage Example

A processing interval of 1ms is recommended for most use cases and fits into the common 1kHz frequency setting of an RTOS or a SysTick counter.

```rust
// Number of pulses required for one step. 4 is a typical value for encoders with detents.
const PULSE_DIVIDER: i32 = 4;

// Update frequency in Hz, used for velocity calculation
const UPDATE_FREQUENCY: i32 = 1000;

// Create a new instance of an encoder.
let mut encoder = sb_rotary_encoder::RotaryEncoder::new();

// Some dummy input signals to be replaced by the real pin states.
let input_a = true;
let input_b = false;

// Some dummy tick value, can also be `None` if you don't want to use the velocity feature.
let tick = Some(42);

// Process the input and get an event if a step was recognized.
if let Some(event) = encoder.update(input_a, input_b, tick, PULSE_DIVIDER) {
    println!("{:?}", event);

    // If timestamps were used, the velocity can be calculated.
    // This is done as an additional step because it requires some calculations
    // which may not be always needed.
    if let Some(velocity) = event.velocity(UPDATE_FREQUENCY) {
        println!("{:?}", velocity);

        // The velocity allows to calculate a dynamic step value to accelerate the encoder
        // when moved quickly.
        let acceleration = velocity >> 4;
        let step = event.step() + (event.step() * acceleration);
    }
}
```

## Tests

Run `cargo test` for the unit tests.

## License

Published under the MIT license. Any contribution to this project must be provided under the same license conditions.

Author: Oliver Rockstedt <info@sourcebox.de>
