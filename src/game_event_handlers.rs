use crate::game::*;
use crate::ui::main::UiState;
use gloo::console::log as console_log;
use crate::model::{CombatResult, Coord};
use crate::ui::ui_state_manager::{StatefullView};
use crate::ui::view_combat::StateCombat;


impl Game{
    pub fn draw_board(&self) {
        if self.model.players.len() > 0 {
            crate::canvas::redraw_board_state(&self.model, self.flag_scale, true);
        } else {
            crate::canvas::redraw_board_state(&self.model, self.flag_scale, false);
        }
    }

    pub fn handle_canvas_click(&mut self, clicked_coord: Coord) {
        let prov_id_opt = self.lookup_coord(&clicked_coord);
        if prov_id_opt.is_some() {
            let prov_id = prov_id_opt.unwrap();
            match self.ui_info_ref().ui_state.get() {
                UiState::SETUP => {self.handle_canvas_noop(UiState::SETUP)}
                UiState::ARMY_PLACEMENT_START => self.handle_canvas_army_placement(prov_id, true),
                UiState::ARMY_PLACEMENT => self.handle_canvas_army_placement(prov_id, false),
                UiState::TURN => self.handle_canvas_turn(prov_id),
                UiState::COMBAT => { self.handle_canvas_noop(UiState::COMBAT) }
                UiState::GAME_END => { self.handle_canvas_noop(UiState::GAME_END) }
                UiState::CARD_SELECT => { self.handle_canvas_noop(UiState::CARD_SELECT) }
                UiState::DICE_ROLL => { self.handle_canvas_noop(UiState::DICE_ROLL) }
                UiState::MOVE => { self.handle_canvas_move(prov_id) }
            }
        }
    }

    fn handle_canvas_turn(&mut self, prov_id: u32) {
        if self.state_turn.attack_target.is_some() {
            let id_attack_from = self.state_turn.attack_target.as_ref().unwrap();
            if self.is_owned_by_active(&prov_id) {
                self.info_print("You can't attack your own province".to_string())
            } else {
                let nav_res = self.model.nav_tree.navigate_adjacent(
                    prov_id,
                    self.state_turn.attack_target.as_ref().unwrap().clone(),
                );
                if nav_res.is_some() {
                    if nav_res.unwrap() {
                        self.ui_man.hide_all();
                        let prov_attack = self.model.get_prov_from_id(id_attack_from).unwrap();
                        let prov_defend = self.model.get_prov_from_id(&prov_id).unwrap();

                        self.ui_man.combat.update(StateCombat {
                            attack_location: prov_defend.name.clone(),
                            armies_attacking: prov_attack.army_count - 1,
                            armies_defending: prov_defend.army_count,
                            id_attacker: prov_attack.owner_id,
                            id_defender: prov_defend.owner_id,
                            prov_id_attacker: prov_attack.id,
                            prov_id_defender: prov_defend.id,
                            attack_visible: true,
                            defend_visible: true,
                        });
                        self.set_ui_state(UiState::COMBAT);
                        self.info_print(format!(
                            "Attacking {}",
                            self.model.get_name_from_prov_id(&prov_id).unwrap()
                        ))
                    } else {
                        self.info_print(format!(
                            "Can't attack {} no connection",
                            self.model.get_name_from_prov_id(&prov_id).unwrap()
                        ))
                    }
                } else {
                    self.info_print(format!(
                        "Can't attack from {} to {}, no direct route.",
                        self.model.get_name_from_prov_id(id_attack_from).unwrap(),
                        self.model.get_name_from_prov_id(&prov_id).unwrap()
                    ))
                }
            }
        } else {
            if self.is_owned_by_active(&prov_id) {
                self.state_turn.attack_target = Some(prov_id);
                let prov_name = self.model.get_prov_from_id(&prov_id).unwrap().name.clone();
                self.info_print(format!("Attacking from {}", prov_name));
            } else {
                self.info_print(format!("Please select a province you own."));
            }
        }
        self.log("turn".to_string())
    }

