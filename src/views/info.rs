
use gloo_timers::callback::Timeout;
use std::cell::{RefCell};
use std::rc::Rc;
use marble::wrap::{*};
use marble::traits::{*};
#[allow(unused_imports)]
use gloo::console::log as console_log;



pub struct ViewInfo{
    info_div:WrpDiv,
    prev_timeout:Option<Timeout>,
    pub default_string:String,
}

impl ViewInfo{
    pub fn display_with_timeout(&mut self, display_str:&String, timeout_ms:u32){
        let ref_info_div = self.info_div.get_clone();
        let default_text = self.default_string.clone();
        self.info_div.inline_txt(display_str);

        if self.prev_timeout.is_some(){
            let prev =self.prev_timeout.take().unwrap();
            prev.cancel();
        }
        let timeout = Timeout::new(timeout_ms, move || {
            ref_info_div.inline_txt(&default_text);
        });
        self.prev_timeout = Some(timeout)
    }

    fn display_default(&mut self){
        self.info_div.inline_txt(&self.default_string);
    }
}

impl View for ViewInfo {
    fn update(&mut self) {
        self.display_default();
    }
}

fn create_view_info(mount_id:&str, default_string:String)->Rc<RefCell<ViewInfo>>{
    return Rc::new(RefCell::new(ViewInfo{
        info_div: WrpDiv::from_id(mount_id),
        prev_timeout: None,
        default_string,
    }))
}


