use crate::game::*;
use crate::ui::main::UiState;
use gloo::console::log as console_log;
use marble::traits::View;
use crate::model::{CombatState, Coord};
use crate::ui::ui_state_manager::{StatefullView};
use crate::ui::ui_state_manager::SelectedView::Combat;
use crate::utils::structs::AttackDefendPair;
use crate::views::info::ViewInfo;
use crate::views::main::ViewsEnum;

const DISPLAY_TIMEOUT_DEFAULT_MS: u32 = 2000;

impl Game {
    pub fn draw_board(&self) {
        if self.model.players.len() > 0 {
            crate::canvas::redraw_board_state(&self.model, self.flag_scale, true);
        } else {
            crate::canvas::redraw_board_state(&self.model, self.flag_scale, false);
        }
    }

    pub fn display_default_ms(&mut self, msg: &String) {
        self.info_display_div.display_with_timeout(msg, DISPLAY_TIMEOUT_DEFAULT_MS)
    }

    pub fn handle_canvas_click(&mut self, clicked_coord: Coord) {
        let prov_id_opt = self.lookup_coord(&clicked_coord);
        if prov_id_opt.is_some() {
            let prov_id = prov_id_opt.unwrap();
            match self.ui_info_ref().ui_state.get() {
                UiState::SETUP => { self.handle_canvas_noop(UiState::SETUP) }
                UiState::ARMY_PLACEMENT_START => self.handle_canvas_army_placement(prov_id, true),
                UiState::ARMY_PLACEMENT => self.handle_canvas_army_placement(prov_id, false),
                UiState::TURN => self.handle_canvas_turn(prov_id),
                UiState::COMBAT => { self.handle_canvas_noop(UiState::COMBAT) }
                UiState::GAME_END => { self.handle_canvas_noop(UiState::GAME_END) }
                UiState::CARD_SELECT => { self.handle_canvas_noop(UiState::CARD_SELECT) }
                UiState::DICE_ROLL => { self.handle_canvas_noop(UiState::DICE_ROLL) }
                UiState::MOVE => { self.handle_canvas_move(prov_id) }
                UiState::LABEL => {self.handle_canvas_noop(UiState::LABEL)}
            }
        }
    }

    pub fn handle_label_next(&mut self){
        let next_state = self.ui_man.view_label.get();
        self.log(format!("handleing label, setting state to {:?}", next_state.return_state));
        self.set_ui_state(next_state.return_state)
    }



    fn handle_canvas_turn(&mut self, prov_id: u32) {
        if self.is_owned_by_active(&prov_id) {
            let prov_name = self.model.get_prov_from_id(&prov_id).unwrap().name.clone();
            if self.model.get_prov_from_id(&prov_id).unwrap().army_count > 1{
                self.state_turn.targets.attack = Some(prov_id);
                self.display_default_ms(&format!("Attacking from {}", prov_name));
                return;
            }else {
                self.display_default_ms(&format!("Can't attack from {}. you need at least 2 armies", prov_name));
                return;
            }
        }

        if self.state_turn.targets.attack.is_some() {
            let id_attack_from = self.state_turn.targets.attack.as_ref().unwrap();

            let nav_res = self.model.nav_tree.navigate_adjacent(
                prov_id,
                self.state_turn.targets.attack.as_ref().unwrap().clone(),
            );
            if nav_res.is_some() {
                if nav_res.unwrap() {
                    self.state_turn.targets.defend = Some(prov_id.clone());

                    self.push_menu(ViewsEnum::Combat);
                    self.display_default_ms(&format!(
                        "Attacking {}",
                        self.model.get_name_from_prov_id(&prov_id).unwrap()
                    ))
                } else {
                    self.display_default_ms(&format!(
                        "Can't attack {} no connection",
                        self.model.get_name_from_prov_id(&prov_id).unwrap()
                    ))
                }
            } else {
                self.display_default_ms(&format!(
                    "Can't attack from {} to {}, no direct route.",
                    self.model.get_name_from_prov_id(id_attack_from).unwrap(),
                    self.model.get_name_from_prov_id(&prov_id).unwrap()
                ))
            }
        } else {
            self.display_default_ms(&format!("Please select a province you own."));
        }
        self.log("turn".to_string())
    }

