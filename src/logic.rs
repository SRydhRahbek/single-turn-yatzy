use itertools::Itertools;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::fmt;

//================================================================================
// Fraction
//================================================================================

use fraction::GenericFraction;
type F = GenericFraction<u32>;

//================================================================================
// StraightData
//================================================================================
#[derive(Debug)]
pub struct StraightData {
    pub hand: Hand,
    pub subset: Vec<u32>,
    pub slag: u32,
    pub empty_categories: Vec<Category>,
}

impl StraightData {
    pub fn new(hand: Hand, subset: Vec<u32>, slag: u32, empty_categories: Vec<Category>) -> StraightData {
        StraightData {
            hand,
            subset,
            slag,
            empty_categories,
        }       
    }
}


//================================================================================
// Category
//================================================================================

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Category {
    Ettor,
    Tvaor,
    Treor,
    Fyror,
    Femmor,
    Sexor,
    Par,
    Tretal,
    Fyrtal,
    Tvapar,
    LitenStraight,
    StorStraight,
    Kak,
    Chans,
    Yatzy,
}

//Mortals: Funktioner för Category.
impl Category {
    //Mortals: den här funktionen spottar ur sig "en påse med en av varje sorts bröd i".
    pub fn all_categories() -> Vec<Category> {
        return vec![
            Category::Ettor,
            Category::Tvaor,
            Category::Treor,
            Category::Fyror,
            Category::Femmor,
            Category::Sexor,
            Category::Par,
            Category::Tretal,
            Category::Fyrtal,
            Category::Tvapar,
            Category::LitenStraight,
            Category::StorStraight,
            Category::Kak,
            Category::Chans,
            Category::Yatzy,
        ];
    }
}

//================================================================================
// Board
//================================================================================

//          S: Board is hashmap of category:points
//             Starts as category: None

#[derive(Debug, Clone)]
//TODO: consider replacing with just bools - we should probably just be interested in whether they are filled at all
pub struct Board(pub HashMap<Category, Option<u32>>);

impl Board {
    //Mortals: Den första vi skapar är new(). Den gör att vi enkelt kan skapa ett nytt helt tomt bräde (ett som bara innehåller lediga fält None), genom Board::new()
    pub fn new() -> Board {
        let mut new_board: HashMap<Category, Option<u32>> = HashMap::new();
        for category in Category::all_categories() {
            new_board.insert(category, None);
        }
        return Board(new_board);
    }

    //Mortals: Andra funktionen är zero_chance. Den är till för att jag ska kunna fiffla runt med vad som händer om jag stryker fält. Just här har jag strukit chans.
    pub fn zero_chance() -> Board {
        let mut board = Board::new();
        board.0.insert(Category::Chans, Some(0));
        return board;
    }

    pub fn new_only(cat: Category) -> Board {
        let mut board = Board::new();
        for category in Category::all_categories() {
            if category == cat {
                continue;
            }
            board.0.insert(category, Some(0));
        }
        return board;
    }

    // S:   Check if supplied Category is empty
    pub fn category_empty(&self, category: Category) -> bool {
        self.0
            .get(&category)
            .expect("Board is missing a category")
            .is_none()
    }

    // S:   Puts all empty categories in a vector and returns it
    pub fn empty_categories(&self) -> Vec<Category> {
        let mut empty_categories = Vec::new();
        for (category, value) in self.0.iter() {
            if value.is_none() {
                empty_categories.push(*category);
            }
        }
        return empty_categories;
    }

    // S:   Places a value in specified category if empty
    pub fn place_value_in_category(&mut self, value: u32, category: Category) {
        if !self.category_empty(category) {
            panic!("Trying to place value in already filled category!");
        }
        self.0.insert(category, Some(value)); // S: Placing value at first index in vec?
    }

    pub fn get(&self, category: Category) -> &Option<u32> {
        self.0.get(&category).unwrap()
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "
        Ones: {:?}
        Twos: {:?}
        Threes: {:?}
        Fours: {:?}
        Fives: {:?}
        Sixes: {:?}
        One pair: {:?}
        Two pairs: {:?}
        Three of a kind: {:?}
        Four of a kind: {:?}
        Small straight: {:?}
        Large straight: {:?}
        Full house: {:?}
        Chance: {:?}
        Yatzy: {:?}",
            self.0[&Category::Ettor].unwrap(),
            self.0[&Category::Tvaor].unwrap(),
            self.0[&Category::Treor].unwrap(),
            self.0[&Category::Fyror].unwrap(),
            self.0[&Category::Femmor].unwrap(),
            self.0[&Category::Sexor].unwrap(),
            self.0[&Category::Par].unwrap(),
            self.0[&Category::Tvapar].unwrap(),
            self.0[&Category::Tretal].unwrap(),
            self.0[&Category::Fyrtal].unwrap(),
            self.0[&Category::LitenStraight].unwrap(),
            self.0[&Category::StorStraight].unwrap(),
            self.0[&Category::Kak].unwrap(),
            self.0[&Category::Chans].unwrap(),
            self.0[&Category::Yatzy].unwrap(),
        )
    }
}

