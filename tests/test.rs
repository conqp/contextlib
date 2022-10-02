use contextlib::Contextmanager;

mod counter;
use counter::Counter;

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

    timer.with(|this| {
        assert!(this.start().is_some());
        println!("Start: {:?}, end: {:?}", this.start(), this.end());
        assert!(this.end().is_none());
        assert!(this.duration().is_none());
    });
    assert!(timer.end().is_some());
    assert!(timer.duration().is_some());
    println!("Duration: {:?}", timer.duration());
}
