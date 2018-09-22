use Action;

pub struct ActionComposite<A: Action, B: Action> {
    data: (A, B)
}

impl<A: Action, B: Action> ActionComposite<A, B> {
    pub fn new(a: A, b: B) -> Self {
        Self { data: (a, b) }
    }

    pub fn compose<C: Action>(self, other: C) -> ActionComposite<Self, C> {
        ActionComposite::<Self, C>::new(self, other)
    }
}

impl<A: Action, B: Action> Action for ActionComposite<A, B> {
    fn do_action(&self) {
        self.data.0.do_action();
        self.data.1.do_action();
    }
}
