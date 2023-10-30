use std::rc::Rc;
use std::cell::{BorrowMutError, RefCell, RefMut};
use crate::element_getters::get_document;
use crate::ui::army_placement::{TemplHtml, TemplLabel, HTML_Label, HTMLable};
use crate::ui::templates::TEMPL_START_ARMY_PLACEMENT;

struct StartArmyPlacementVars{
    template:TemplHtml,
    player_label:TemplLabel,
    army_count_label:TemplLabel,
}

impl StartArmyPlacementVars {
    fn mount(&self ){
        self.template.mount();
        self.army_count_label.mount();
        self.player_label.mount();
    }
}

pub struct UiStateManager{
    start_army_placement:StartArmyPlacementVars
}

impl UiStateManager{
    pub fn build()->UiStateManager{
        let doc = get_document();
        UiStateManager{
            start_army_placement: StartArmyPlacementVars {
                template: TemplHtml::new(&doc, "templ_test".to_string(),TEMPL_START_ARMY_PLACEMENT.to_string()),
                player_label: TemplLabel::new(&doc,
                                              "label1".to_string(), "unset".to_string()),
                army_count_label: TemplLabel::new(&doc,
                                                  "label2".to_string(), "unset".to_string()),
            },
        }
    }

    pub fn mount(&self){
        self.start_army_placement.mount()
    }
}