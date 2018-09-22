use Action;

pub struct ActionComposite<A: Action, B: Action>(A, B);

impl<A: Action, B: Action> ActionComposite<A, B> {
    pub fn new(a: A, b: B) -> Self {
        ActionComposite(a, b)
    }

    pub fn compose<C: Action>(self, other: C) -> ActionComposite<Self, C> {
        ActionComposite::new(self, other)
    }
}

impl<A: Action, B: Action> Action for ActionComposite<A, B> {
    fn do_action(&self) {
        self.0.do_action();
        self.1.do_action();
    }
}
