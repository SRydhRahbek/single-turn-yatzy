use fraction::GenericFraction;
use std::collections::HashMap;
use uuid::Uuid;
//use std::sync::Arc;
type F = GenericFraction<u32>;
use crate::logic::{Board, Category, Hand, Mask, Maskhand, MaskhandKey, is_subset_straight, StraightData};
//use fraction::convert::TryToConvertFrom;
use std::fmt;

pub fn play_game(maskhandmap: HashMap<MaskhandKey, Maskhand>) -> (String, Vec<StraightData>) {
    //Initialize the game
    let empty_return_mask = Mask::empty();
    let all_hands = Hand::all_hands();

    let mut board = Board::zero_chance(); // S:   Create a board with a 0 in 'Chance' with Board::zero_chance()

    //HashMap to match each hand in the final step to it's point value in each caregory
    let mut final_step_evalmap: HashMap<&Hand, HashMap<Category, u32>> = HashMap::new();

    //Add stuff to aforementioned hashmap
    for hand in &all_hands {
        final_step_evalmap.insert(hand, hand.evaluate_and_stack());
    }

    //TODO: input the actual order in which we want to cross out the categories
    let crossout_order_list = Category::all_categories();
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
    let david_crossout_order_list: Vec<Category> = vec![
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
    ];

    let mut empty_categories = board.empty_categories();
    let mut straight_data_vec: Vec<StraightData> = Vec::new();
    loop {
        let step3_best_evalmap = get_final_step_best_evalmap(
            &empty_categories,
            &all_hands,
            &final_step_evalmap,
            &empty_return_mask,
        );
        let step2_evalmap = get_next_evalmap(&maskhandmap, &step3_best_evalmap);
        let step2_best_evalmap = transform_to_best_evalmap(&maskhandmap, step2_evalmap);
        let step1_evalmap = get_next_evalmap(&maskhandmap, &step2_best_evalmap);
        let step1_best_evalmap = transform_to_best_evalmap(&maskhandmap, step1_evalmap);
        let uuid = format!("{}", Uuid::now_v7());
        let mut hand = Hand::random();
        let best_mask = step1_best_evalmap
            .get(&hand)
            .expect("step1 best doesn't contain all hands")
            .0;

        let maskhand_to_subset = MaskhandKey::from(hand.clone(), *best_mask);
        let mut subset = maskhand_to_subset.merge_to_subset();
        if is_subset_straight(&mut subset) {
            straight_data_vec.push(StraightData::new(hand.clone(), subset, 1, empty_categories.clone(), uuid.clone()))
        }

        hand.reroll_with_mask(&best_mask);
        let best_mask = step2_best_evalmap
            .get(&hand)
            .expect("step2 best doesn't contain all hands")
            .0;

        let maskhand_to_subset = MaskhandKey::from(hand.clone(), *best_mask);
        let mut subset = maskhand_to_subset.merge_to_subset();
        if is_subset_straight(&mut subset) {
            straight_data_vec.push(StraightData::new(hand.clone(), subset, 2, empty_categories.clone(), uuid.clone()))
        }

        hand.reroll_with_mask(&best_mask);
        if is_subset_straight(&mut hand.0.clone()) {
            straight_data_vec.push(StraightData::new(hand.clone(), hand.0.clone(), 3, empty_categories.clone(), uuid.clone()));
        }
        //println!("{hand:?}");
        let hand_value_category = hand.evaluate(&board);


        if let Some(category) = hand_value_category.1 {
            board.place_value_in_category(hand_value_category.0, category);
            empty_categories = empty_categories
                .into_iter()
                .filter(|x| x != &category)
                .collect();
        } else {
            for category in &david_crossout_order_list {
                // can switch crossout order list
                if empty_categories.contains(category) {
                    board.place_value_in_category(0, *category);
                    empty_categories = empty_categories
                        .into_iter()
                        .filter(|x| x != category)
                        .collect();
                }
            }
        }
        if empty_categories.len() == 0 {
            break;
        }
    }
    let analysis: BoardAnalysis = BoardAnalysis::new(&board);

    //println!("Board: {}", board);
    //println!("{}", analysis);

    let analysis_string: String = format!(
        "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
        analysis.finalboard[&Category::Ettor],
        analysis.finalboard[&Category::Tvaor],
        analysis.finalboard[&Category::Treor],
        analysis.finalboard[&Category::Fyror],
        analysis.finalboard[&Category::Femmor],
        analysis.finalboard[&Category::Sexor],
        analysis.finalboard[&Category::Par],
        analysis.finalboard[&Category::Tvapar],
        analysis.finalboard[&Category::Tretal],
        analysis.finalboard[&Category::Fyrtal],
        analysis.finalboard[&Category::LitenStraight],
        analysis.finalboard[&Category::StorStraight],
        analysis.finalboard[&Category::Kak],
        analysis.finalboard[&Category::Chans],
        analysis.finalboard[&Category::Yatzy],
        analysis.bonus,
        analysis.total_score
    );

    return (analysis_string, straight_data_vec);
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
    maskhandmap: &'a HashMap<MaskhandKey, Maskhand>,
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
