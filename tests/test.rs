use contextlib::{Contextmanager,Suppress};

mod counter;
use counter::Counter;

mod errors;
use errors::ErrorsToSuppress;

mod timer;
use timer::Timer;

#[test]
fn test_basic_ctm() {
    let mut counter = Counter::new();

    counter.with(|this| {
        assert_eq!(1, this.calls());
    });
    assert_eq!(0, counter.calls());
}

#[test]
fn test_timer() {
    let mut timer = Timer::new();

    let duration = timer.with(|this| {
        assert!(this.start().is_some());
        println!("Start: {:?}, end: {:?}", this.start(), this.end());
        assert!(this.end().is_none());
        assert!(this.duration().is_none());
    });
    assert!(timer.end().is_some());
    assert!(duration.is_some());
    println!("Duration: {:?}", duration);
}

#[test]
fn test_suppress() {
    let mut suppress = Suppress::new([ErrorsToSuppress::NotAFizzbuzz]);

    let suppressed_error = suppress.with(|_| {
        if false {
            Ok("Infer type &str")
        } else {
            Err(ErrorsToSuppress::NotAFizzbuzz)
        }
    });

    assert!(suppressed_error.is_ok());
    assert_eq!(None, suppressed_error.unwrap());

    let suppressed_error = suppress.with(|_| {
        if false {
            Ok("Infer type &str")
        } else {
            Err(ErrorsToSuppress::FoobarOutOfRange)
        }
    });

    assert!(suppressed_error.is_err());
    assert_eq!(ErrorsToSuppress::FoobarOutOfRange, suppressed_error.err().unwrap());

    let suppressed_error = suppress.with(|_| {
        if true {
            Ok("That worked.")
        } else {
            Err(ErrorsToSuppress::FoobarOutOfRange)   // Infer error type
        }
    });

    assert!(suppressed_error.is_ok());
    assert!(suppressed_error.unwrap().is_some());
    assert_eq!("That worked.", suppressed_error.unwrap().unwrap());
}