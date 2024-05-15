#[derive(Clone, Debug)]
pub(super) struct Counter {
    total: usize,
    forward: usize,
    backward: usize,
}

impl Counter {
    pub(super) fn new(total: usize) -> Self {
        Self {
            total,
            forward: 0,
            backward: 0,
        }
    }

    pub(super) fn is_reached(&self) -> bool {
        self.forward + self.backward >= self.total
    }

    pub(super) fn forward_pointer(&self) -> usize {
        self.forward
    }

    pub(super) fn backward_pointer(&self) -> usize {
        self.total - self.backward - 1
    }

    pub(super) fn forward_once(&mut self) {
        self.forward += 1;
    }

    pub(super) fn backward_once(&mut self) {
        self.backward += 1;
    }
}

#[derive(Debug)]
pub(super) struct ForwardGuard<'a> {
    counter: &'a mut Counter,
}

impl<'a> ForwardGuard<'a> {
    pub(super) fn build(counter: &'a mut Counter) -> Option<Self> {
        if counter.is_reached() {
            None
        } else {
            Some(ForwardGuard { counter })
        }
    }

    pub(super) fn pointer(&self) -> usize {
        self.counter.forward_pointer()
    }
}

impl<'a> Drop for ForwardGuard<'a> {
    fn drop(&mut self) {
        self.counter.forward_once();
    }
}

#[derive(Debug)]
pub(super) struct BackwardGuard<'a> {
    counter: &'a mut Counter,
}

impl<'a> BackwardGuard<'a> {
    pub(super) fn build(counter: &'a mut Counter) -> Option<Self> {
        if counter.is_reached() {
            None
        } else {
            Some(BackwardGuard { counter })
        }
    }

    pub(super) fn pointer(&self) -> usize {
        self.counter.backward_pointer()
    }
}

impl<'a> Drop for BackwardGuard<'a> {
    fn drop(&mut self) {
        self.counter.backward_once();
    }
}
