// Copyright 2023 Sharif Abdullahi.
// Use of this source code is governed by The Unlicense
// which can be found in the LICENSE file.
use std::{
    io::{stdin, stdout, Write},
    thread,
    time::Duration,
};

use device_query::{DeviceQuery, DeviceState, Keycode};
use rand::{thread_rng, Rng};

#[derive(Debug)]
struct Lives {
    current_lives: u32,
    max_lives: u32
}

impl Lives {
    fn reset(&mut self) {
        self.current_lives = self.max_lives;
    }
}

const STANDARD_LIVES: Lives = Lives { current_lives: 1, max_lives: 1 };

fn main() {
    let mut player: Lives = STANDARD_LIVES;
    let mut enemy: Lives = STANDARD_LIVES;

    let mut done: bool = false;

    while !done {
        clear();
        println!("Welcome To Shadowboxing");
        thread::sleep(Duration::from_secs(2));

        done = cont(arena(&mut player, &mut enemy));

        if !done {
            player.reset();
            enemy.reset();
        }
    }
}

fn listen_to_keyboard(player_turn: bool) -> String {
    let listener: DeviceState = DeviceState::new();
    let mut prev_keys: Vec<Keycode> = vec![];

    loop {
        let keys: Vec<Keycode> = listener.get_keys();

        if keys != prev_keys && !keys.is_empty() {
            if player_turn {
                println!("\nYou Pointed: [{}]", keys[0]);
            } else {
                println!("\nYou Looked: [{}]", keys[0]);
            }

            if keys.len() == 1 {
                match keys.get(0).unwrap_or(&Keycode::Space) {
                    Keycode::Up | Keycode::W => break "Up".to_string(),
                    Keycode::Down | Keycode::S => break "Down".to_string(),
                    Keycode::Left | Keycode::A => break "Left".to_string(),
                    Keycode::Right | Keycode::D => break "Right".to_string(),
                    _ => {}
                }
            }

            prev_keys = keys;
        }
    }
}

