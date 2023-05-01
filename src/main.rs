// Copyright 2023 Sharif Abdullahi.
// Use of this source code is governed by The Unlicense
// which can be found in the LICENSE file.

// The Standard Library contains tools that base Rust
// has but you need to specifically say you're using.
use std::{
    // IO is Input/Output, it let's do things with
    // the terminal like reading input.
    io::{
        // Standard Output allows you to get 
        // information from and manipulate what you see
        // in the terminal.
        stdout,
        // Write is a trait let's you write things to stdout.
        // You don't need this for print! or println! because
        // they use it internally.
        Write 
    },
    // Threads are how many tasks your computer can do at the
    // same time, you can let you program have more than one.
    thread,
    // Time let's you manipulate program time and duration is
    // specifying a length of time.
    time::Duration,
};

// The Device Query Library lets you "listen" to keyboard input.
use device_query::{
    // DeviceQuery is a trait that lets you ask your computer
    // for information.
    DeviceQuery,
    // DeviceState (it depends on DeviceQuery) lets you capture
    // key presses and mouse movement.
    DeviceState,
    // Keycodes represent a key on your keyboard.
    Keycode
};

use rand::{
    // thread_rng lets you use your specific computer to get a
    // seed for random number generation.
    thread_rng,
    // Rng is a trait that allow you to manipulate numbers randomly.
    Rng
};

fn main() {
    // Clears the terminal.
    clear();

    // ASCII Art Reading "Shadowbox"
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

    // The arena function returns a boolean (true/false).
    let won: bool = arena();

    // Clears the terminal.
    clear();

    if won {
        // ASCII Art Reading "Winner!"
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
        // ASCII Art Reading "Loser!"
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

// Gets key presses from your computer and does something if you press
// arrow/WASD keys.
fn listen_to_keyboard(player_turn: bool) -> Keycode {
    // Variable to ask about changes in the mouse and keyboard.
    let listener: DeviceState = DeviceState::new();
    // Keys from a previous ask.
    let mut prev_keys: Vec<Keycode> = vec![];

    // Until the player presses a valid key. 
    loop {
        // Ask listener variable for the current keys being pressed.
        let keys: Vec<Keycode> = listener.get_keys();

        // If the keys being pressed are not 
        // the same ones as before AND you are pressing something.
        if keys != prev_keys && !keys.is_empty() {
            // Prints the keys you pressed with a different prefix based on 
            // if it's the player's turn.
            if player_turn {
                println!("\nYou Pointed: [{}]", keys[0]);
            } else {
                println!("\nYou Looked: [{}]", keys[0]);
            }

            // If the amount of keys being pressed is 1.
            if keys.len() == 1 {
                // Match the single key against the keys that we want to do something with.
                // Returns the direction
                match keys.get(0).unwrap_or(&Keycode::Space) {
                    Keycode::Up | Keycode::W => break Keycode::Up,
                    Keycode::Down | Keycode::S => break Keycode::Down,
                    Keycode::Left | Keycode::A => break Keycode::Left,
                    Keycode::Right | Keycode::D => break Keycode::Right,
                    _ => {}
                }
            }

            // Set the current keys as the previous keys.
            prev_keys = keys;
        }
    }
}

// The actual game.
fn arena() -> bool {
    // The Player's (You) lives.
    let mut player = 2;
    // The Opponent's lives.
    let mut opponent = 2;

    // If it's the players turn (always get's first turn).
    let mut player_turn: bool = true;

    // Amounts of round you've played for.
    let mut rounds = 1;

    // While both opponents have health.
    while player > 0 && opponent > 0 {
        // Clears the terminal.
        clear();

        // Prints round.
        println!("\n[Round {rounds}]\n");
        // Prints both lives.
        println!("Your Lives: {player}\n\nOpponent Lives: {opponent}\n");

        // Print directions with a different prefix based on if it's the player turn.
        println!(
            "Press the direction you'll {} (Press an Arrow/WASD key.)...",
            if player_turn {
                "ATTACK from!"
            } else {
                "DODGE to!"
            }
        );

        // Keys that the opponent can "press".
        let poss_keys: [Keycode; 4] = [
            Keycode::Up,
            Keycode::Left,
            Keycode::Down,
            Keycode::Right,
        ];

        // (Valid) Key that player pressed.
        let player_dir: Keycode = listen_to_keyboard(player_turn);
        // Gets a random Keycode from the poss_keys array.
        let enemy_dir: Keycode = poss_keys[thread_rng().gen_range(0..=3)];

        // Prints the keys you pressed with a different prefix based on 
        // if it's the player's turn.
        if player_turn {
            println!("Your Opponent Looked: [{enemy_dir}]\n");
        } else {
            println!("Your Opponent Pointed: [{enemy_dir}]\n");
        }

        // If player and opponent choose the same direction.
        if player_dir == enemy_dir {
            // Returns a string slice of who lost a life based on who's
            // turn it is.
            let victim = if player_turn {
                // Reduces opponent's lives by 1.
                opponent -= 1;

                // ASCII Art Reading "BANG!"
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

                // The string slice being returned.
                "OPPONENT"
            } else {
                // Reduces player's lives by 1.
                player -= 1;

                // ASCII Art Reading "BOOM!"
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

                // The string slice being returned.
                "YOU"
            };

            // Print who list a life.
            println!("\n{victim} LOST A LIFE!");
        } else {
            // ASCII Art Reading "MISSED..."
            println!(" ███▄ ▄███▓ ██▓  ██████   ██████ ▓█████ ▓█████▄               ");
            println!("▓██▒▀█▀ ██▒▓██▒▒██    ▒ ▒██    ▒ ▓█   ▀ ▒██▀ ██▌              ");
            println!("▓██    ▓██░▒██▒░ ▓██▄   ░ ▓██▄   ▒███   ░██   █▌              ");
            println!("▒██    ▒██ ░██░  ▒   ██▒  ▒   ██▒▒▓█  ▄ ░▓█▄   ▌              ");
            println!("▒██▒   ░██▒░██░▒██████▒▒▒██████▒▒░▒████▒░▒████▓  ██▓  ██▓  ██▓");
            println!("░ ▒░   ░  ░░▓  ▒ ▒▓▒ ▒ ░▒ ▒▓▒ ▒ ░░░ ▒░ ░ ▒▒▓  ▒  ▒▓▒  ▒▓▒  ▒▓▒");
            println!("░  ░      ░ ▒ ░░ ░▒  ░ ░░ ░▒  ░ ░ ░ ░  ░ ░ ▒  ▒  ░▒   ░▒   ░▒ ");
            println!("░      ░    ▒ ░░  ░  ░  ░  ░  ░     ░    ░ ░  ░  ░    ░    ░  ");
            println!("       ░    ░        ░        ░     ░  ░   ░      ░    ░    ░ ");
            println!("                                         ░        ░    ░    ░ ");
        }

        // Program pauses for a second.
        thread::sleep(Duration::from_secs(1));

        // Increments the rounds.
        rounds += 1;

        // Makes the turn the opposite of what it is currently.
        player_turn = !player_turn;
    }

    // If the player has more than 0 lives (is alive).
    if player > 0 {
        return true;
    }

    // Player lost.
    false
}

/// This function clears the teriminal by filling it with enough white spaces to reset the terminal.
fn clear() {
    // White space spam.
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    // Push the white space out to the terminal.
    stdout().flush().expect("SysErr: Could Not Flush [STDOUT]");
}