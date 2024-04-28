#[derive(Clone, Debug)]
pub(super) struct Counter {
    total: usize,
    forward: usize,
    backward: usize,
}

impl Counter {
    pub fn new(total: usize) -> Self {
        Self {
            total,
            forward: 0,
            backward: 0,
        }
    }

    pub fn is_reached(&self) -> bool {
        self.forward + self.backward >= self.total
    }

    pub fn forward_pointer(&self) -> usize {
        self.forward
    }

    pub fn backward_pointer(&self) -> usize {
        self.total - self.backward - 1
    }

    pub fn forward_once(&mut self) {
        self.forward += 1;
    }

    pub fn backward_once(&mut self) {
        self.backward += 1;
    }
}

#[derive(Debug)]
pub(super) struct ForwardGuard<'a> {
    counter: &'a mut Counter,
}

impl<'a> ForwardGuard<'a> {
    pub fn build(counter: &'a mut Counter) -> Option<Self> {
        if counter.is_reached() {
            None
        } else {
            Some(ForwardGuard { counter })
        }
    }

    pub fn pointer(&self) -> usize {
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
    pub fn build(counter: &'a mut Counter) -> Option<Self> {
        if counter.is_reached() {
            None
        } else {
            Some(BackwardGuard { counter })
        }
    }

    pub fn pointer(&self) -> usize {
        self.counter.backward_pointer()
    }
}

impl<'a> Drop for BackwardGuard<'a> {
    fn drop(&mut self) {
        self.counter.backward_once();
    }
}
