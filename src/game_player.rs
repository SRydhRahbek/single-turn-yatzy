use fraction::GenericFraction;
use std::collections::HashMap;
//use std::sync::Arc;
type F = GenericFraction<u32>;
use crate::logic::{Board, Category, Hand, Mask, Maskhand, MaskhandKey, StraightData, Turn, GameData};
//use fraction::convert::TryToConvertFrom;
use std::fmt;

pub fn play_game(maskhandmap: HashMap<MaskhandKey, Maskhand>) -> (String, Vec<StraightData>, GameData) {
    let mut board = Board::new(); // S:   Create a board with a 0 in 'Chance' with Board::zero_chance()
    
    //Initialize the game
    let (
        empty_return_mask, all_hands, crossout_order_list, mut empty_categories, mut straight_data_vec, mut turn_number, mut turn_vec
    ) = (
        Mask::empty(), Hand::all_hands(), selected_crossout_order_list(), board.empty_categories(), Vec::new(), 1, Vec::new()
    );

    //HashMap to match each hand in the final step to it's point value in each caregory
    let mut final_step_evalmap: HashMap<&Hand, HashMap<Category, u32>> = HashMap::new();
    for hand in &all_hands {final_step_evalmap.insert(hand, hand.evaluate_and_stack());}
    
    loop {
        let mut current_turn = Turn::new(turn_number, empty_categories.clone());

        let (step1_best_evalmap, step2_best_evalmap, step3_best_evalmap) = generate_all_evalmaps(&empty_categories, &all_hands, &final_step_evalmap, &empty_return_mask, &maskhandmap);
        
        
        let mut hand = Hand::random();
        let best_mask = step1_best_evalmap
            .get(&hand)
            .expect("step1 best doesn't contain all hands")
            .0;
        analyze_roll(&mut current_turn, &mut straight_data_vec, hand.clone(), *best_mask, 1, &empty_categories);

        hand.reroll_with_mask(&best_mask);
        let best_mask = step2_best_evalmap
            .get(&hand)
            .expect("step2 best doesn't contain all hands")
            .0;
        analyze_roll(&mut current_turn, &mut straight_data_vec, hand.clone(), *best_mask, 2, &empty_categories);
        
        hand.reroll_with_mask(&best_mask);
        let best_mask = step3_best_evalmap  //This step is only done for analysis purposes, hand.evaluate() works fine without it
            .get(&hand)
            .expect("step3 best doesn't contain all hands")
            .0;
        analyze_roll(&mut current_turn, &mut straight_data_vec, hand.clone(), *best_mask, 3, &empty_categories);

        let hand_value_category = hand.evaluate(&board);
        
        current_turn.placed_category = hand_value_category.1.clone();
        current_turn.points = hand_value_category.0;

        if let Some(category) = hand_value_category.1 {
            board.place_value_in_category(hand_value_category.0, category);
            empty_categories = empty_categories
                .into_iter()
                .filter(|x| x != &category)
                .collect();
        } else {
            for category in &crossout_order_list {
                if empty_categories.contains(category) {
                    board.place_value_in_category(0, *category);
                    empty_categories = empty_categories
                        .into_iter()
                        .filter(|x| x != category)
                        .collect();
                    break;
                }
            }
        }
        turn_vec.push(current_turn);
        turn_number += 1;
        if empty_categories.len() == 0 {break;}
        
    }
    let game_data: GameData = GameData::new(turn_vec, board.clone());
    let analysis: BoardAnalysis = BoardAnalysis::new(&board);

    //println!("Board: {}", board);
    //println!("{}", analysis);

    let analysis_string: String = analysis.create_analysis_string();

    return (analysis_string, straight_data_vec, game_data);
}

