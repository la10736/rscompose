use Action;

impl<A: Action, B: Action> Action for (A, B) {
    fn do_action(&self) {
        self.0.do_action();
        self.1.do_action();
    }
}
