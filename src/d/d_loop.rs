#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// d_loop.h
/////////////////////////////
//
// DESCRIPTION:
//	Main loop stuff.
//

// Callback function invoked while waiting for the netgame to start.
// The callback is invoked when new players are ready. The callback
// should return true, or return false to abort startup.

pub type netgame_startup_callback_t = fn(ready_players: i32, num_players: i32) -> bool;

pub struct loop_interface_t {
    // Read events from the event queue, and process them.
    pub ProcessEvents: fn(),
    // Given the current input state, fill in the fields of the specified
    // ticcmd_t structure with data for a new tic.
    pub BuildTiccmd: fn(cmd: *mut ticcmd_t, maketic: i32),
    // Advance the game forward one tic, using the specified player input.
    pub RunTic: fn(doom: &mut modules, cmds: *mut ticcmd_t, ingame: *mut bool),
    // Run the menu (runs independently of the game).
    pub RunMenu: fn(),
}

/////////////////////////////
// d_loop.c
/////////////////////////////
//
// DESCRIPTION:
//     Main loop code.
//

// The complete set of data for a particular tic.

#[derive(Copy, Clone)]
pub struct ticcmd_set_t {
    pub cmds: [ticcmd_t; NET_MAXPLAYERS],
    pub ingame: [bool; NET_MAXPLAYERS],
}

impl ticcmd_set_t {
    pub fn new() -> Self {
        Self {
            cmds: [ticcmd_t::new(); NET_MAXPLAYERS],
            ingame: [false; NET_MAXPLAYERS],
        }
    }
}

// 35 fps clock adjusted by offsetms milliseconds

pub fn GetAdjustedTime(doom: &mut modules) -> i32 {
    println!("GetAdjustedTime");

    let mut time_ms: i32;

    time_ms = I_GetTimeMS(doom);

    if doom.d.new_sync {
        // Use the adjustments from net_client.c only if we are
        // using the new sync mode.

        time_ms += doom.d.offsetms / FRACUNIT;
    }

    return (time_ms * TICRATE) / 1000;
}

pub fn BuildNewTic(doom: &mut modules) -> bool {
    println!("BuildNewTic");

    let gameticdiv: i32;
    let mut cmd: ticcmd_t;

    gameticdiv = doom.d.gametic / doom.d.ticdup;

    I_StartTic();
    unsafe { ((*doom.d.loop_interface).ProcessEvents)() };

    // Always run the menu

    unsafe { ((*doom.d.loop_interface).RunMenu)() };

    if drone {
        // In drone mode, do not generate any ticcmds.

        return false;
    }

    if doom.d.new_sync {
        // If playing single player, do not allow tics to buffer
        // up very far

        if !net_client_connected && doom.d.maketic - gameticdiv > 2 {
            return false;
        }

        // Never go more than ~200ms ahead

        if doom.d.maketic - gameticdiv > 8 {
            return false;
        }
    } else {
        if doom.d.maketic - gameticdiv >= 5 {
            return false;
        }
    }

    //printf ("mk:%i ",maketic);
    //memset(&cmd, 0, sizeof(ticcmd_t));
    cmd = ticcmd_t::new();
    unsafe { ((*doom.d.loop_interface).BuildTiccmd)(&mut cmd, doom.d.maketic) };

    doom.d.ticdata[(doom.d.maketic % BACKUPTICS) as usize].cmds[doom.d.localplayer as usize] = cmd;
    doom.d.ticdata[(doom.d.maketic % BACKUPTICS) as usize].ingame[doom.d.localplayer as usize] =
        true;

    doom.d.maketic += 1;

    return true;
}

//
// NetUpdate
// Builds ticcmds for console player,
// sends out a packet
//

pub fn NetUpdate(doom: &mut modules) {
    println!("NetUpdate");

    let nowtime: i32;
    let mut newtics: i32;

    // If we are running with singletics (timing a demo), this
    // is all done separately.

    if doom.d.singletics {
        return;
    }

    // check time
    nowtime = GetAdjustedTime(doom) / doom.d.ticdup;
    newtics = nowtime - doom.d.lasttime;

    doom.d.lasttime = nowtime;

    if doom.d.skiptics <= newtics {
        newtics -= doom.d.skiptics;
        doom.d.skiptics = 0;
    } else {
        doom.d.skiptics -= newtics;
        newtics = 0;
    }

    // build new ticcmds for console player

    for i in 0..newtics {
        if !BuildNewTic(doom) {
            break;
        }
    }
}

pub fn D_Disconnected() {
    println!("D_Disconnected");
}

//
// Invoked by the network engine when a complete set of ticcmds is
// available.
//

pub fn D_ReceiveTic(ticcmds: *mut ticcmd_t, players_mask: *mut bool) {
    println!("D_ReceiveTic");
}

//
// Start game loop
//
// Called after the screen is set but before the game starts running.
//

pub fn D_StartGameLoop(doom: &mut modules) {
    println!("D_StartGameLoop");

    doom.d.lasttime = GetAdjustedTime(doom) / doom.d.ticdup;
}

