/*
TODO: map and store each hand to a point value in each category; then you can plug in which categories aren't filled and get the best point value in hopefully less than 8ms/lookup and keep this throughout the whole runtime
Value by turning MaskhandKeys to subsets, which can be valued and mapped, to save yet a few more calculations
*/

mod algorithms;
mod game_player;
mod logic;
use itertools::Itertools;
//use std::collections::HashMap;
//use std::path::Path;
//type F = GenericFraction<u32>;
use crate::logic::{Board, Hand, Maskhand, StraightData, Category, GameData}; //Category, Mask, MaskhandKey
                                           //use fraction::convert::TryToConvertFrom;
use rayon::prelude::*;
use std::time::Instant;

use std::fs::{File, OpenOptions};
use std::io::Write;

fn main() {
    let games_per_update: u32 = 5;
    let updates: u32 = 1;

    //###################################################################################

    let start_time = Instant::now();
    println!("Generating hashmaps...\nStart time: {:?}", start_time);

    let maskhandmap = Maskhand::get_maskhandmap(); //generate the expensive Maskhandmap

    let t_1 = Instant::now();
    println!("Time taken: {:?}\nSimulating games...", t_1 - start_time);

    for i in 1..=updates {
        let output_doublevec: Vec<(String, Vec<StraightData>, GameData)> = (0..games_per_update)
            .into_par_iter()
            .map(|_x| game_player::play_game(maskhandmap.clone()))
            .collect();
        //.for_each(|_x| game_player::play_game(maskhandmap.clone()));
        let (output_vec, straightdata_unflat_vec, game_data_vec): (Vec<String>, Vec<Vec<StraightData>>, Vec<GameData>) = output_doublevec.into_iter().multiunzip();
        let straightdata_flat_vec = straightdata_unflat_vec.into_iter().flatten().collect::<Vec<StraightData>>();
        println!(
            "{} out of {} games played. Sharing results...\nTime taken: {:?}",
            i * games_per_update,
            updates * games_per_update,
            Instant::now() - t_1
        );

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("turn final board data.txt")
            .unwrap();
        let _ = write_game_data_boards_to_file(game_data_vec.clone(), file);

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("turn data.txt")
            .unwrap();
        let _ = write_game_data_to_file(game_data_vec, file);

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("straight choices.txt")
            .unwrap();
        let _ = write_straight_stuff_to_file(straightdata_flat_vec, file);

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("data.txt")
            .unwrap();
        let _ = write_to_file(output_vec, file); //Writes the result to simulated parties into "data.txt"
                                                 //Will I get punished by my cheap error handling?
    }

    let end_time = Instant::now();
    println!(
        "All done. End time: {:?}\nTotal duration: {:?}\nTime per party: {:?}",
        end_time,
        end_time - start_time,
        (end_time - t_1) / (updates * games_per_update)
    );
}

fn write_game_data_to_file(game_data_vec: Vec<GameData>, mut file: File) ->  std::io::Result<()> {
    writeln!(file, "Game UUID\tTurn UUID\tTurn Number\tHand 1\tSubset 1\tHand 2\tSubset 2\tHand 3\tPoints\tPlaced Category\tEttor\tTvåor\tTreor\tFyror\tFemor\tSexor\tPar\tTvå par\tTretal\tFyrtal\tLiten stege\tStor stege\tKåk\tChans\tYatzy\tStraight State").expect("writeln function messed up in write_game_data_to_file");
    for game_data in game_data_vec {
        game_data.turns.iter().for_each(|turn| {
            writeln!(file, "{}\t{}\t{}\t{}\t{:?}\t{}\t{:?}\t{}\t{}\t{:?}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}", game_data.game_uuid, turn.turn_uuid, turn.turn_number, turn.hand1, turn.subset1, turn.hand2, turn.subset2, turn.hand3, turn.points, turn.placed_category, turn.empty(Category::Ettor), turn.empty(Category::Tvaor), turn.empty(Category::Treor), turn.empty(Category::Fyror), turn.empty(Category::Femmor), turn.empty(Category::Sexor), turn.empty(Category::Par), turn.empty(Category::Tvapar), turn.empty(Category::Tretal), turn.empty(Category::Fyrtal), turn.empty(Category::LitenStraight), turn.empty(Category::StorStraight), turn.empty(Category::Kak), turn.empty(Category::Chans), turn.empty(Category::Yatzy), turn.straight_state).expect("writeln function messed up in write_game_data_to_file");
        });
    }

    Ok(())
}

fn write_game_data_boards_to_file(game_data_vec: Vec<GameData>, mut file: File) -> std::io::Result<()> {
    writeln!(file, "UUID\tOnes\tTwos\tThrees\tFours\tFives\tSixes\tOne pair\tTwo pairs\tThree of a kind\tFour of a kind\tSmall straight\tLarge straight\tFull House\tChance\tYatzy\tBonus\tTotal").expect("writeln function messed up in write_game_data_boards_to_file");
    for game_data in game_data_vec {
        write!(file, "{}", game_data.game_uuid).expect("writeln function messed up in write_game_data_boards_to_file");
        for category in Category::all_categories() {
            let points = game_data.final_board.get(category).unwrap();
            write!(file, "\t{}", points).expect("writeln function messed up in write_game_data_boards_to_file");
        }
        write!(file, "\n").expect("writeln function messed up in write_game_data_boards_to_file");
    }

    Ok(())
}

fn write_straight_stuff_to_file(vec: Vec<StraightData>, mut file: File) ->  std::io::Result<()> {
    writeln!(file, "UUID\tHand\tSubset\tEmpty categories\tSlag index").expect("writeln function messed up in write_straight_stuff_to_file");
    vec.iter().for_each(|straight_thing| {
        writeln!(file, "{}\t{}\t{:?}\t{:?}\t{}", straight_thing.uuid, straight_thing.hand, straight_thing.subset, straight_thing.empty_categories, straight_thing.slag).expect("writeln function messed up in write_straight_stuff_to_file");
    });
    Ok(())
}

fn write_to_file(vec: Vec<String>, mut file: File) -> std::io::Result<()> {
    writeln!(file, "Ones\tTwos\tThrees\tFours\tFives\tSixes\tOne pair\tTwo pairs\tThree of a kind\tFour of a kind\tSmall straight\tLarge straight\tFull House\tChance\tYatzy\tBonus\tTotal").expect("writeln function messed up in write_to_file");
    vec.iter().for_each(|f| {
        writeln!(file, "{}", f).expect("writeln function messed up in write_to_file");
    });
    Ok(())
}