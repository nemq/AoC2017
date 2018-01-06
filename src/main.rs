use std::env::args;
extern crate calendar;

fn main() 
{
    if let Some(day) = args().nth(1) {
        match day.as_str() {
            "day_1" => {
                println!("day_1");
                println!("first_puzzle: {}", calendar::day_1::first_puzzle());
                println!("second_puzzle: {}",calendar::day_1::second_puzzle());
            },
            "day_2" => {
                println!("day_2");
                println!("first_puzzle: {}", calendar::day_2::first_puzzle());
                println!("second_puzzle: {}",calendar::day_2::second_puzzle());
            },
            "day_3" => {
                println!("day_3");
                println!("first_puzzle: {}", calendar::day_3::first_puzzle());
                println!("second_puzzle: {}", calendar::day_3::second_puzzle());
            },
            "day_4" => {
                println!("day_4");
                println!("first_puzzle: {}", calendar::day_4::first_puzzle());
                println!("second_puzzle: {}", calendar::day_4::second_puzzle());
            },
            "day_5" => {
                println!("day_5");
                println!("first_puzzle: {}", calendar::day_5::first_puzzle());
                println!("second_puzzle: {}", calendar::day_5::second_puzzle());
            },
            "day_6" => {
                println!("day_6");
                println!("first_puzzle: {}", calendar::day_6::first_puzzle());
                println!("second_puzzle: {}", calendar::day_6::second_puzzle());
            },
            "day_7" => {
                println!("day_7");
                println!("first_puzzle: {}", calendar::day_7::first_puzzle());
                println!("second_puzzle: {}", calendar::day_7::second_puzzle());
            },
            "day_8" => {
                println!("day_8");
                println!("first_puzzle: {}", calendar::day_8::first_puzzle());
                println!("second_puzzle: {}", calendar::day_8::second_puzzle());
            },
            "day_9" => {
                println!("day_9");
                println!("first_puzzle: {}", calendar::day_9::first_puzzle());
                println!("second_puzzle: {}", calendar::day_9::second_puzzle());
            },
            "day_10" => {
                println!("day_10");
                println!("first_puzzle: {}", calendar::day_10::first_puzzle());
                println!("second_puzzle: {}", calendar::day_10::second_puzzle());
            },
            "day_11" => {
                println!("day_11");
                println!("first_puzzle: {}", calendar::day_11::first_puzzle());
                println!("second_puzzle: {}", calendar::day_11::second_puzzle());
            },
            "day_12" => {
                println!("day_12");
                println!("first_puzzle: {}", calendar::day_12::first_puzzle());
                println!("second_puzzle: {}", calendar::day_12::second_puzzle());
            },
            "day_13" => {
                println!("day_13");
                println!("first_puzzle: {}", calendar::day_13::first_puzzle());
                println!("second_puzzle: {}", calendar::day_13::second_puzzle());
            },
            "day_14" => {
                println!("day_14");
                println!("first_puzzle: {}", calendar::day_14::first_puzzle());
                println!("second_puzzle: {}", calendar::day_14::second_puzzle());
            },
            "day_15" => {
                println!("day_15");
                println!("first_puzzle: {}", calendar::day_15::first_puzzle());
                println!("second_puzzle: {}", calendar::day_15::second_puzzle());
            },
            "day_16" => {
                println!("day_16");
                println!("first_puzzle: {}", calendar::day_16::first_puzzle());
                println!("second_puzzle: {}", calendar::day_16::second_puzzle());
            },
            "day_17" => {
                println!("day_17");
                println!("first_puzzle: {}", calendar::day_17::first_puzzle());
                println!("second_puzzle: {}", calendar::day_17::second_puzzle());
            },
            "day_18" => {
                println!("day_18");
                println!("first_puzzle: {}", calendar::day_18::first_puzzle());
                println!("second_puzzle: {}", calendar::day_18::second_puzzle());
            },
            "day_19" => {
                println!("day_19");
                println!("first_puzzle: {}", calendar::day_19::first_puzzle());
                println!("second_puzzle: {}", calendar::day_19::second_puzzle());
            },
            "day_20" => {
                println!("day_20");
                println!("first_puzzle: {}", calendar::day_20::first_puzzle());
                println!("second_puzzle: {}", calendar::day_20::second_puzzle());
            },
            _ => {
                println!("Unrecognized day: {}", day);
            }
        }
    }
    else {
        println!("Usage: calendar.exe day");
    }
}