use std::any::Any;

pub trait Event{
    fn as_any(&self) -> &dyn Any;
    fn event_id() -> String;
    fn event_id_this(&self) -> String;
}

#[macro_export]
macro_rules! implement_event {
    (
        $struct:ident,
        $event_id:expr
    ) => {
        impl Event for $struct {
            fn as_any(&self) -> &dyn Any{ self }
            fn event_id() -> String{ String::from($event_id) }
            fn event_id_this(&self) -> String{ String::from($event_id) }
        }
    }
}

#[cfg(test)]
mod tests {
    struct TestEvent;
    use super::*;

    implement_event!(TestEvent, "test_event");

    #[test]
    fn id_test() {
        println!("{}", TestEvent::event_id());

        let ev = TestEvent{};
        println!("{}", ev.event_id_this());
    }
}