fn analyze_roll(turn: &mut Turn, straight_data_vec: &mut Vec<StraightData>, hand: Hand, best_mask: Mask, slag: u32, empty_categories: &Vec<Category>) {
    let maskhand_to_subset = MaskhandKey::from(hand.clone(), best_mask);
    let mut subset = maskhand_to_subset.merge_to_subset();
    subset.sort();
    let mut duplicate_free_subset = subset.clone();
    duplicate_free_subset.dedup();
    turn.straight_state.update(&duplicate_free_subset);
    match slag {
        1 => {
            turn.subset1 = subset.clone();
            turn.hand1 = hand.clone();
        }
        2 => {
            turn.subset2 = subset.clone();
            turn.hand2 = hand.clone();
        }
        3 => {
            turn.subset3 = subset.clone();
            turn.hand3 = hand.clone();
        }
        _ => {}
    }
    if duplicate_free_subset.len() == subset.len() && duplicate_free_subset.len() >= 3 {
        straight_data_vec.push(StraightData::new(hand.clone(), subset, 2, empty_categories.clone(), turn.turn_uuid.clone()))
    }

    /*if duplicate_free_subset.len() = 5 {
        turn.straight_state = StraightState::Failed;
    }

    if is_subset_straight(&mut subset) {
        straight_data_vec.push(StraightData::new(hand.clone(), subset, 2, empty_categories.clone(), uuid.clone()))
    }*/
}


fn generate_all_evalmaps<'a>(empty_categories: &'a Vec<Category>, all_hands: &'a Vec<Hand>, final_step_evalmap: &'a HashMap<&Hand, HashMap<Category, u32>>, empty_return_mask: &'a Mask, maskhandmap: &'a HashMap<MaskhandKey, Maskhand>) -> (HashMap<&'a Hand, (&'a Mask, F)>, HashMap<&'a Hand, (&'a Mask, F)>, HashMap<&'a Hand, (&'a Mask, F)>) {
    let step3_best_evalmap = get_final_step_best_evalmap(
        empty_categories,
        all_hands,
        final_step_evalmap,
        empty_return_mask,
    );
    let step2_evalmap = get_next_evalmap(maskhandmap, &step3_best_evalmap);
    let step2_best_evalmap = transform_to_best_evalmap(/*maskhandmap,*/ step2_evalmap);
    let step1_evalmap = get_next_evalmap(maskhandmap, &step2_best_evalmap);
    let step1_best_evalmap = transform_to_best_evalmap(/*maskhandmap,*/ step1_evalmap);
    return (step1_best_evalmap, step2_best_evalmap, step3_best_evalmap);
}

fn selected_crossout_order_list() -> Vec<Category> {
    //let crossout_order_list = Category::all_categories();
    let sara_crossout_order_list: Vec<Category> = vec![
        Category::Ettor,
        Category::Yatzy,
        Category::LitenStraight,
        Category::StorStraight,
        Category::Tvaor,
        Category::Fyrtal,
        Category::Treor,
        Category::Kak,
        Category::Fyror,
        Category::Tretal,
        Category::Femmor,
        Category::Par,
        Category::Tvapar,
        Category::Sexor,
        Category::Chans,
    ];
    /*let david_crossout_order_list: Vec<Category> = vec![
        Category::LitenStraight,
        Category::StorStraight,
        Category::Yatzy,
        Category::Fyrtal,
        Category::Kak,
        Category::Ettor,
        Category::Tvaor,
        Category::Tretal,
        Category::Tvapar,
        Category::Treor,
        Category::Par,
        Category::Fyror,
        Category::Femmor,
        Category::Sexor,
        Category::Chans,
    ];*/
    return sara_crossout_order_list;
}

//=======================================

fn get_final_step_best_evalmap<'a>(
    empty_categories: &Vec<Category>,
    all_hands: &'a Vec<Hand>,
    final_step_evalmap: &HashMap<&Hand, HashMap<Category, u32>>,
    empty_return_mask: &'a Mask,
) -> HashMap<&'a Hand, (&'a Mask, F)> {
    let mut final_step_best_evalmap = HashMap::new();
    for hand in all_hands {
        let handmap = final_step_evalmap.get(&hand).expect(
            "Somehow there exists a hand that has not been evaluated in final_step_evalmap",
        );
        let mut best_category_value = 0;
        for category in empty_categories {
            let category_value = handmap.get(category).expect(
                "Somehow there exists a category that has not been evaluated in final_step_evalmap",
            );
            if category_value > &best_category_value {
                best_category_value = *category_value;
            }
        }
        final_step_best_evalmap.insert(hand, (empty_return_mask, F::from(best_category_value)));
    }
    return final_step_best_evalmap;
} //play_game()

//=======================================THINGS FOR PLAYING THE GAME===============================================

