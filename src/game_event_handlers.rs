use crate::game::*;
use gloo::console::log as console_log;
use marble::traits::View;
use crate::{bind, bind_mut};
use crate::model::{CombatState, Coord, Rules};
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
        let mut view_info = self.info_view.borrow_mut();
        view_info.display_with_timeout(msg, DISPLAY_TIMEOUT_DEFAULT_MS);
    }

    pub fn handle_canvas_click(&mut self, clicked_coord: Coord) {
        if self.state_turn.in_setup {
            return;
        }
        let prov_id_opt = self.lookup_coord(&clicked_coord);
        if prov_id_opt.is_some() {
            let prov_id = prov_id_opt.unwrap();
            match self.menu_stack.get() {
                ViewsEnum::Turn => { self.handle_canvas_turn(prov_id) }
                ViewsEnum::ArmyPlacement => { self.handle_canvas_army_placement(prov_id) }
                ViewsEnum::Combat => { self.handle_canvas_noop(ViewsEnum::Combat) }
                ViewsEnum::DiceRolls => { self.handle_canvas_noop(ViewsEnum::DiceRolls) }
                ViewsEnum::Message => {self.handle_canvas_noop(ViewsEnum::Message)}
            }
        }
    }

    fn handle_canvas_turn(&mut self, prov_id: u32) {
        if self.is_owned_by_active(&prov_id) {
            let prov_name = self.model.get_prov_from_id(&prov_id).unwrap().name.clone();
            if self.model.get_prov_from_id(&prov_id).unwrap().army_count > 1 {
                self.state_turn.targets.attack = Some(prov_id);
                self.display_default_ms(&format!("Attacking from {}", prov_name));
                return;
            } else {
                self.display_default_ms(&format!(
                    "Can't attack from {}. you need at least 2 armies", prov_name));
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

                    self.push_and_activate_menu(ViewsEnum::Combat);
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


    fn handle_canvas_army_placement(&mut self, prov_id: u32) {
        // handles the canvas click event for amry placement, then pops the stack
        self.log("handling canvas army placement".to_string());

        if !self.is_owned_by_active(&prov_id) {
            self.display_default_ms(
                &"You can only place armies, in provinces you own.".to_string());
            return;
        }

        let mut next_menu = false;
        {
            bind_mut!(self.get_army_placement(), placement_menu);
            let armies_available: u32 = placement_menu.armies;

            if armies_available > 0 {
                self.change_armies_in_prov(1, &prov_id);
                self.display_default_ms(&format!("Placed army in {}.",
                                                 self.model.get_name_from_prov_id(&prov_id).unwrap()));
                placement_menu.armies -= 1;
                if placement_menu.armies == 0 {
                    next_menu = true;
                } else {
                    placement_menu.update();
                }
            } else {
                panic!(
                    "in placement state, with 0 armies to Place"
                );
            }
        }
        self.draw_board();
        if next_menu {
            self.pop_menu()
        }
    }

    pub fn handle_canvas_move(&mut self, _prov_id: u32) {
        todo!()
    }


    pub fn army_placement_start_next(&mut self, from_setup: bool) {
        self.log("in initial placement phase".to_string());
        if self.state_turn.active_player == self.model.get_player_count() - 1 {
            self.log("done full cycle, resetting".to_string());
            self.state_turn.in_initial_placement_phase = false;
            self.pop_menu();
        } else {
            if !from_setup {
                self.state_turn.active_player += 1;
            }
            self.log(format!("setting up placement for player {}",
                             self.state_turn.active_player));

            {
                let armies_per_player =
                    Rules::armies_per_players_start(self.model.get_player_count()).unwrap();
                let placeable_armies = armies_per_player -
                    self.model.get_prov_count_owned_by_player(self.state_turn.active_player);
                self.push_army_placement(placeable_armies);
                self.activate_current_menu();
            }
            if from_setup {
                self.activate_current_menu();// borrows army placement
            }
        }
    }

    pub fn handle_end_turn(&mut self, can_reinforce:bool){
        if can_reinforce{
            console_log!("handling player reinforce");
            bind_mut!(self.get_army_placement(), placement_menu);

            let player_prov_num =
                self.model.get_prov_count_owned_by_player(self.state_turn.active_player);
            let mut reinforcing_army_count = player_prov_num / 3;
            if reinforcing_army_count < 3 {
                reinforcing_army_count = 3;
            }
            placement_menu.reset(reinforcing_army_count);
            self.menu_stack.set_current(ViewsEnum::ArmyPlacement);
            self.push_message_view(format!("Because you own {} provinces, you're allowed \
                to place {} extra armies.", player_prov_num, reinforcing_army_count));
        }
        // pops the turn menu
        // should move on to next player if no reinforcements
        // otherwise should pop
        self.pop_menu_async(1);
    }


    pub fn push_message_view(&mut self, msg:String){
        bind_mut!(self.get_message(), msg_menu);
        msg_menu.message = msg;
        self.menu_stack.push(ViewsEnum::Message)
    }

    pub fn pop_menu(&mut self) {
        if self.menu_stack.is_empty() {
            // this means that the current players turn has ended
            //todo check if a player is knocked out and skip their turn
            self.log("menu stack empty moving on to next player".to_string());

            if self.state_turn.in_initial_placement_phase {
                self.army_placement_start_next(false);
            } else {
                self.setup_next_turn();
            }
            self.activate_current_menu();
        } else {
            let _ = self.menu_stack.pop();
            self.activate_current_menu();
        }
    }

    pub fn push_army_placement(&mut self, armies: u32) {
        bind_mut!(self.get_army_placement(), menu);
        menu.armies = armies;
        self.menu_stack.push(ViewsEnum::ArmyPlacement);
    }

    pub fn setup_next_turn(&mut self) {
        // check if a player owns a continent
        self.log("setting up next turn".to_string());

        // select next player
        //todo skip over players that have been defeated
        let prev_player = self.state_turn.active_player;
        self.state_turn.active_player = if prev_player + 1
            < self.model.players.len() as u32 {
            prev_player + 1
        } else {
            0
        };

        self.info_view.borrow_mut().set_default(
            &format!("Player {}'s turn", self.state_turn.active_player));

        bind_mut!(self.get_turn(), turn_menu);
        let active_player = self.state_turn.active_player;
        turn_menu.reset(active_player);
        self.menu_stack.push(ViewsEnum::Turn);

        let extra_armies = self.model.get_player_continent_armies(&active_player);
        if extra_armies > 0 {
            self.push_army_placement(extra_armies);
            self.push_message_view(format!("you get to place {extra_armies} \
                extra armies, because you control continents."));
        }

    }

    pub fn handle_message_next(&mut self){
        self.pop_menu();
    }


    pub fn handle_ui_combat_roll(&mut self, is_attack: bool) {
        self.log("combat syca handle".to_string());
        bind_mut!(self.get_combat(), combat_view);
        //bind_mut!(self.get_combat(), combat_view);
        if is_attack {
            combat_view.is_visible.attack = false;
        } else {
            combat_view.is_visible.defend = false;
        }
        if !combat_view.is_visible.defend && !combat_view.is_visible.attack {
            // if both sides have selected their armies.
            if self.combat_state.combat_ongoing {
                // if this is not the first round of combat
                self.combat_state.apply_losses();
            } else {
                // if it is setup state
                self.combat_state.losses = AttackDefendPair::default();
                self.combat_state.combat_ongoing = true;

                let prov_ids = AttackDefendPair {
                    attack: self.state_turn.targets.attack.unwrap(),
                    defend: self.state_turn.targets.defend.unwrap(),
                };
                self.combat_state.armies = AttackDefendPair {
                    attack: self.model.get_prov_from_id(&prov_ids.attack).unwrap().army_count,
                    defend: self.model.get_prov_from_id(&prov_ids.defend).unwrap().army_count,
                };
                self.combat_state.prov_id = prov_ids;
            }

            self.combat_state.num_dice_used = AttackDefendPair {
                attack: combat_view.submenus.attack.get_armies_selected(),
                defend: combat_view.submenus.defend.get_armies_selected(),
            };
            self.combat_state = self.model.combat_engine.next_round(
                self.combat_state.clone());
            // the combat state gets applied to the map in the dice roll menu

            bind_mut!(self.get_dice_rolls(), dice_menu);
            dice_menu.reset(self.combat_state.dice_rolls.clone(),
                    self.combat_state.losses.clone());
            self.push_and_activate_menu(ViewsEnum::DiceRolls);
        } else {
            combat_view.update();
        }
    }

    pub fn handle_ui_dice_next(&mut self) {
        todo!()
        /*        self.log("dice roll handler".to_string());
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
        */
    }



    pub fn apply_combat_res_to_map(&mut self) {
        self.combat_state.apply_losses();
        let provs = self.combat_state.prov_id.clone();
        let armies = self.combat_state.armies.clone();
        if self.combat_state.attacker_has_won() {
            self.set_armies_in_prov(1, &provs.attack);
            self.set_armies_in_prov(armies.attack - 1, &provs.defend);
            let new_owner = self.model.get_owner_from_prov_id(&provs.attack).unwrap();
            let conquered_prov = self.model
                .get_prov_from_id_mut(&provs.defend).unwrap();
            conquered_prov.owner_id = new_owner;
        } else {
            self.set_armies_in_prov(armies.attack, &provs.attack);
            self.set_armies_in_prov(armies.defend, &provs.defend);
        }
    }

    pub fn handle_ui_retreat(&mut self) {
        self.log("retreat handle".to_string());
        bind_mut!(self.get_turn(), turn);
        turn.can_reinforce = false;
        self.pop_menu();
    }
}