//================================================================================
// Hand
//================================================================================

#[derive(Eq, Hash, PartialEq, PartialOrd, Debug, Clone)]
pub struct Hand(pub Vec<u32>);
// S:   Hand is ex. <1,2,4,4,6>

impl Hand {
    // S: Returns the hand <0,0,0,0,0>
    pub fn empty() -> Hand {
        return Hand(vec![0; 5]);
    }

    // S:   Roll new hand
    pub fn random() -> Hand {
        let mut hand = Hand::empty();
        hand.reroll_with_mask(&Mask::empty()); // Rerolls empty hand with empty mask
        return hand;
    }

    // S:   Makes a vector a hand? Rather, assigns the type 'Hand' to a vector?
    pub fn from_vec(v: Vec<u32>) -> Hand {
        return Hand(v);
    }

    // S: Vector with all 252 possible hands, probably ordered <1,1,1,1,1> to <6,6,6,6,6>, stored in vector
    pub fn all_hands() -> Vec<Hand> {
        let all_hands = (1..=6)
            .combinations_with_replacement(5)
            .map(|h| Hand::from_vec(h))
            .collect::<Vec<Hand>>();
        return all_hands;
    }

    //Takes a hand, combines it with a mask, rerolls dice accordingly, sorts it, and updates the input-hand
    //Only used in simulations; not used in theoretical calculations
    pub fn reroll_with_mask(&mut self, mask: &Mask) {
        let mut rng = thread_rng();
        for (index, hold) in mask.to_array().iter().enumerate() {
            if !*hold {
                self.0[index] = rng.gen_range(1..=6);
            }
        }
        self.0.sort();
    }

    // S:   Counts how many of a requested dice-value is in hand
    pub fn count_instances(&self, requested_die: u32) -> u32 {
        let mut count = 0;
        for x in &self.0 {
            if x == &requested_die {
                count += 1;
            }
        }
        return count;
    }
}

//Describes how Hand should look when printed in the terminal
impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}, {}, {}, {}, {})",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4]
        )
    }
}

//================================================================================
// Mask
//================================================================================

#[derive(Clone, Debug, Hash, PartialEq, Eq, Copy)]
pub struct Mask(bool, bool, bool, bool, bool); //true = hold; false = reroll

impl Mask {
    // S: makes a new mask from manually assigning each position ---- is this a kind of standard function?
    pub fn new(a: bool, b: bool, c: bool, d: bool, e: bool) -> Mask {
        Mask(a, b, c, d, e)
    }

    // S:   Makes a new empty mask that says "reroll all"
    pub fn empty() -> Mask {
        return Mask(false, false, false, false, false);
    }

    //It works, trust me bro
    // S:   Can't this be done by using the binary 0 through 32?
    //      Returns a vector of all (32) masks
    pub fn all_masks() -> Vec<Mask> {
        return (1..=5)
            .map(|_| [true, false])
            .multi_cartesian_product()
            .map(|x| Mask::new(x[0], x[1], x[2], x[3], x[4]))
            .collect();
    }

    // S: Transform tuple Mask to array Mask
    pub fn to_array(&self) -> [bool; 5] {
        [self.0, self.1, self.2, self.3, self.4]
    }
}

//Describes how Mask should look when printed in the terminal
impl fmt::Display for Mask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}, {}, {}, {}, {}]",
            self.0, self.1, self.2, self.3, self.4
        )
    }
}

//Declares that a Mask can be created from 5 bools
impl From<[bool; 5]> for Mask {
    fn from(boolarr: [bool; 5]) -> Self {
        Mask(boolarr[0], boolarr[1], boolarr[2], boolarr[3], boolarr[4])
    }
}

//================================================================================
// Maskhand
//================================================================================

//The expansion of a MaskhandKey, containing a list of each hand that would be generated by the MaskhandKey, and how many times it appears in the expansion
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Maskhand(pub HashMap<Hand, u32>);

// S: should be calles 'Outcomes' or such
impl Maskhand {
    //Maps and stores each MaskhandKey to it's corresponding expansion
    //This associated function is expensive as shit to run (≈10s), and should thus only be ran once per program
    pub fn get_maskhandmap() -> HashMap<MaskhandKey, Maskhand> {
        println!("THIS CODE SHOULD BE CALLED ONCE ONLY");
        let mut maskhandmap = HashMap::new();
        let mut all_maskhandkeys = MaskhandKey::all_maskhandkeys();
        for maskhandkey in all_maskhandkeys {
            let maskhand = maskhandkey.expand();
            maskhandmap.insert(maskhandkey, maskhand); //TODO: efficiency here?
        }
        return maskhandmap; // hashmap<key, outcomes of key>, outcomes of key = MaskHand, i.e <Hand, amount>
                            // outcomes = <key,<outcomehand, amount>>
    }