    fn handle_canvas_army_placement(&mut self, prov_id: u32, placement_start: bool) {
        if !self.is_owned_by_active(&prov_id) {
            self.log("placement: prov not owned by active player".to_string());
            return;
        }

        console_log!(format!(
            "running placement id {}, start {}",
            prov_id, placement_start
        ));
        let armies_available: u32 = if placement_start {
            let tmp = self.ui_man.start_army_placement.get();
            tmp.armies[tmp.current_player as usize]
        } else {
            self.ui_man.army_placement.get().armies
        };

        if armies_available > 0 {
            self.change_armies_in_prov(1, &prov_id);
            if placement_start {
                let mut state = self.ui_man.start_army_placement.get();
                state.armies[state.current_player as usize] -= 1;
                if state.armies[state.current_player as usize] == 0 {
                    if state.current_player + 1 < state.num_players {
                        state.current_player += 1;
                        self.ui_info
                            .as_ref()
                            .unwrap()
                            .active_player
                            .set(state.current_player);
                    } else {
                        self.ui_info.as_ref().unwrap().active_player.set(0); //todo pass this to ui
                        self.set_ui_state(UiState::TURN);
                    }
                }
                self.ui_man.start_army_placement.update(state);

            } else {
                let mut state = self.ui_man.army_placement.get();
                state.armies -= 1;
                if state.armies == 0 {
                    //todo next state
                }
                self.ui_man.army_placement.update(state);
            }
        } else {
            panic!(
                "in placement state, with 0 armies to Place {:?}",
                self.ui_man.start_army_placement.get()
            );
        }
        self.ui_man.update_all();
        self.draw_board();
    }

    pub fn handle_canvas_move(&mut self, prov_id:u32){
        todo!()
    }


    pub fn handle_ui_end_turn(&mut self){
        todo!()
    }


    pub fn handle_ui_combat_roll(&mut self, is_attack:bool){
        self.log("combat ui handle".to_string());
        let mut state = self.ui_man.combat.get();
        if is_attack{
            state.attack_visible = false;
        }else {
            state.defend_visible = false;
        }
        if !state.defend_visible && !state.attack_visible{
            let armies_involved = self.ui_man.combat.get_armies_selected();
            self.ui_man.dice_rolls.update(CombatResult{
                armies_attacker: state.armies_attacking,
                armies_defender: state.armies_defending,
                losses_defender: 0,
                losses_attacker: 0,
                dice_roll_attacker: vec![],
                dice_roll_defender: vec![],
                has_rolled: false,
                combat_finished: false,
                active_attacker: armies_involved.0,
                active_defender: armies_involved.1,
            });
            self.set_ui_state(UiState::DICE_ROLL);
        }else {
            self.ui_man.combat.update(state);
        }
    }

    pub fn handle_ui_dice_next(&mut self){
        //todo retreat button
        self.log("dice roll handler".to_string());
        let mut combat_result = self.ui_man.dice_rolls.get();
        self.log(format!("{:?}", combat_result));
        if !combat_result.has_rolled{
            self.log("rolling".to_string());
            combat_result = self.model.combat_engine.next_round(combat_result);
            self.ui_man.dice_rolls.update(combat_result);
        }else {
            self.log("not rolling".to_string());
            combat_result.dice_roll_attacker.clear();
            combat_result.dice_roll_defender.clear();
            combat_result.losses_defender = 0;
            combat_result.losses_attacker = 0;
            //  state.has_rolled = false;
            let mut combat_state = self.ui_man.combat.get();
            combat_state.armies_attacking = combat_result.armies_attacker;
            combat_state.armies_defending = combat_result.armies_defender;
            self.apply_combat_result_to_map(&combat_state);
            if combat_result.combat_finished{
                //todo combat
                self.log("combat finished".to_string());
            }else{
                self.ui_man.combat.update(combat_state);
                self.ui_man.combat.reset_player_visibilty(&self.model.players);
                self.set_ui_state(UiState::COMBAT);
                self.log("combat ongoing".to_string());
            }
        }
    }
}