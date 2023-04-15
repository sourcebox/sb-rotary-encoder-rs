//! Unit tests.

use super::*;

/// Clockwise movement.
#[test]
fn clockwise() {
    const PULSE_DIVIDER: i32 = 1;

    let mut encoder = RotaryEncoder::new();

    let result = encoder.update(false, false, None, PULSE_DIVIDER);
    assert_eq!(result, None);

    let result = encoder.update(true, false, None, PULSE_DIVIDER);
    assert_eq!(
        result,
        Some(RotaryEncoderEvent {
            value: 1,
            direction: Direction::Clockwise,
            timedelta: None
        })
    );

    let result = encoder.update(true, true, None, PULSE_DIVIDER);
    assert_eq!(
        result,
        Some(RotaryEncoderEvent {
            value: 2,
            direction: Direction::Clockwise,
            timedelta: None
        })
    );

    let result = encoder.update(false, true, None, PULSE_DIVIDER);
    assert_eq!(
        result,
        Some(RotaryEncoderEvent {
            value: 3,
            direction: Direction::Clockwise,
            timedelta: None
        })
    );

    let result = encoder.update(false, false, None, PULSE_DIVIDER);
    assert_eq!(
        result,
        Some(RotaryEncoderEvent {
            value: 4,
            direction: Direction::Clockwise,
            timedelta: None
        })
    );
}

/// Counter-clockwise movement.
#[test]
fn counter_clockwise() {
    const PULSE_DIVIDER: i32 = 1;

    let mut encoder = RotaryEncoder::new();

    let result = encoder.update(false, false, None, PULSE_DIVIDER);
    assert_eq!(result, None);

    let result = encoder.update(false, true, None, PULSE_DIVIDER);
    assert_eq!(
        result,
        Some(RotaryEncoderEvent {
            value: -1,
            direction: Direction::CounterClockwise,
            timedelta: None
        })
    );

    let result = encoder.update(true, true, None, PULSE_DIVIDER);
    assert_eq!(
        result,
        Some(RotaryEncoderEvent {
            value: -2,
            direction: Direction::CounterClockwise,
            timedelta: None
        })
    );

    let result = encoder.update(true, false, None, PULSE_DIVIDER);
    assert_eq!(
        result,
        Some(RotaryEncoderEvent {
            value: -3,
            direction: Direction::CounterClockwise,
            timedelta: None
        })
    );

    let result = encoder.update(false, false, None, PULSE_DIVIDER);
    assert_eq!(
        result,
        Some(RotaryEncoderEvent {
            value: -4,
            direction: Direction::CounterClockwise,
            timedelta: None
        })
    );
}

/// Clockwise movement with pulse divider of 4.
#[test]
fn clockwise_div4() {
    const PULSE_DIVIDER: i32 = 4;

    let mut encoder = RotaryEncoder::new();

    let result = encoder.update(false, false, None, PULSE_DIVIDER);
    assert_eq!(result, None);

    let result = encoder.update(true, false, None, PULSE_DIVIDER);
    assert_eq!(result, None);

    let result = encoder.update(true, true, None, PULSE_DIVIDER);
    assert_eq!(result, None);

    let result = encoder.update(false, true, None, PULSE_DIVIDER);
    assert_eq!(result, None);

    let result = encoder.update(false, false, None, PULSE_DIVIDER);
    assert_eq!(
        result,
        Some(RotaryEncoderEvent {
            value: 1,
            direction: Direction::Clockwise,
            timedelta: None
        })
    );
}

/// Counter-clockwise movement with pulse divider of 4.
#[test]
fn counter_clockwise_div4() {
    const PULSE_DIVIDER: i32 = 4;

    let mut encoder = RotaryEncoder::new();

    let result = encoder.update(false, false, None, PULSE_DIVIDER);
    assert_eq!(result, None);

    let result = encoder.update(false, true, None, PULSE_DIVIDER);
    assert_eq!(result, None);

    let result = encoder.update(true, true, None, PULSE_DIVIDER);
    assert_eq!(result, None);

    let result = encoder.update(true, false, None, PULSE_DIVIDER);
    assert_eq!(result, None);

    let result = encoder.update(false, false, None, PULSE_DIVIDER);
    assert_eq!(
        result,
        Some(RotaryEncoderEvent {
            value: -1,
            direction: Direction::CounterClockwise,
            timedelta: None
        })
    );
}
