// Answer 0

#[test]
fn test_poll_left_future() {
    use core::pin::Pin;
    use core::future::Future;
    use core::task::Context;
    use std::task::Poll;

    struct MockFuture;

    impl Future for MockFuture {
        type Output = i32;

        fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
            Poll::Ready(42)
        }
    }

    struct AnotherFuture;

    impl Future for AnotherFuture {
        type Output = i32;

        fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
            Poll::Ready(100)
        }
    }

    let future_left = MockFuture;
    let future_right = AnotherFuture;

    let either_instance = Either::Left(future_left);
    let pinned_either = Pin::new(&mut either_instance);
    let mut context = Context::from_waker(futures::task::noop_waker_ref());

    let _ = pinned_either.poll(&mut context);
}

#[test]
fn test_poll_right_future() {
    use core::pin::Pin;
    use core::future::Future;
    use core::task::Context;
    use std::task::Poll;

    struct AnotherFuture;

    impl Future for AnotherFuture {
        type Output = i32;

        fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
            Poll::Ready(100)
        }
    }

    struct MockFuture;

    impl Future for MockFuture {
        type Output = i32;

        fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
            Poll::Ready(42)
        }
    }

    let future_left = AnotherFuture;
    let future_right = MockFuture;

    let either_instance = Either::Right(future_right);
    let pinned_either = Pin::new(&mut either_instance);
    let mut context = Context::from_waker(futures::task::noop_waker_ref());

    let _ = pinned_either.poll(&mut context);
}

