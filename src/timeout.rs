pub(crate) struct TimeoutInterrupt {
    start: std::time::Instant,
    timeout: u128,
}

impl TimeoutInterrupt {
    pub(crate) fn new_with_timeout(timeout: u128) -> Self {
        Self {
            start: std::time::Instant::now(),
            timeout,
        }
    }
}

impl fend_core::Interrupt for TimeoutInterrupt {
    fn should_interrupt(&self) -> bool {
        std::time::Instant::now()
            .duration_since(self.start)
            .as_millis()
            > self.timeout
    }
}