    //Creates a maskhand from a hashmap of hands and instances
    pub fn wrap(compressed_map: HashMap<Hand, u32>) -> Maskhand {
        return Maskhand(compressed_map);
    }

    //Maskhands are compressed by definition, to save computations; this method calculates the decompressed length by summing how many instances there are of each hand
    pub fn decompressed_len(&self) -> u32 {
        self.0.values().sum()
    }

    //Currently unused
    pub fn _decompress() {
        todo!();
    }

    // S: Values the maskhand/outcomes based on the current board state, assumes no rerolls
    // Never used
    pub fn evaluate_maskhand(&self, board: &Board) -> F {
        let mut running_count = 0;
        for (hand, instances) in self.0.iter() {
            let (raw_evalutaion, _) = hand.evaluate(board);
            let weighted_evaluation = instances * raw_evalutaion;
            running_count += weighted_evaluation;
        }
        let maskhand_value = F::new(running_count, self.decompressed_len() as u32);
        return maskhand_value;
    }

    //We take in a maskhand and current_best_evalmap, being a map describng the best value for each hand in the current step,
    //and using these two we calculate the weighted value of the maskhand in the current step
    pub fn evaluate_maskhand_against_map<'a>(
        &self,
        current_best_evalmap: &'a HashMap<&'a Hand, (&'a Mask, F)>,
    ) -> F {
        let mut running_count = F::from(0);
        for (hand, instances) in self.0.iter() {
            let (_, raw_evalutaion) = current_best_evalmap
                .get(hand)
                .expect("Somehow current_step_best_evalmap is missing a hand");
            let weighted_evaluation = F::from(*instances) * raw_evalutaion.clone(); //Might be possible to use regular ints here?
            running_count += weighted_evaluation;
        }
        let maskhand_value = running_count / (self.decompressed_len() as u32);

        return maskhand_value;
    }
}

//================================================================================
// MaskhandKey
//================================================================================

//A simple combination of a hand and a mask. Can be expanded into a Maskhand, to get the full list of hands and how many of them would be generated if the mask were to be applied to the hand
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct MaskhandKey {
    pub hand: Hand,
    pub mask: Mask,
}

pub fn is_subset_straight(subset: &mut Vec<u32>) -> bool {
    if subset.len() < 3 {
        return false;
    }
    subset.sort();
    let mut duplicate_free_subset = subset.clone();
    duplicate_free_subset.dedup();
    if duplicate_free_subset.len() == subset.len() {
        return true;
    } else {
        return false;
    }
}

impl MaskhandKey {
    //Turns the MaskhandKey into a Subset - will be used later for finding out when the algorithm is going for straight
    pub fn merge_to_subset(&self) -> Vec<u32> {
        let mut subset = Vec::new();
        for (die, mask_hole) in self.hand.0.iter().zip(self.mask.to_array()) {
            if mask_hole {
                subset.push(*die);
            }
        }
        return subset;
    }

    //Function to create the MaskhandKey
    pub fn from(hand: Hand, mask: Mask) -> MaskhandKey {
        return MaskhandKey { hand, mask };
    }

    //Returns a list of all MaskhandKeys, with each combination of a mask hand a hand appearing only once
    pub fn all_maskhandkeys() -> Vec<MaskhandKey> {
        let mut all_maskhandkeys = Vec::new();
        for hand in Hand::all_hands() {
            for mask in Mask::all_masks() {
                let maskhandkey = MaskhandKey {
                    hand: hand.clone(), //TODO: reference here
                    mask,
                };
                all_maskhandkeys.push(maskhandkey);
            }
        }
        return all_maskhandkeys;
    }

    //Apply the mask to the hand of the MaskhandKey to get a Maskhand
    pub fn expand(&self) -> Maskhand {
        let mut compressed_map: HashMap<Hand, u32> = HashMap::new(); //TODO: implmenet Ord for hand and use above implementation

        let mut dice_metavec = Vec::new();

        let all_dice = Vec::from([1, 2, 3, 4, 5, 6]);
        for (index, dice) in self.hand.0.iter().enumerate() {
            if self.mask.to_array()[index] {
                dice_metavec.push(vec![*dice]);
            } else {
                dice_metavec.push(all_dice.clone());
            }
        }
        for d1 in &dice_metavec[0] {
            for d2 in &dice_metavec[1] {
                for d3 in &dice_metavec[2] {
                    for d4 in &dice_metavec[3] {
                        for d5 in &dice_metavec[4] {
                            let mut dicevec = vec![*d1, *d2, *d3, *d4, *d5];
                            dicevec.sort();
                            let hand = Hand(dicevec);
                            let instances = compressed_map.get(&hand).unwrap_or(&0);
                            compressed_map.insert(hand, instances + 1);
                        }
                    }
                }
            }
        }

        return Maskhand::wrap(compressed_map);
    }
}
