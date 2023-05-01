// Copyright 2023 Sharif Abdullahi.
// Use of this source code is governed by The Unlicense
// which can be found in the LICENSE file.
use std::{
    io::{stdout, Write},
    thread,
    time::Duration,
};

use device_query::{DeviceQuery, DeviceState, Keycode};
use rand::{thread_rng, Rng};

fn main() {
    let (mut player, mut enemy) = (2, 2);

    clear();

    println!("  ██████  ██░ ██  ▄▄▄      ▓█████▄  ▒█████   █     █░ ▄▄▄▄    ▒█████  ▒██   ██▒");
    println!("▒██    ▒ ▓██░ ██▒▒████▄    ▒██▀ ██▌▒██▒  ██▒▓█░ █ ░█░▓█████▄ ▒██▒  ██▒▒▒ █ █ ▒░");
    println!("░ ▓██▄   ▒██▀▀██░▒██  ▀█▄  ░██   █▌▒██░  ██▒▒█░ █ ░█ ▒██▒ ▄██▒██░  ██▒░░  █   ░");
    println!("  ▒   ██▒░▓█ ░██ ░██▄▄▄▄██ ░▓█▄   ▌▒██   ██░░█░ █ ░█ ▒██░█▀  ▒██   ██░ ░ █ █ ▒ ");
    println!("▒██████▒▒░▓█▒░██▓ ▓█   ▓██▒░▒████▓ ░ ████▓▒░░░██▒██▓ ░▓█  ▀█▓░ ████▓▒░▒██▒ ▒██▒");
    println!("▒ ▒▓▒ ▒ ░ ▒ ░░▒░▒ ▒▒   ▓▒█░ ▒▒▓  ▒ ░ ▒░▒░▒░ ░ ▓░▒ ▒  ░▒▓███▀▒░ ▒░▒░▒░ ▒▒ ░ ░▓ ░");
    println!("░ ░▒  ░ ░ ▒ ░▒░ ░  ▒   ▒▒ ░ ░ ▒  ▒   ░ ▒ ▒░   ▒ ░ ░  ▒░▒   ░   ░ ▒ ▒░ ░░   ░▒ ░");
    println!("░  ░  ░   ░  ░░ ░  ░   ▒    ░ ░  ░ ░ ░ ░ ▒    ░   ░   ░    ░ ░ ░ ░ ▒   ░    ░  ");
    println!("      ░   ░  ░  ░      ░  ░   ░        ░ ░      ░     ░          ░ ░   ░    ░  ");
    println!("                            ░                              ░                   ");
    thread::sleep(Duration::from_secs(2));

    let won = arena(&mut player, &mut enemy);

    clear();

    if won {
        println!(" █     █░ ██▓ ███▄    █  ███▄    █ ▓█████  ██▀███   ▐██▌ ");
        println!("▓█░ █ ░█░▓██▒ ██ ▀█   █  ██ ▀█   █ ▓█   ▀ ▓██ ▒ ██▒ ▐██▌ ");
        println!("▒█░ █ ░█ ▒██▒▓██  ▀█ ██▒▓██  ▀█ ██▒▒███   ▓██ ░▄█ ▒ ▐██▌ ");
        println!("░█░ █ ░█ ░██░▓██▒  ▐▌██▒▓██▒  ▐▌██▒▒▓█  ▄ ▒██▀▀█▄   ▓██▒ ");
        println!("░░██▒██▓ ░██░▒██░   ▓██░▒██░   ▓██░░▒████▒░██▓ ▒██▒ ▒▄▄  ");
        println!("░ ▓░▒ ▒  ░▓  ░ ▒░   ▒ ▒ ░ ▒░   ▒ ▒ ░░ ▒░ ░░ ▒▓ ░▒▓░ ░▀▀▒ ");
        println!("  ▒ ░ ░   ▒ ░░ ░░   ░ ▒░░ ░░   ░ ▒░ ░ ░  ░  ░▒ ░ ▒░ ░  ░ ");
        println!("  ░   ░   ▒ ░   ░   ░ ░    ░   ░ ░    ░     ░░   ░     ░ ");
        println!("    ░     ░           ░          ░    ░  ░   ░      ░    ");
    } else {
        println!(" ██▓     ▒█████    ██████ ▓█████  ██▀███   ▐██▌ ");
        println!("▓██▒    ▒██▒  ██▒▒██    ▒ ▓█   ▀ ▓██ ▒ ██▒ ▐██▌ ");
        println!("▒██░    ▒██░  ██▒░ ▓██▄   ▒███   ▓██ ░▄█ ▒ ▐██▌ ");
        println!("▒██░    ▒██   ██░  ▒   ██▒▒▓█  ▄ ▒██▀▀█▄   ▓██▒ ");
        println!("░██████▒░ ████▓▒░▒██████▒▒░▒████▒░██▓ ▒██▒ ▒▄▄  ");
        println!("░ ▒░▓  ░░ ▒░▒░▒░ ▒ ▒▓▒ ▒ ░░░ ▒░ ░░ ▒▓ ░▒▓░ ░▀▀▒ ");
        println!("░ ░ ▒  ░  ░ ▒ ▒░ ░ ░▒  ░ ░ ░ ░  ░  ░▒ ░ ▒░ ░  ░ ");
        println!("  ░ ░   ░ ░ ░ ▒  ░  ░  ░     ░     ░░   ░     ░ ");
        println!("    ░  ░    ░ ░        ░     ░  ░   ░      ░    ");
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

fn arena(player: &mut u32, enemy: &mut u32) -> bool {
    let mut player_turn: bool = true;
    let mut rounds = 1;

    while *player > 0 && *enemy > 0 {
        clear();

        println!("\n[Round {rounds}]\n");
        println!("Your Lives: {}\n\nOpponent Lives: {}\n", player, enemy);

        println!(
            "Press the direction you'll {} (Press an Arrow/WASD key.)...",
            if player_turn {
                "ATTACK from!"
            } else {
                "DODGE to!"
            }
        );

        let poss_keys: [Keycode; 8] = [
            Keycode::Up,
            Keycode::Left,
            Keycode::Down,
            Keycode::Right,
            Keycode::W,
            Keycode::A,
            Keycode::S,
            Keycode::D,
        ];

        let player_dir: String = listen_to_keyboard(player_turn);
        let enemy_dir: String = poss_keys[thread_rng().gen_range(0..=3)].to_string();

        if player_turn {
            println!("Your Opponent Looked: [{enemy_dir}]\n");
        } else {
            println!("Your Opponent Pointed: [{enemy_dir}]\n");
        }

        if player_dir == enemy_dir {
            let victim = if player_turn {
                *enemy -= 1;

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

                "OPPONENT"
            } else {
                *player -= 1;

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

                "YOU"
            };

            println!("\n{victim} LOST A LIFE!");
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

        thread::sleep(Duration::from_secs_f64(2.3));

        rounds += 1;

        player_turn = !player_turn;
    }

    if *player > 0 {
        return true;
    }

    false
}

fn clear() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    stdout().flush().expect("SysErr: Could Not Flush [STDOUT]");
}
