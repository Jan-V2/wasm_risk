use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use marble::impl_visibility;
use marble::wrap::{*};
use marble::traits::{*};
use crate::game::Game;
#[allow(unused_imports)]
use gloo::console::log as console_log;
use crate::canvas::{clear_canvas, DiceFaceTex, draw_dice, get_dice_tex};
use crate::utils::structs::AttackDefendPair;
use crate::element_getters::get_drawing_context;
use crate::model::Coord;

const CANVAS_HEIGHT: u32 = 100;
const CANVAS_WIDTH: u32 = CANVAS_HEIGHT * 3;


pub struct ViewDiceRoll{
    head:WrpDiv,
    next_btn:WrpBtn,
    canvas:AttackDefendPair<WrpCanvas>,
    dice_face_texes: Rc<RefCell<Vec<DiceFaceTex>>>,
    game_ref: Rc<RefCell<Game>>,
    pub armies:AttackDefendPair<u32>,
    pub losses:AttackDefendPair<u32>,
    pub rolls:AttackDefendPair<Vec<u32>>,
    pub active:AttackDefendPair<bool>,
    pub has_rolled:bool,
    pub combat_finished:bool
}

impl View for ViewDiceRoll{
    fn update(&mut self) {
        //console_log!("updateing dice roll")
        self.draw_dice_rolls(true);
        self.draw_dice_rolls(false);
    }
}

impl_visibility!(ViewDiceRoll);


pub fn create_view_dice_roll(game: Rc<RefCell<Game>>, mount_id: &str)
                             ->Rc<RefCell<ViewDiceRoll>> {
    let mut next_btn = Button();
    let mut canvas = AttackDefendPair{
        attack: Canvas(),
        defend: Canvas(),
    };

    let head = Div().children(vec![
        Div().style("margin-bottom: 15px;").children(vec![
            H4().txt("Attacker rolled").child(
                canvas.attack.get_clone().attr("width", &CANVAS_WIDTH.to_string())
                    .attr("height", &CANVAS_HEIGHT.to_string())
            ),
            H4().txt("Defender rolled").child(
                canvas.defend.get_clone().attr("width", &CANVAS_WIDTH.to_string())
                    .attr("height", &CANVAS_HEIGHT.to_string())
            )
        ])
    ]).child(
        next_btn.get_clone().txt("Next")
    ).mount(mount_id);

    let view = Rc::from(RefCell::from(ViewDiceRoll{
        head,
        next_btn,
        canvas,
        dice_face_texes: get_dice_tex(),
        game_ref: game,
        armies: Default::default(),
        losses: Default::default(),
        rolls: Default::default(),
        active: Default::default(),
        has_rolled: false,
        combat_finished: false,
    }));

    view.borrow().next_btn.set_state_handler( view.clone(),
        #[allow(unused_mut)]
        |mut s:RefMut<ViewDiceRoll>|{
            s.game_ref.borrow_mut().handle_ui_dice_next();
    }, "attack btn"
    );
    return view;
}

impl ViewDiceRoll{
    fn draw_dice_rolls(&self,is_attacker:bool, ){
        let node = if is_attacker {
            self.canvas.attack.node.clone()
        }else{
            self.canvas.defend.node.clone()
        };

        let dice_rolls = if is_attacker {
            self.rolls.attack.clone()
        }else{
            self.rolls.defend.clone()
        };

        clear_canvas(&node, &get_drawing_context(&node), "LightCyan");
        let size = CANVAS_HEIGHT;
        for i in 0..dice_rolls.len(){
            let roll = dice_rolls[i];
            if roll > 6 || roll == 0{
                panic!("invalid dice roll. number {}", roll)
            }
            draw_dice(get_drawing_context(&node),
                      &self.dice_face_texes.as_ref().borrow()[(roll-1) as usize],
                      Coord{ y: 0, x: i as i32 * size as i32 }, size )
        }
    }
}

