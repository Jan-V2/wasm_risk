use crate::ui::wrap_elem::{WrapDiv, HTML_Div, HTMLable};
use gloo_timers::callback::Timeout;

pub struct ViewInfo{
    display_default:String,
    target:WrapDiv,
    target_id:String,
    prev_timeout:Option<Timeout>,
}

impl ViewInfo{
    pub fn create(div_id:&String, default_str:String)->ViewInfo{
        let mut ret = ViewInfo{
            display_default: default_str,
            target: WrapDiv::new_from_id(div_id),
            target_id: div_id.clone(),
            prev_timeout: None,
        };
        ret.display_default();
        return ret
    }

    pub fn set_default(&mut self, default_str:String){
        self.display_default = default_str;
        self.display_default();
    }

    pub fn display_with_timeout(&mut self, display_str:&String, timeout_ms:u32){
        let _target_id = self.target_id.clone();
        let default_text = self.display_default.clone();
        self.target.set_text(display_str);

        if self.prev_timeout.is_some(){
            let prev =self.prev_timeout.take().unwrap();
            prev.cancel();
        }
        let timeout = Timeout::new(timeout_ms, move || {
            let mut div = WrapDiv::new_from_id(&_target_id);
            div.set_text(&default_text);
        });
        self.prev_timeout = Some(timeout)
    }

    fn display_default(&mut self){
        self.target.set_text(&self.display_default);
    }

}