//We take in maskhandmap, which maps each maskhandkey to its corresponding expansion of stacked Hand:s, and current_best_evalmap which describes what each Hand is worth at most in the current step, and with these two we calculate and return the weighted value of each maskhandkey in the next step
fn get_next_evalmap<'a>(
    maskhandmap: &'a HashMap<MaskhandKey, Maskhand>,
    current_best_evalmap: &HashMap<&'a Hand, (&'a Mask, F)>,
) -> HashMap<&'a MaskhandKey, F> {
    let mut next_evalmap: HashMap<&'a MaskhandKey, F> = HashMap::new();
    for (maskhandkey, maskhand) in maskhandmap.iter() {
        let f = maskhand.evaluate_maskhand_against_map(current_best_evalmap);
        next_evalmap.insert(maskhandkey, f);
    }
    return next_evalmap;
}

//Takes in an evalmap which assigns a value to each possible combination of mask and hand, and extracts the best value for each hand in it
//The input evalmap should already have taken into account that some fields may be taken, and thus a board should not be needed in the input
fn transform_to_best_evalmap<'a>(
    /*maskhandmap: &'a HashMap<MaskhandKey, Maskhand>,*/
    next_evalmap: HashMap<&'a MaskhandKey, F>,
) -> HashMap<&'a Hand, (&'a Mask, F)> {
    //Create an empty map where we place the best hands from next_evalmap
    let mut next_best_evalmap: HashMap<&'a Hand, (&'a Mask, F)> = HashMap::new();

    //We go through each pair of maskhand and value from next_evalmap, and if the value for the hand from the maskhand is higher than the current highest for that hand, we replace the previous best hand and value with the new best hand and value
    for (maskhandkey, value) in next_evalmap {
        //We declare a fallback value, to be used later
        let fallback = &(&Mask::empty(), F::from(0));

        //If the hand we are comparing hasn't been evaluated before, we pretend its previous evaluation is our fallback value, eg. The Empty Hand, with the value of 0
        let (_, current_best_handvalue) =
            next_best_evalmap.get(&maskhandkey.hand).unwrap_or(fallback);
        if value > *current_best_handvalue {
            next_best_evalmap.insert(&maskhandkey.hand, (&maskhandkey.mask, value));
        }
    }

    return next_best_evalmap;
}

/***********************BOARD ANALYSIS FOR GETTING DATA***************************/

#[derive(Debug, Clone)]
pub struct BoardAnalysis {
    finalboard: HashMap<Category, u32>,
    //top_category_score: u32,
    bonus: u32,
    //fill_chance: u32,
    total_score: u32,
}

impl BoardAnalysis {
    fn new(board: &Board) -> BoardAnalysis {
        let mut top_category_score = 0;
        for category in Category::all_categories().drain(..6) {
            top_category_score += board.get(category).unwrap();
        }

        let bonus = if top_category_score >= 63 { 50 } else { 0 };

        /*
        let fill_chance = if board.get(Category::Chans).unwrap() == 0 {
            15
        } else {
            0
        };
        */

        let mut finalboard: HashMap<Category, u32> = HashMap::new();
        for category in Category::all_categories() {
            finalboard.insert(category, board.get(category).unwrap());
        }

        /*
        if fill_chance != 0 {
        finalboard.insert(Category::Chans, fill_chance);
        }
         */

        let mut total_score = 0;
        for category in Category::all_categories() {
            total_score += board.get(category).unwrap();
        }
        total_score += bonus; // + fill_chance

        BoardAnalysis {
            finalboard,
            //top_category_score,
            bonus,
            //fill_chance,
            total_score, //including bonus and non-zero category Chance
        }
    }

    fn create_analysis_string(&self) -> String {
        format!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            self.finalboard[&Category::Ettor],
            self.finalboard[&Category::Tvaor],
            self.finalboard[&Category::Treor],
            self.finalboard[&Category::Fyror],
            self.finalboard[&Category::Femmor],
            self.finalboard[&Category::Sexor],
            self.finalboard[&Category::Par],
            self.finalboard[&Category::Tvapar],
            self.finalboard[&Category::Tretal],
            self.finalboard[&Category::Fyrtal],
            self.finalboard[&Category::LitenStraight],
            self.finalboard[&Category::StorStraight],
            self.finalboard[&Category::Kak],
            self.finalboard[&Category::Chans],
            self.finalboard[&Category::Yatzy],
            self.bonus,
            self.total_score
        )
    }
}

impl std::fmt::Display for BoardAnalysis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(Final board: {:?}\nTotal score: {}\nBonus: {})",
            self.finalboard, self.total_score, self.bonus
        )
    }
}
