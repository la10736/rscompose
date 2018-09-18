trait Action {
    fn do_action(&self);
}

fn action<'a, A: Action + 'a >(a: A)
    -> Box<Action + 'a> {
    Box::new(a)
}

struct ActionComposite {
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

//impl<'a, T: AsRef<[Box<Action>]>> Action for T {
//    fn do_action(&self) {
//        self.as_ref().iter().for_each(|a| a.do_action());
//    }
//}

macro_rules! compose {
( $first:expr ) => ( ($first) );
( $first:expr, $second:expr, ) => ( ($first, $second) );
( $first:expr, $( $x:expr ),* ) => {
        {
            ($first, compose!( $($x),*))
        }
};
}

fn main() {
    let v = ActionComposite::new(
        vec![action(Hello {}), action(World {})]
    );

    println!("{:?}", compose![Hello{}]);
    println!("{:?}", compose![Hello{}, World {}]);
    println!("{:?}", compose![Hello{}, World {}, Hello {}]);

    v.do_action();

//    let vv = [action(Hello {}), action(World {})];
//
//    vv.do_action();

    let vvv = Hello {}.compose(World {}).compose(Hello {});

    vvv.do_action();
    compose![Hello{}, World {}, Hello {}].do_action();
}

#[derive(Debug)]
struct Hello;
#[derive(Debug)]
struct World;

impl Action for Hello {
    fn do_action(&self) {
        print!("Hello");
    }
}

impl Action for World {
    fn do_action(&self) {
        println!(" World");
    }
}

impl<A: Action, B: Action> Action for (A, B) {
    fn do_action(&self) {
        self.0.do_action();
        self.1.do_action();
    }
}
impl<A: Action> Action for (A, ) {
    fn do_action(&self) {
        self.0.do_action();
    }
}

trait Compose: Sized {
    fn compose<A: Compose>(self, other: A)
        -> (Self, A) {
        (self, other)
    }
}

impl<A: Compose, B: Compose> Compose for (A, B) {}

impl Compose for Hello {}
impl Compose for World {}

