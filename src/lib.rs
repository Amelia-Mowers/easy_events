use std::any::Any;

pub trait Event{
    fn as_any(&self) -> &dyn Any;
}

#[macro_export]
macro_rules! implement_event {
    (
        $struct:ident
    ) => {
        impl Event for $struct {
            fn as_any(&self) -> &dyn Any{ self }
        }
    }
}

#[cfg(test)]
mod tests {
}