    pub fn handle_start_turn(&mut self){
        // check if a player owns a continent
        console_log!("handle start turn");
        //self.state_turn.
        let player = self.get_active_player();
        let extra_armies = self.model.get_player_continent_armies(&player);

        if extra_armies > 0{
            self.show_label(
                format!("you get to place {extra_armies} extra armies, because you control continents."),
                UiState::ARMY_PLACEMENT);
            self.ui_man.army_placement.update(StateArmyPlacement{
                armies: extra_armies,
                active_player: player,
                end_turn_placement: false,
            });
            self.set_ui_state(UiState::LABEL);
        }

    }

    fn handle_canvas_army_placement(&mut self, prov_id: u32, placement_start: bool) {
        if !self.is_owned_by_active(&prov_id) {
            self.display_default_ms(&"You can only place armies, in provinces you own.".to_string());
            return;
        }

/*        console_log!(format!(
            "running placement id {}, start {}",
            prov_id, placement_start
        ));*/
        let armies_available: u32 = if placement_start {
            let tmp = self.ui_man.start_army_placement.get();
            tmp.armies[tmp.current_player as usize]
        } else {
            self.ui_man.army_placement.get().armies
        };

        if armies_available > 0 {
            self.change_armies_in_prov(1, &prov_id);
            self.display_default_ms(&format!("Placed army in {}.",
                                             self.model.get_name_from_prov_id(&prov_id).unwrap()));
            if placement_start {
                let mut state = self.ui_man.start_army_placement.get();
                state.armies[state.current_player as usize] -= 1;
                if state.armies[state.current_player as usize] == 0 {
                    if state.current_player + 1 < state.num_players {
                        state.current_player += 1;
                        self.config_sig
                            .as_ref()
                            .unwrap()
                            .active_player
                            .set(state.current_player);
                    } else {
                        self.config_sig.as_ref().unwrap().active_player.set(0);
                        self.set_ui_state(UiState::TURN);
                    }
                }
                self.ui_man.start_army_placement.update(state);
            } else {
                let mut state = self.ui_man.army_placement.get();
                state.armies -= 1;
                if state.armies == 0 {
                    if state.end_turn_placement {
                        self.set_next_player();
                    }
                    self.set_ui_state(UiState::TURN)
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

    pub fn handle_canvas_move(&mut self, _prov_id: u32) {
        todo!()
    }


    pub fn handle_ui_end_turn(&mut self) {
        console_log!("handle end turn");
        let menu = self.get_turn().borrow_mut();
        let _ = self.menu_stack.pop();
        if menu.can_reinforce {
            let mut reinforcing_army_count =
                self.model.get_prov_count_owned_by_player(self.state_turn.active_player);
            reinforcing_army_count = reinforcing_army_count / 3;
            if reinforcing_army_count < 3 {
                reinforcing_army_count = 3;
            }
            self.push_army_placement(reinforcing_army_count);
        }
        self.next_menu();
    }

    pub fn next_menu(&mut self){
        if self.menu_stack.is_empty(){
            // this means that the current players turn has ended
            //todo check if a player is knocked out and skip their turn
            // could be done recursively, but would need game end check
            self.log("menu stack empty moving on to next player".to_string());

            let current_player = self.state_turn.active_player;
            self.state_turn.active_player = if current_player + 1
                < self.model.players.len() as u32 {
                current_player + 1
            } else {
                0
            };

            self.info_display_div.set_default(format!("Player {}'s turn", self.state_turn.active_player));
        }else{

        }
    }

    pub fn push_army_placement(&mut self, armies:u32){
        let menu = self.get_army_placement().borrow_mut();
        menu.armies = armies;
        self.push_menu(ViewsEnum::ArmyPlacement);
    }

    pub fn push_turn_menu(&mut self){
    }


    pub fn handle_ui_combat_roll(&mut self, is_attack: bool) {
        self.log("combat ui handle".to_string());
        let mut combat_view = self.get_combat().borrow_mut();
        if is_attack {
            combat_view.is_visible.attack = false;
        } else {
            combat_view.is_visible.defend = false;
        }
        if !combat_view.is_visible.defend && !combat_view.is_visible.attack {
            if self.combat_state.combat_ongoing{
                self.combat_state.apply_losses();
            }else {
                self.combat_state.losses = AttackDefendPair::default();
                self.combat_state.combat_ongoing = true;

                let prov_ids = AttackDefendPair{
                    attack: self.state_turn.targets.attack.unwrap(),
                    defend: self.state_turn.targets.defend.unwrap(),
                };
                self.combat_state.armies = AttackDefendPair{
                    attack: self.model.get_prov_from_id(&prov_ids.attack).unwrap().army_count,
                    defend: self.model.get_prov_from_id(&prov_ids.defend).unwrap().army_count,
                };
                self.combat_state.prov_id = prov_ids;
            }

            self.combat_state.num_dice_used = AttackDefendPair{
                attack: combat_view.submenus.attack.get_armies_selected(),
                defend: combat_view.submenus.defend.get_armies_selected()
            };
            self.combat_state = self.model.combat_engine.next_round(
                self.combat_state.clone());

            let remaining_armies = self.combat_state.get_remaining();

            if remaining_armies.defend == 0 || remaining_armies.attack == 1{

            }else {

            }

            let dice_menu = self.get_view_dice().borrow_mut();

            self.set_ui_state(UiState::DICE_ROLL);
        } else {
            combat_view.update();
        }
    }

    pub fn handle_ui_dice_next(&mut self) {
        self.log("dice roll handler".to_string());
        let mut combat_result = self.ui_man.dice_rolls.get();
        self.log(format!("{:?}", combat_result));
        if !combat_result.has_rolled {
            self.log("rolling".to_string());
            combat_result = self.model.combat_engine.next_round(combat_result);
            self.info_display_div.set_default(format!("Losses attacker {}:{} defender",
                                                       combat_result.losses_attacker, combat_result.losses_defender));
            self.log(format!("dice attack {:?}", combat_result.dice_roll_attacker));
            self.log(format!("dice defend {:?}", combat_result.dice_roll_defender));

            self.ui_man.dice_rolls.update(combat_result);
        } else {
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
            if combat_result.combat_finished {
                self.log("combat finished".to_string());
                let mut turn_state = self.ui_man.turn_menu.get();
                turn_state.can_reinforce = false;
                self.ui_man.turn_menu.update(turn_state);
                self.set_ui_state(UiState::TURN)
            } else {
                self.log("combat ongoing".to_string());
                self.ui_man.combat.update(combat_state);
                self.ui_man.combat.reset_player_visibilty(&self.model.players);
                self.set_ui_state(UiState::COMBAT);
            }
        }
    }

    pub fn apply_combat_res_to_map(&mut self){
        self.combat_state.apply_losses();
        let provs = self.combat_state.prov_id.clone();
        let armies = self.combat_state.armies.clone();
        if self.combat_state.attacker_has_won(){
            self.set_armies_in_prov(1, &provs.attack);
            self.set_armies_in_prov(armies.attack - 1, &provs.defend);
            let mut conquered_prov = self.model
                .get_prov_from_id_mut(&provs.defend).unwrap();
            let new_owner = self.model.get_owner_from_prov_id(&provs.attack).unwrap();
            conquered_prov.owner_id = new_owner;
        } else {
            self.set_armies_in_prov(armies.attack, &provs.attack);
            self.set_armies_in_prov(armies.defend, &provs.defend);
        }
    }

    pub fn handle_ui_retreat(&mut self){
        todo!()
/*        self.log("retreat handle".to_string());
        let mut state = self.ui_man.turn_menu.get();
        state.can_reinforce = false;
        self.ui_man.turn_menu.update(state);
        self.set_ui_state(UiState::TURN);
    */}
}