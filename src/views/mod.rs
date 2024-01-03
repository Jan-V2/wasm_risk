extern crate marble;

pub mod turn;
pub mod army_placement;
pub mod main;
mod combat;


#[macro_export] //todo make this derive
macro_rules! build_constructor {
    ($func_name:ident, $view_construct:ident, $t:ty ) => {
        pub fn $func_name(game: Rc<RefCell<Game>>, mount_id:&str)-> Rc<RefCell<$t>>{
            let mut res = $view_construct(game, mount_id);
            res.update();
            return Rc::new(RefCell::new(res))
        }
    };
}