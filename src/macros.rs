


#[macro_export]
macro_rules! create_getter {
    ($view_name:ident, $ty:ident) => {
        paste::paste!{
        impl Game{
            pub fn [<get_ $view_name>](&self)->Rc<RefCell<$ty>>{
                self.views.as_ref().unwrap().$view_name.clone()
            }
        }
        }
    };
}

#[macro_export]
macro_rules! bind {
    ($gettter:stmt, $var_name:ident) => {
        paste::paste!{
            let [<bind_ $var_name>] = $gettter;
            let $var_name = [<bind_ $var_name>].borrow();
        }
    };
}

#[macro_export]
macro_rules! bind_mut {
    ($gettter:stmt, $var_name:ident) => {
        paste::paste!{
            let [<bind_ $var_name>] = $gettter;
            let mut $var_name = [<bind_ $var_name>].borrow_mut();
        }
    };
}
