use alloc::boxed::Box;
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

pub mod keyboard;
pub mod simple_executor;

// Output = () because tasks are executed for side effects not returns
//dyn allows different types of Futures to be held in Task
// Pin means value cannot be moved in memory
pub struct Task {
    future: Pin<Box<dyn Future<Output = ()>>>,
}

// takes arbitrary future with output type () and pins it in memory
// Then wraps boxed future in Task and returns it
// 'static lifetime required because Task can live for arbitrary amount of time so future must be valid for that amount of time too
impl Task {
    pub fn new(future: impl Future<Output = ()> + 'static) -> Task {
        Task {
            future: Box::pin(future),
        }
    }

    // convert Pin<Box<T>> -> Pin<&mut T> to call poll
    fn poll(&mut self, context: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(context)
    }
}