fn arena(player: &mut Lives, enemy: &mut Lives) -> bool {
    let mut player_turn: bool = true;
    let mut rounds = 1;

    while player.current_lives > 0 && enemy.current_lives > 0 {
        clear();

        println!("\n[Round {rounds}]\n");
        println!("Your Lives: {}\n\nOpponent Lives: {}\n", player.current_lives, enemy.current_lives);

        println!(
            "Press the direction you'll {} (Press an Arrow/WASD key.)...",
            if player_turn {
                "ATTACK from!"
            } else {
                "DODGE to!"
            }
        );

        let poss_keys: [Keycode; 8] = [
            Keycode::Up, Keycode::Left, Keycode::Down, Keycode::Right,
            Keycode::W, Keycode::A, Keycode::S, Keycode::D,
        ];

        let player_dir: String = listen_to_keyboard(player_turn);
        let enemy_dir: String = poss_keys[thread_rng().gen_range(0..=3)].to_string();

        if player_turn {
            println!("Your Opponent Looked: [{enemy_dir}]\n");
        } else {
            println!("Your Opponent Pointed: [{enemy_dir}]\n");
        }

        if player_dir == enemy_dir {
            let (x, y) = if player_turn {
                enemy.current_lives -= 1;

                println!(" ▄▄▄▄    ▄▄▄       ███▄    █   ▄████  ▐██▌");
                println!("▓█████▄ ▒████▄     ██ ▀█   █  ██▒ ▀█▒ ▐██▌");
                println!("▒██▒ ▄██▒██  ▀█▄  ▓██  ▀█ ██▒▒██░▄▄▄░ ▐██▌");
                println!("▒██░█▀  ░██▄▄▄▄██ ▓██▒  ▐▌██▒░▓█  ██▓ ▓██▒");
                println!("░▓█  ▀█▓ ▓█   ▓██▒▒██░   ▓██░░▒▓███▀▒ ▒▄▄ ");
                println!("░▒▓███▀▒ ▒▒   ▓▒█░░ ▒░   ▒ ▒  ░▒   ▒  ░▀▀▒");
                println!("▒░▒   ░   ▒   ▒▒ ░░ ░░   ░ ▒░  ░   ░  ░  ░");
                println!(" ░    ░   ░   ▒      ░   ░ ░ ░ ░   ░     ░");
                println!(" ░            ░  ░         ░       ░  ░   ");
                println!("      ░                                   ");

                ("YOUR", "YOUR")
            } else {
                player.current_lives -= 1;

                println!(" ▄▄▄▄    ▒█████   ▒█████   ███▄ ▄███▓ ▐██▌");
                println!("▓█████▄ ▒██▒  ██▒▒██▒  ██▒▓██▒▀█▀ ██▒ ▐██▌");
                println!("▒██▒ ▄██▒██░  ██▒▒██░  ██▒▓██    ▓██░ ▐██▌");
                println!("▒██░█▀  ▒██   ██░▒██   ██░▒██    ▒██  ▓██▒");
                println!("░▓█  ▀█▓░ ████▓▒░░ ████▓▒░▒██▒   ░██▒ ▒▄▄ ");
                println!("░▒▓███▀▒░ ▒░▒░▒░ ░ ▒░▒░▒░ ░ ▒░   ░  ░ ░▀▀▒");
                println!("▒░▒   ░   ░ ▒ ▒░   ░ ▒ ▒░ ░  ░      ░ ░  ░");
                println!(" ░    ░ ░ ░ ░ ▒  ░ ░ ░ ▒  ░      ░       ░");
                println!(" ░          ░ ░      ░ ░         ░    ░   ");
                println!("      ░                                   ");

                ("THEY", "THEIR")
            };

            println!("\n{x} LOST A LIFE!\n NOW IT'S {y} TURN");
        } else {
            println!(" █     █░ ██▓  █████▒ █████▒▓█████ ▓█████▄               ");
            println!("▓█░ █ ░█░▓██▒▓██   ▒▓██   ▒ ▓█   ▀ ▒██▀ ██▌              ");
            println!("▒█░ █ ░█ ▒██▒▒████ ░▒████ ░ ▒███   ░██   █▌              ");
            println!("░█░ █ ░█ ░██░░▓█▒  ░░▓█▒  ░ ▒▓█  ▄ ░▓█▄   ▌              ");
            println!("░░██▒██▓ ░██░░▒█░   ░▒█░    ░▒████▒░▒████▓  ██▓  ██▓  ██▓");
            println!("░ ▓░▒ ▒  ░▓   ▒ ░    ▒ ░    ░░ ▒░ ░ ▒▒▓  ▒  ▒▓▒  ▒▓▒  ▒▓▒");
            println!("  ▒ ░ ░   ▒ ░ ░      ░       ░ ░  ░ ░ ▒  ▒  ░▒   ░▒   ░▒ ");
            println!("  ░   ░   ▒ ░ ░ ░    ░ ░       ░    ░ ░  ░  ░    ░    ░  ");
            println!("    ░     ░                    ░  ░   ░      ░    ░    ░ ");
            println!("                                    ░        ░    ░    ░ ");

            println!("\n{} MISSED!", if player_turn { "YOU" } else { "OPPONENT" });
        }

        thread::sleep(Duration::from_secs(1));

        rounds += 1;

        player_turn = !player_turn;
    }

    if player.current_lives > 0 {
        return true;
    }

    false
}

fn read_input(query: &'static str, new_line: bool) -> String {
    let mut binder: String = String::new();

    print!("{query}{}", if new_line { "\n" } else { "" });
    stdout().flush().expect("SysErr: Could Not Flush [STDOUT]");

    stdin()
        .read_line(&mut binder)
        .expect("SysErr: Could Not Read [STDIN]");

    binder
}

fn cont(won: bool) -> bool {
    loop {
        clear();

        println!("You {}!", if won { "WON" } else { "LOST" });
        let response: String =
            read_input("Do you want to reset (yes/si or no)? ", false).trim().to_lowercase();

        println!("You chose: {response}");

        match response.as_str() {
            "yes" | "y" | "si" | "s" => break false,
            "no" | "n" => break true,
            _ => continue,
        }
    }
}

fn clear() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    stdout().flush().expect("SysErr: Could Not Flush [STDOUT]");
}
