// Answer 0

#[test]
fn test_poll_with_right_future() {
    struct RightFuture;
    impl Future for RightFuture {
        type Output = i32;
        fn poll(self: Pin<&mut Self>, _: &mut core::task::Context<'_>) -> core::task::Poll<Self::Output> {
            core::task::Poll::Ready(42)
        }
    }

    struct LeftFuture;
    impl Future for LeftFuture {
        type Output = i32;
        fn poll(self: Pin<&mut Self>, _: &mut core::task::Context<'_>) -> core::task::Poll<Self::Output> {
            core::task::Poll::Pending
        }
    }

    let right_future = RightFuture;
    let either = Either::Right(right_future);
    
    let mut cx = core::task::Context::from_waker(futures::task::noop_waker_ref());
    let mut pinned_either = Pin::new(&mut either);
    let result = pinned_either.poll(&mut cx);
}

#[test]
fn test_poll_with_left_future() {
    struct RightFuture;
    impl Future for RightFuture {
        type Output = i32;
        fn poll(self: Pin<&mut Self>, _: &mut core::task::Context<'_>) -> core::task::Poll<Self::Output> {
            core::task::Poll::Pending
        }
    }

    struct LeftFuture;
    impl Future for LeftFuture {
        type Output = i32;
        fn poll(self: Pin<&mut Self>, _: &mut core::task::Context<'_>) -> core::task::Poll<Self::Output> {
            core::task::Poll::Ready(36)
        }
    }

    let left_future = LeftFuture;
    let either = Either::Left(left_future);
    
    let mut cx = core::task::Context::from_waker(futures::task::noop_waker_ref());
    let mut pinned_either = Pin::new(&mut either);
    let result = pinned_either.poll(&mut cx);
}

#[test]
fn test_poll_with_both_futures_ready() {
    struct BothReadyFuture;
    impl Future for BothReadyFuture {
        type Output = i32;
        fn poll(self: Pin<&mut Self>, _: &mut core::task::Context<'_>) -> core::task::Poll<Self::Output> {
            core::task::Poll::Ready(100)
        }
    }

    let either = Either::Right(BothReadyFuture);
    
    let mut cx = core::task::Context::from_waker(futures::task::noop_waker_ref());
    let mut pinned_either = Pin::new(&mut either);
    let result = pinned_either.poll(&mut cx);
}

