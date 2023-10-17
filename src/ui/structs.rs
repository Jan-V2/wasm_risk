use gloo::console::log;
use sycamore::prelude::{create_rc_signal, RcSignal};
use crate::ui::main::UiState;
use crate::utils::consts::MAX_PLAYERS;

pub trait UiUpdatable {
    fn update<F>(self, f:F ) -> Self
        where F: Fn(&mut Self),  Self: Sized;
}

#[derive(Clone)]
pub struct UiInfo{
    pub ui_state: RcSignal<UiState>,
    pub start_placement:RcSignal<StartArmyPlacementInfo>,
    pub placement:RcSignal<ArmyPlacementInfo>,
}

impl UiInfo {
    pub fn new()->UiInfo{
        UiInfo{
            ui_state:create_rc_signal(UiState::SETUP),
            start_placement: create_rc_signal(StartArmyPlacementInfo::new()),
            placement: create_rc_signal(ArmyPlacementInfo::new()),
        }
    }
}



#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct StartArmyPlacementInfo {
    pub is_done: bool,
    pub updated: bool,
    pub current_player: u32,
    pub num_players:u32,
    pub armies_per_player: [u32; MAX_PLAYERS],
}


impl UiUpdatable for StartArmyPlacementInfo{
    fn update<F>(self, f: F) -> Self
        where F: Fn(&mut Self), Self: Sized {
        let mut tmp = self.clone();
        f(&mut tmp);
        tmp.updated = true;
        log!(format!("updating start placement info {:?}", tmp.clone()));
        return tmp;
    }
}

impl StartArmyPlacementInfo {
    pub fn new() -> StartArmyPlacementInfo {
        StartArmyPlacementInfo {
            is_done: false,
            updated: false,
            current_player: 0,
            num_players: 0,
            armies_per_player: [0; MAX_PLAYERS],
        }
    }
}




#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct ArmyPlacementInfo {
    pub army_count: u32,
    pub is_done: bool,
    pub updated: bool,
    pub current_player: u32,
}


impl UiUpdatable for ArmyPlacementInfo{
    fn update<F>(self, f: F) -> Self
        where F: Fn(&mut Self), Self: Sized {
        let mut tmp = self.clone();
        f(&mut tmp);
        tmp.updated = true;
        return tmp;
    }
}

impl ArmyPlacementInfo {
    pub fn new() -> ArmyPlacementInfo {
        ArmyPlacementInfo {
            army_count: 0,
            is_done: false,
            updated: false,
            current_player: 0,
        }
    }
}
