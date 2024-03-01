use gloo::console::log as console_log;
use crate::views::main::ViewsEnum;


pub struct MenuStack{
    stack:Vec<ViewsEnum>,
    debug:bool,
}


impl MenuStack{
    pub fn new(debug:bool)->MenuStack{
        MenuStack{
            stack:vec![],
            debug,
        }
    }

    pub fn push(&mut self, menu:ViewsEnum){
        if self.debug{
            console_log!(format!("pushing {:?} to stack", menu));
            self.print_stack();
        }
        self.stack.push(menu);
    }
    
    pub fn pop(&mut self)->Option<ViewsEnum>{
        if self.debug{
            console_log!("popping stack");
            self.print_stack();
        }
        return self.stack.pop()
    }

    pub fn get(&self)->Option<ViewsEnum>{
        if self.debug{
            console_log!("getting from stack");
            self.print_stack();
        }
        if self.stack.len() > 0{
            return Some(self.stack[0].clone());
        }
        None
    }

    pub fn next_menu(&mut self,)->Option<ViewsEnum>{
        self.stack.pop()
    }

    pub fn is_empty(&self)->bool{
        self.stack.len() == 0
    }
   
    pub fn len(&self)->u32{
        self.stack.len() as u32 
    }
    
    fn print_stack(&self){
        console_log!(format!("stack is {:?}", self.stack));
    }
}
