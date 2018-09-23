extern crate rand;

mod trait_obj;

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
}
