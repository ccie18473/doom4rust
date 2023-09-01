#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// d_net.c
/////////////////////////////
//
// DESCRIPTION:
//	DOOM Network game communication and protocol,
//	all OS independend parts.
//

// Called when a player leaves the game

pub fn PlayerQuitGame(player: *mut player_t) {
    println!("PlayerQuitGame");
}

pub fn RunTic(doom: &mut modules, cmds: *mut ticcmd_t, ingame: *mut bool) {
    println!("RunTic");

    // Check for player quits.

    for i in 0..MAXPLAYERS {
        //if !doom.g.demoplayback && playeringame[i] && !ingame[i])
        //{
        //    PlayerQuitGame(&players[i]);
        //}
    }

    doom.d.netcmds = cmds;

    // check that there are players in the game.  if not, we cannot
    // run a tic.

    if doom.d.advancedemo {
        D_DoAdvanceDemo(doom);
    }

    G_Ticker();
}

// Load game settings from the specified structure and
// set global variables.

pub fn LoadGameSettings(settings: *mut net_gamesettings_t) {
    println!("LoadGameSettings");
}

// Save the game settings from global variables to the specified
// game settings structure.

pub fn SaveGameSettings(settings: *mut net_gamesettings_t) {
    println!("SaveGameSettings");
}

pub fn InitConnectData(connect_data: *mut net_connect_data_t) {
    println!("InitConnectData");
}

pub fn D_ConnectNetGame() {
    println!("D_ConnectNetGame");
}

//
// D_CheckNetGame
// Works out player numbers among the net participants
//
pub fn D_CheckNetGame(doom: &mut modules) {
    println!("D_CheckNetGame");

    let mut settings: net_gamesettings_t = net_gamesettings_t::new();

    if doom.g.netgame {
        doom.d.autostart = true;
    }

    D_RegisterLoopCallbacks(doom);

    SaveGameSettings(&mut settings);
    //D_StartNetGame(&mut settings, NULL);
    LoadGameSettings(&mut settings);

    println!(
        "startskill {}  deathmatch: {}  startmap: {}  startepisode: {}",
        doom.d.startskill as i32, doom.g.deathmatch, doom.d.startmap, doom.d.startepisode
    );

    println!(
        "player {} of {} ({} nodes)",
        doom.g.consoleplayer + 1,
        settings.num_players,
        settings.num_players
    );

    // Show players here; the server might have specified a time limit

    if doom.g.timelimit > 0 && doom.g.deathmatch != 0 {
        // Gross hack to work like Vanilla:

        if doom.g.timelimit == 20 && M_CheckParm(doom, "-avg") != 0 {
            println!(
                "Austin Virtual Gaming: Levels will end \
                           after 20 minutes",
            );
        } else {
            println!("Levels will end after {} minute", doom.g.timelimit);
            if doom.g.timelimit > 1 {
                print!("s");
            }
            print!(".\n");
        }
    }
}