pub fn D_StartNetGame(settings: *mut net_gamesettings_t, callback: netgame_startup_callback_t) {
    println!("D_StartNetGame");
}

pub fn D_InitNetGame(connect_data: *mut net_connect_data_t) -> bool {
    println!("D_InitNetGame");

    return false;
}

//
// D_QuitNetGame
// Called before quitting to leave a net game
// without hanging the other players
//
pub fn D_QuitNetGame() {
    println!("D_QuitNetGame");
}

pub fn GetLowTic(doom: &mut modules) -> i32 {
    println!("GetLowTic");

    let lowtic: i32;

    lowtic = doom.d.maketic;

    return lowtic;
}

pub fn OldNetSync() {
    println!("OldNetSync");
}

// Returns true if there are players in the game:

pub fn PlayersInGame(doom: &mut modules) -> bool {
    println!("PlayersInGame");

    let mut result: bool = false;

    // If we are connected to a server, check if there are any players
    // in the game.

    if net_client_connected {
        for i in 0..NET_MAXPLAYERS {
            result = result || doom.d.local_playeringame[i];
        }
    }

    // Whether single or multi-player, unless we are running as a drone,
    // we are in the game.

    if !drone {
        result = true;
    }

    return result;
}

// When using ticdup, certain values must be cleared out when running
// the duplicate ticcmds.

pub fn TicdupSquash(set: *mut ticcmd_set_t) {
    println!("TicdupSquash");
}

// When running in single player mode, clear all the ingame[] array
// except the local player.

pub fn SinglePlayerClear(doom: &mut modules, set: *mut ticcmd_set_t) {
    println!("SinglePlayerClear");

    for i in 0..NET_MAXPLAYERS {
        if i != doom.d.localplayer as usize {
            unsafe { (*set).ingame[i] = false };
        }
    }
}

//
// TryRunTics
//

pub fn TryRunTics(doom: &mut modules) {
    println!("TryRunTics");

    let mut lowtic: i32;
    let entertic: i32;
    //static int oldentertics;
    let realtics: i32;
    let availabletics: i32;
    let mut counts: i32;

    // get real tics
    entertic = I_GetTime(doom) / doom.d.ticdup;
    realtics = entertic - doom.d.oldentertics;
    doom.d.oldentertics = entertic;

    // in singletics mode, run a single tic every time this function
    // is called.

    if doom.d.singletics {
        BuildNewTic(doom);
    } else {
        NetUpdate(doom);
    }

    lowtic = GetLowTic(doom);

    availabletics = lowtic - doom.d.gametic / doom.d.ticdup;

    // decide how many tics to run

    if doom.d.new_sync {
        counts = availabletics;
    } else {
        // decide how many tics to run
        if realtics < availabletics - 1 {
            counts = realtics + 1;
        } else if realtics < availabletics {
            counts = realtics;
        } else {
            counts = availabletics;
        }

        if counts < 1 {
            counts = 1;
        }

        if net_client_connected {
            OldNetSync();
        }
    }

    if counts < 1 {
        counts = 1;
    }

    // wait for new tics if needed

    while !PlayersInGame(doom) || lowtic < doom.d.gametic / doom.d.ticdup + counts {
        NetUpdate(doom);

        lowtic = GetLowTic(doom);

        if lowtic < doom.d.gametic / doom.d.ticdup {
            println!("TryRunTics: lowtic < gametic");
            I_Error();
        }

        // Don't stay in this loop forever.  The menu is still running,
        // so return to update the screen

        if I_GetTime(doom) / doom.d.ticdup - entertic > 0 {
            return;
        }

        I_Sleep(doom, 1);
    }

    // run the count * ticdup dics
    while counts > 0 {
        counts -= 1;
        let set: *mut ticcmd_set_t;

        if !PlayersInGame(doom) {
            return;
        }

        set = &mut doom.d.ticdata[((doom.d.gametic / doom.d.ticdup) % BACKUPTICS) as usize];

        if !net_client_connected {
            SinglePlayerClear(doom, set);
        }

        for i in 0..doom.d.ticdup {
            if doom.d.gametic / doom.d.ticdup > lowtic {
                println!("gametic>lowtic");
                I_Error();
            }

            //memcpy(local_playeringame, set->ingame, sizeof(local_playeringame));
            unsafe { doom.d.local_playeringame = (*set).ingame };

            unsafe {
                ((*doom.d.loop_interface).RunTic)(
                    doom,
                    (*set).cmds.as_mut_ptr(),
                    (*set).ingame.as_mut_ptr(),
                )
            };

            doom.d.gametic += 1;

            // modify command for duplicated tics

            TicdupSquash(set);
        }

        NetUpdate(doom); // check for new console commands
    }
}

pub fn D_RegisterLoopCallbacks(doom: &mut modules) {
    println!("D_RegisterLoopCallbacks");

    doom.d.loop_interface = &mut doom.d.doom_loop_interface;
}
