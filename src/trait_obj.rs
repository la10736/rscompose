use ::Action;

// Utility Function to wrap a concrete impl in a dynamic trait object
pub fn action<'a, A: Action + 'a >(a: A)
                                   -> Box<Action + 'a> {
    Box::new(a)
}

pub struct ActionComposite {
    actions: Vec<Box<Action>>
}

impl ActionComposite {
    pub fn new(actions: Vec<Box<Action>>) -> Self {
        ActionComposite { actions }
    }
}

impl Action for ActionComposite {
    fn do_action(&self) {
        self.actions.do_action()
    }
}

impl Action for Vec<Box<Action>> {
    fn do_action(&self) {
        self.iter()
            .for_each(|a|
                a.do_action()
            );
    }
}

//More generic implementation. Pay attention: use it prevent any other generic impl
//
//impl<'a, T: AsRef<[Box<Action>]>> Action for T {
//    fn do_action(&self) {
//        self.as_ref().iter().for_each(|a| a.do_action());
//    }
//}
