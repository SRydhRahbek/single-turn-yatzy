/*
TODO: map and store each hand to a point value in each category; then you can plug in which categories aren't filled and get the best point value in hopefully less than 8ms/lookup and keep this throughout the whole runtime
Value by turning MaskhandKeys to subsets, which can be valued and mapped, to save yet a few more calculations
*/

mod algorithms;
mod game_player;
mod logic;

use fraction::GenericFraction;
//use std::collections::HashMap;
//use std::path::Path;
type F = GenericFraction<u32>;
use crate::logic::{Board, Hand, Maskhand}; //Category, Mask, MaskhandKey
                                           //use fraction::convert::TryToConvertFrom;
use rayon::prelude::*;
use std::time::Instant;

use std::fs::{File, OpenOptions};
use std::io::Write;

fn main() {
    let games_per_update: u32 = 1000;
    let updates: u32 = 1;

    //###################################################################################

    let start_time = Instant::now();
    println!("Generating hashmaps...\nStart time: {:?}", start_time);

    let maskhandmap = Maskhand::get_maskhandmap(); //generate the expensive Maskhandmap
                                                   /*



                                                   *///PLAYING GAMES

    let t_1 = Instant::now();
    println!("Time taken: {:?}\nSimulating games...", t_1 - start_time);

    for i in 1..=updates {
        let output_vec: Vec<String> = (0..games_per_update)
            .into_par_iter()
            .map(|_x| game_player::play_game(maskhandmap.clone()))
            .collect();
        //.for_each(|_x| game_player::play_game(maskhandmap.clone()));

        println!(
            "{} out of {} games played. Sharing results...\nTime taken: {:?}",
            i * games_per_update,
            updates * games_per_update,
            Instant::now() - t_1
        );

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("data.txt")
            .unwrap();
        let _ = write_to_file(output_vec, file); //Writes the result to simulated parties into "data.txt"
                                                 //Will I get punished by my cheap error handling?
    }
    /*





    */

    let end_time = Instant::now();
    println!(
        "All done. End time: {:?}\nTotal duration: {:?}\nTime per party: {:?}",
        end_time,
        end_time - start_time,
        (end_time - t_1) / (updates * games_per_update)
    );
}

fn write_to_file(vec: Vec<String>, mut file: File) -> std::io::Result<()> {
    writeln!(file, "Ones\tTwos\tThrees\tFours\tFives\tSixes\tOne pair\tTwo pairs\tThree of a kind\tFour of a kind\tSmall straight\tLarge straight\tFull House\tChance\tYatzy\tBonus\tTotal");
    vec.iter().for_each(|f| {
        writeln!(file, "{}", f);
    });
    Ok(())
}
