mod trait_obj;
mod concrete;
mod action_tuple;
#[macro_use]
mod compose;

pub trait Action {
    fn do_action(&self);
}

#[derive(Debug)]
struct Hello;
#[derive(Debug)]
struct World;

impl Action for Hello { fn do_action(&self) { print!("Hello"); } }
impl Action for World { fn do_action(&self) { println!(" World"); } }

fn main() {
    println!("Trait Obj:");
    use trait_obj::action;

    let t_obj = trait_obj::ActionComposite::new(
        vec![action(Hello {}), action(World {})]
    );

    t_obj.do_action();
    println!("----------------------------------------");

    //
    //    println!("Wild Slice:");
    //
    //    let wild_slice = [action(Hello {}), action(World {})];
    //
    //    wild_slice.do_action();
    //    println!("----------------------------------------");
    //

    println!("Concrete Impl:");

    let c = concrete::ActionComposite::new(Hello {}, World {});

    c.do_action();

    println!("... compose by World {{}} again:");
    c.compose(World {}).do_action();
    println!("----------------------------------------");


    println!("Wild Tuple:");
    let wild_tuple = (Hello {}, World {});
    wild_tuple.do_action();
    println!("----------------------------------------");

    println!("Composite tuple!");
    use compose::Compose;

    impl Compose for Hello {}
    impl Compose for World {}

    let wild_compose = Hello {}.compose(World {});
    wild_compose.do_action();

    println!("... compose by Hello {{}} again:");
    wild_compose.compose(Hello {}).do_action();
    println!();
    println!("----------------------------------------");


    println!("compose! macro:");
    println!("{:?}", compose![Hello{}]);
    println!("{:?}", compose![Hello{}, World {}]);
    println!("{:?}", compose![Hello{}, World {}, Hello {}]);
    println!("{:?}", compose![Hello{}, World {}, Hello {}, World {}]);

    compose![Hello{}, World {}, Hello {}, World {}].do_action();
    println!("----------------------------------------");
}
