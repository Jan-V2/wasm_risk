use crate::views::main::ViewsEnum;
use gloo::console::log as console_log;

pub struct MenuStack {
    stack: Vec<ViewsEnum>,
    debug: bool,
}

impl MenuStack {
    pub fn new(debug: bool) -> MenuStack {
        MenuStack {
            stack: vec![],
            debug,
        }
    }

    pub fn push(&mut self, menu: ViewsEnum) {
        if self.debug {
            console_log!(format!("pushing {:?} to stack", menu));
            self.print_stack();
        }
        self.stack.push(menu);
    }


    pub fn get(&self) -> Option<ViewsEnum> {
        if self.debug {
            console_log!("getting from stack");
            self.print_stack();
        }
        if self.stack.len() > 0 {
            return Some(self.stack[self.stack.len() - 1].clone());
        }
        None
    }

    pub fn next_menu(&mut self) -> Option<ViewsEnum> {
        if self.debug {
            console_log!("getting next menu");
            self.print_stack();
        }
        let _ = self.stack.pop();
        let ret = self.get();
        console_log!(format!("returning {:?} as next menu", ret));
        return ret;
    }


    pub fn is_empty(&self) -> bool {
        self.stack.len() == 0
    }

    pub fn len(&self) -> u32 {
        self.stack.len() as u32
    }

    // clears stack and returns the num of items cleared
    pub fn clear(&mut self) -> u32 {
        if self.debug {
            console_log!("clearing stack");
            self.print_stack();
        }
        let ret = self.len();
        self.stack.clear();
        return ret;
    }

    fn print_stack(&self) {
        console_log!(format!("stack is {:?}", self.stack));
    }
}
