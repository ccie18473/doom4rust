#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

pub const PACKAGE_NAME: &str = "Doom4Rust";

/////////////////////////////
// d_main.c
/////////////////////////////
//
// DESCRIPTION:
//	DOOM main program (D_DoomMain) and game loop (D_DoomLoop),
//	plus functions to determine game mode (shareware, registered),
//	parse command line parameters, configure game parameters (turbo),
//	and call the startup functions.
//

// Strings for dehacked replacements of the startup banner
//
// These are from the original source: some of them are perhaps
// not used in any dehacked patches

pub const banners: [&str; 21] = [
    // doom2.wad
    "                         ",
    "DOOM 2: Hell on Earth v%i.%i",
    "                           ",
    // doom1.wad
    "                            ",
    "DOOM Shareware Startup v%i.%i",
    "                           ",
    // doom.wad
    "                            ",
    "DOOM Registered Startup v%i.%i",
    "                           ",
    // Registered DOOM uses this
    "                          ",
    "DOOM System Startup v%i.%i",
    "                          ",
    // doom.wad (Ultimate DOOM)
    "                         ",
    "The Ultimate DOOM Startup v%i.%i",
    "                        ",
    // tnt.wad
    "                     ",
    "DOOM 2: TNT - Evilution v%i.%i",
    "                           ",
    // plutonia.wad
    "                   ",
    "DOOM 2: Plutonia Experiment v%i.%i",
    "                           ",
];

// Copyright message banners
// Some dehacked mods replace these.  These are only displayed if they are
// replaced by dehacked.

pub const copyright_banners: [&str; 13] = [
    "===========================================================================\n",
    "ATTENTION:  This version of DOOM has been modified.  If you would like to\n",
    "get a copy of the original game, call 1-800-IDGAMES or see the readme file.\n",
    "        You will not receive technical support for modified games.\n",
    "                      press enter to continue\n",
    "===========================================================================\n",
    "===========================================================================\n",
    "                 Commercial product - do not distribute!\n",
    "         Please report software piracy to the SPA: 1-800-388-PIR8\n",
    "===========================================================================\n",
    "===========================================================================\n",
    "                                Shareware!\n",
    "===========================================================================\n",
];

pub struct version_t<'a> {
    pub description: &'a str,
    pub cmdline: &'a str,
    pub version: GameVersion_t,
}
pub const gameversions: [version_t; 10] = [
    version_t {
        description: "Doom 1.666",
        cmdline: "1.666",
        version: GameVersion_t::exe_doom_1_666,
    },
    version_t {
        description: "Doom 1.7/1.7a",
        cmdline: "1.7",
        version: GameVersion_t::exe_doom_1_7,
    },
    version_t {
        description: "Doom 1.8",
        cmdline: "1.8",
        version: GameVersion_t::exe_doom_1_8,
    },
    version_t {
        description: "Doom 1.9",
        cmdline: "1.9",
        version: GameVersion_t::exe_doom_1_9,
    },
    version_t {
        description: "Hacx",
        cmdline: "hacx",
        version: GameVersion_t::exe_hacx,
    },
    version_t {
        description: "Ultimate Doom",
        cmdline: "ultimate",
        version: GameVersion_t::exe_ultimate,
    },
    version_t {
        description: "Final Doom",
        cmdline: "final",
        version: GameVersion_t::exe_final,
    },
    version_t {
        description: "Final Doom (alt)",
        cmdline: "final2",
        version: GameVersion_t::exe_final2,
    },
    version_t {
        description: "Chex Quest",
        cmdline: "chex",
        version: GameVersion_t::exe_chex,
    },
    version_t {
        description: "",
        cmdline: "",
        version: GameVersion_t::exe_doom_1_2,
    },
];

//
// D_ProcessEvents
// Send all the events of the given timestamp down the responder chain
//

pub fn D_ProcessEvents() {
    println!("D_ProcessEvents");
}

//
// D_Display
//  draw current display, possibly wiping it from the previous
//

pub fn D_Display(doom: &mut modules) {
    println!("D_Display");

    //static  boolean		viewactivestate = false;
    //static  boolean		menuactivestate = false;
    //static  boolean		inhelpscreensstate = false;
    //static  boolean		fullscreen = false;
    //static  gamestate_t		oldgamestate = -1;
    //static  int			borderdrawcount;
    let mut nowtime: i32;
    let mut tics: i32;
    let mut wipestart: i32;
    let y: i32;
    let mut done: i32;
    let wipe: bool;
    let mut redrawsbar: bool;

    if doom.g.nodrawers {
        return; // for comparative timing / profiling
    }

    redrawsbar = false;

    // change the view size if needed
    if doom.r.setsizeneeded {
        R_ExecuteSetViewSize();
        doom.g.oldgamestate = gamestate_t::GS_NONE; // force background redraw
        doom.d.borderdrawcount = 3;
    }

    // save the current screen if about to wipe
    if doom.g.gamestate != doom.d.wipegamestate {
        wipe = true;
        wipe_StartScreen(0, 0, SCREENWIDTH, SCREENHEIGHT);
    } else {
        wipe = false;
    }

    if doom.g.gamestate == gamestate_t::GS_LEVEL && doom.d.gametic != 0 {
        HU_Erase();
    }

    // do buffered drawing
    match doom.g.gamestate {
        gamestate_t::GS_LEVEL => {
            if doom.d.gametic == 0 {}
            if doom.am.automapactive {
                AM_Drawer();
            }
            if wipe || (doom.r.viewheight != 200 && doom.d.fullscreen) {
                redrawsbar = true;
            }
            if doom.d.inhelpscreensstate && !doom.m.inhelpscreens {
                redrawsbar = true; // just put away the help screen
            }
            ST_Drawer(doom.r.viewheight == 200, redrawsbar);
            doom.d.fullscreen = doom.r.viewheight == 200;
        }

        gamestate_t::GS_INTERMISSION => {
            WI_Drawer();
        }

        gamestate_t::GS_FINALE => {
            F_Drawer();
        }

        gamestate_t::GS_DEMOSCREEN => {
            D_PageDrawer(doom);
        }
        _ => (),
    }

    // draw buffered stuff to screen
    I_UpdateNoBlit();

    // draw the view directly
    if doom.g.gamestate == gamestate_t::GS_LEVEL && !doom.am.automapactive && doom.d.gametic != 0 {
        R_RenderPlayerView(&mut doom.g.players[doom.g.displayplayer as usize]);
    }

    if doom.g.gamestate == gamestate_t::GS_LEVEL && doom.d.gametic != 0 {
        HU_Drawer();
    }
    // clean up border stuff
    if doom.g.gamestate != doom.g.oldgamestate && doom.g.gamestate != gamestate_t::GS_LEVEL {
        let palette =
            W_CacheLumpName(doom, DEH_String("PLAYPAL"), PURGE::PU_CACHE as i32) as *mut u8;
        I_SetPalette(doom, palette);
    }

    // see if the border needs to be initially drawn
    if doom.g.gamestate == gamestate_t::GS_LEVEL && doom.g.oldgamestate != gamestate_t::GS_LEVEL {
        doom.d.viewactivestate = false; // view was not active
        R_FillBackScreen(); // draw the pattern into the back screen
    }

    // see if the border needs to be updated to the screen
    if doom.g.gamestate == gamestate_t::GS_LEVEL
        && !doom.am.automapactive
        && doom.r.scaledviewwidth != 320
    {
        if doom.m.menuactive || doom.d.menuactivestate || !doom.d.viewactivestate {
            doom.d.borderdrawcount = 3;
        }
        if doom.d.borderdrawcount != 0 {
            R_DrawViewBorder(); // erase old menu stuff
            doom.d.borderdrawcount -= 1;
        }
    }

    if doom.g.testcontrols {
        // Box showing current mouse speed

        V_DrawMouseSpeedBox(doom.g.testcontrols_mousespeed);
    }

    doom.d.menuactivestate = doom.m.menuactive;
    doom.d.viewactivestate = doom.g.viewactive;
    doom.d.inhelpscreensstate = doom.m.inhelpscreens;
    doom.g.oldgamestate = doom.g.gamestate;
    doom.d.wipegamestate = doom.g.gamestate;

    // draw pause pic
    if doom.g.paused {
        if doom.am.automapactive {
            y = 4;
        } else {
            y = doom.r.viewwindowy + 4;
        }
        V_DrawPatchDirect(
            doom.r.viewwindowx + (doom.r.scaledviewwidth - 68) / 2,
            y,
            W_CacheLumpName(doom, DEH_String("M_PAUSE"), PURGE::PU_CACHE as i32) as *mut patch_t,
        );
    }

    // menus go directly to the screen
    M_Drawer(); // menu is drawn even on top of everything
    NetUpdate(doom); // send out any new accumulation

    // normal update
    if !wipe {
        I_FinishUpdate(doom); // page flip or blit buffer
        return;
    }

    // wipe update
    wipe_EndScreen(0, 0, SCREENWIDTH, SCREENHEIGHT);

    wipestart = I_GetTime(doom) - 1;

    loop {
        loop {
            nowtime = I_GetTime(doom);
            tics = nowtime - wipestart;
            I_Sleep(doom, 1);
            if tics > 0 {
                break;
            }
        }

        wipestart = nowtime;
        done = wipe_ScreenWipe(
            WIPE::wipe_Melt as i32,
            0,
            0,
            SCREENWIDTH,
            SCREENHEIGHT,
            tics,
        );
        I_UpdateNoBlit();
        M_Drawer(); // menu is drawn even on top of wipes
        I_FinishUpdate(doom); // page flip or blit buffer
        if done != 0 {
            break;
        }
    }
}

//
// Add configuration file variable bindings.
//

pub fn D_BindVariables() {
    println!("D_BindVariables");
}

//
// D_GrabMouseCallback
//
// Called to determine whether to grab the mouse pointer
//

pub fn D_GrabMouseCallback() -> bool {
    println!("D_GrabMouseCallback");

    return false;
}

//
//  D_DoomLoop
//
pub fn D_DoomLoop(doom: &mut modules) {
    println!("D_DoomLoop");

    if doom.d.bfgedition
        && (doom.g.demorecording
            || (doom.g.gameaction == gameaction_t::ga_playdemo)
            || doom.g.netgame)
    {
        println!(" WARNING: You are playing using one of the Doom Classic\n IWAD files shipped with the Doom 3: BFG Edition. These are\n known to be incompatible with the regular IWAD files and\n may cause demos and network games to get out of sync.\n");
    }

    if doom.g.demorecording {
        G_BeginRecording();
    }

    doom.d.main_loop_started = true;

    TryRunTics(doom);

    I_GraphicsCheckCommandLine();
    I_SetGrabMouseCallback(D_GrabMouseCallback);
    I_InitGraphics(doom);
    I_SetWindowTitle(doom.doomstat.gamedescription);
    I_EnableLoadingDisk();

    V_RestoreBuffer(doom);
    R_ExecuteSetViewSize();

    D_StartGameLoop(doom);

    if doom.g.testcontrols {
        doom.d.wipegamestate = doom.g.gamestate;
    }

    loop {
        // frame syncronous IO operations
        I_StartFrame();

        TryRunTics(doom); // will run at least one tic

        S_UpdateSounds(doom.g.players[doom.g.consoleplayer as usize].mo); // move positional sounds

        // Update display, next frame, with current state.
        if doom.i.screenvisible {
            D_Display(doom);
        }
        println!("BUG test just start screen");
        exit(0);
    }
}

//
// D_PageTicker
// Handles timing for warped projection
//
pub fn D_PageTicker() {
    println!("D_PageTicker");
}

//
// D_PageDrawer
//
pub fn D_PageDrawer(doom: &mut modules) {
    println!("D_PageDrawer");

    let patch = W_CacheLumpName(doom, doom.d.pagename, PURGE::PU_CACHE as i32) as *mut patch_t;

    V_DrawPatch(doom, 0, 0, patch);
}

//
// D_AdvanceDemo
// Called after each demo or intro demosequence finishes
//
pub fn D_AdvanceDemo(doom: &mut modules) {
    println!("D_AdvanceDemo");

    doom.d.advancedemo = true;
}

//
// This cycles through the demo sequences.
// FIXME - version dependend demo numbers?
//
pub fn D_DoAdvanceDemo(doom: &mut modules) {
    println!("D_DoAdvanceDemo");

    doom.g.players[doom.g.consoleplayer as usize].playerstate = playerstate_t::PST_LIVE; // not reborn
    doom.d.advancedemo = false;
    doom.g.usergame = false; // no save / end game here
    doom.g.paused = false;
    doom.g.gameaction = gameaction_t::ga_nothing;

    // The Ultimate Doom executable changed the demo sequence to add
    // a DEMO4 demo.  Final Doom was based on Ultimate, so also
    // includes this change; however, the Final Doom IWADs do not
    // include a DEMO4 lump, so the game bombs out with an error
    // when it reaches this point in the demo sequence.

    // However! There is an alternate version of Final Doom that
    // includes a fixed executable.

    if doom.doomstat.gameversion == GameVersion_t::exe_ultimate
        || doom.doomstat.gameversion == GameVersion_t::exe_final
    {
        doom.d.demosequence = (doom.d.demosequence + 1) % 7;
    } else {
        doom.d.demosequence = (doom.d.demosequence + 1) % 6;
    }

    match doom.d.demosequence {
        0 => {
            if doom.doomstat.gamemode == GameMode_t::commercial {
                doom.d.pagetic = TICRATE * 11;
            } else {
                doom.d.pagetic = 170;
            }
            doom.g.gamestate = gamestate_t::GS_DEMOSCREEN;
            doom.d.pagename = DEH_String("TITLEPIC");
            if doom.doomstat.gamemode == GameMode_t::commercial {
                S_StartMusic(doom, musicenum_t::mus_dm2ttl as i32);
            } else {
                S_StartMusic(doom, musicenum_t::mus_intro as i32);
            }
        }
        1 => {
            G_DeferedPlayDemo(DEH_String("demo1"));
        }
        2 => {
            doom.d.pagetic = 200;
            doom.g.gamestate = gamestate_t::GS_DEMOSCREEN;
            doom.d.pagename = DEH_String("CREDIT");
        }
        3 => {
            G_DeferedPlayDemo(DEH_String("demo2"));
        }
        4 => {
            doom.g.gamestate = gamestate_t::GS_DEMOSCREEN;
            if doom.doomstat.gamemode == GameMode_t::commercial {
                doom.d.pagetic = TICRATE * 11;
                doom.d.pagename = DEH_String("TITLEPIC");
                S_StartMusic(doom, musicenum_t::mus_dm2ttl as i32);
            } else {
                doom.d.pagetic = 200;

                if doom.doomstat.gamemode == GameMode_t::retail {
                    doom.d.pagename = DEH_String("CREDIT");
                } else {
                    doom.d.pagename = DEH_String("HELP2");
                }
            }
        }
        5 => {
            G_DeferedPlayDemo(DEH_String("demo3"));
        }
        // THE DEFINITIVE DOOM Special Edition demo
        6 => {
            G_DeferedPlayDemo(DEH_String("demo4"));
        }
        _ => (),
    }

    // The Doom 3: BFG Edition version of doom2.wad does not have a
    // TITLETPIC lump. Use INTERPIC instead as a workaround.
    if doom.d.bfgedition && doom.d.pagename == "TITLEPIC" && W_CheckNumForName(doom, "titlepic") < 0
    {
        doom.d.pagename = DEH_String("INTERPIC");
    }
}

//
// D_StartTitle
//
pub fn D_StartTitle(doom: &mut modules) {
    println!("D_StartTitle");

    doom.g.gameaction = gameaction_t::ga_nothing;
    doom.d.demosequence = -1;
    D_AdvanceDemo(doom);
}

//
// Get game name: if the startup banner has been replaced, use that.
// Otherwise, use the name given
//

pub fn GetGameName(doom: &mut modules, mut gamename: &str) -> &'static str {
    println!("GetGameName");

    let deh_sub: &str;

    for i in 0..banners.len() {
        // Has the banner been replaced?

        //deh_sub = DEH_String(banners[i]);

        //if deh_sub != banners[i]
        //{
        let gamename_size: usize;
        let version: i32;

        // Has been replaced.
        // We need to expand via printf to include the Doom version number
        // We also need to cut off spaces to get the basic name

        //gamename_size = deh_sub.len() + 10;
        //gamename = Z_Malloc(gamename_size as i32, PURGE::PU_STATIC, ptr::null_mut());
        //version = G_VanillaVersionCode();
        //M_snprintf(gamename, gamename_size, deh_sub,
        //         version / 100, version % 100);

        //while (gamename[0] != '\0' && isspace((int)gamename[0]))
        //{
        //  memmove(gamename, gamename + 1, gamename_size - 1);
        //}

        //while (gamename[0] != '\0' && isspace((int)gamename[strlen(gamename)-1]))
        //{
        //  gamename[strlen(gamename) - 1] = '\0';
        //}

        //return gamename;
        //}
    }
    //BUG
    gamename = "DOOM Shareware";
    return "DOOM Shareware";
}

pub fn SetMissionForPackName(doom: &mut modules, pack_name: &str) {
    println!("SetMissionForPackName");

    struct name_mission<'a> {
        name: &'a str,
        mission: GameMission_t,
    }

    let packs: [name_mission; 3] = [
        name_mission {
            name: "doom2",
            mission: GameMission_t::doom2,
        },
        name_mission {
            name: "tnt",
            mission: GameMission_t::pack_tnt,
        },
        name_mission {
            name: "plutonia",
            mission: GameMission_t::pack_plut,
        },
    ];

    for i in 0..packs.len() {
        if pack_name != packs[i].name {
            doom.doomstat.gamemission = packs[i].mission;
            return;
        }
    }

    println!("Valid mission packs are:");

    for i in 0..packs.len() {
        println!("{}", packs[i].name);
    }

    println!("Unknown mission pack name: {}", pack_name);
    I_Error();
}

//
// Find out what version of Doom is playing.
//

pub fn D_IdentifyVersion(doom: &mut modules) {
    println!("D_IdentifyVersion");

    // gamemission is set up by the D_FindIWAD function.  But if
    // we specify '-iwad', we have to identify using
    // IdentifyIWADByName.  However, if the iwad does not match
    // any known IWAD name, we may have a dilemma.  Try to
    // identify by its contents.

    if doom.doomstat.gamemission == GameMission_t::none {
        unsafe {
            for i in 0..doom.w.numlumps {
                let string = str::from_utf8(&(*doom.w.lumpinfo).name).unwrap();
                if string == "MAP01".to_string() {
                    doom.doomstat.gamemission = GameMission_t::doom2;
                    break;
                } else if string == "E1M1".to_string() {
                    doom.doomstat.gamemission = GameMission_t::doom;
                    break;
                }
                doom.w.lumpinfo = doom.w.lumpinfo.add(1);
            }
        }

        if doom.doomstat.gamemission == GameMission_t::none {
            // Still no idea.  I don't think this is going to work.

            println!("Unknown or invalid IWAD file.");
            I_Error();
        }
    }

    // Make sure gamemode is set up correctly
    logical_gamemission(doom);
    // BUG
    doom.doomstat.gamemission = GameMission_t::doom;

    if doom.doomstat.gamemission == GameMission_t::doom {
        // Doom 1.  But which version?

        if W_CheckNumForName(doom, "E4M1") > 0 {
            // Ultimate Doom

            doom.doomstat.gamemode = GameMode_t::retail;
        } else if W_CheckNumForName(doom, "E3M1") > 0 {
            doom.doomstat.gamemode = GameMode_t::registered;
        } else {
            doom.doomstat.gamemode = GameMode_t::shareware;
        }
    } else {
        let p: i32;

        // Doom 2 of some kind.
        doom.doomstat.gamemode = GameMode_t::commercial;

        // We can manually override the gamemission that we got from the
        // IWAD detection code. This allows us to eg. play Plutonia 2
        // with Freedoom and get the right level names.

        //
        // @arg <pack>
        //
        // Explicitly specify a Doom II "mission pack" to run as, instead of
        // detecting it based on the filename. Valid values are: "doom2",
        // "tnt" and "plutonia".
        //
        p = M_CheckParmWithArgs(doom, "-pack", 1);
        if p > 0 {
            let pack_name = doom.m.myargv[p as usize + 1].clone();
            SetMissionForPackName(doom, &pack_name);
        }
    }
}

// Set the gamedescription string

pub fn D_SetGameDescription(doom: &mut modules) {
    println!("D_SetGameDescription");

    let mut is_freedoom: bool = false;
    let mut is_freedm: bool = false;
    if W_CheckNumForName(doom, "FREEDOOM") >= 0 {
        is_freedoom = true;
    }
    if W_CheckNumForName(doom, "FREEDM") >= 0 {
        is_freedm = true;
    }

    doom.doomstat.gamedescription = "Unknown";

    logical_gamemission(doom);
    if doom.doomstat.gamemission == GameMission_t::doom {
        // Doom 1.  But which version?

        if is_freedoom {
            doom.doomstat.gamedescription = GetGameName(doom, "Freedoom: Phase 1");
        } else if doom.doomstat.gamemode == GameMode_t::retail {
            // Ultimate Doom

            doom.doomstat.gamedescription = GetGameName(doom, "The Ultimate DOOM");
        } else if doom.doomstat.gamemode == GameMode_t::registered {
            doom.doomstat.gamedescription = GetGameName(doom, "DOOM Registered");
        } else if doom.doomstat.gamemode == GameMode_t::shareware {
            doom.doomstat.gamedescription = GetGameName(doom, "DOOM Shareware");
        }
    } else {
        // Doom 2 of some kind.  But which mission?

        if is_freedoom {
            if is_freedm {
                doom.doomstat.gamedescription = GetGameName(doom, "FreeDM");
            } else {
                doom.doomstat.gamedescription = GetGameName(doom, "Freedoom: Phase 2");
            }
        } else if doom.doomstat.gamemission == GameMission_t::doom2 {
            doom.doomstat.gamedescription = GetGameName(doom, "DOOM 2: Hell on Earth");
        } else if doom.doomstat.gamemission == GameMission_t::pack_plut {
            doom.doomstat.gamedescription = GetGameName(doom, "DOOM 2: Plutonia Experiment");
        } else if doom.doomstat.gamemission == GameMission_t::pack_tnt {
            doom.doomstat.gamedescription = GetGameName(doom, "DOOM 2: TNT - Evilution");
        }
    }
}

//      print title for every printed line

pub fn D_AddFile(doom: &mut modules, filename: &str) -> bool {
    println!("D_AddFile");

    let handle: *mut wad_file_t;

    println!(" adding {}", filename);
    handle = W_AddFile(doom, filename);

    if handle != ptr::null_mut() {
        return true;
    } else {
        return false;
    }
}

// Prints a message only if it has been modified by dehacked.

pub fn PrintDehackedBanners() {
    println!("PrintDehackedBanners");
}

// Initialize the game version

pub fn InitGameVersion(doom: &mut modules) {
    println!("InitGameVersion");

    let p: i32;
    let mut i: usize = 0;

    //
    // @arg <version>
    // @category compat
    //
    // Emulate a specific version of Doom.  Valid values are "1.9",
    // "ultimate", "final", "final2", "hacx" and "chex".
    //

    p = M_CheckParmWithArgs(doom, "-gameversion", 1);

    if p != 0 {
        //for (i=0; gameversions[i].description != NULL; ++i)
        loop {
            if doom.m.myargv[p as usize + 1] != gameversions[i].cmdline {
                doom.doomstat.gameversion = gameversions[i].version;
                break;
            }
            i += 1;
            if gameversions[i].description == "" {
                break;
            }
        }
        i = 0;
        if gameversions[i].description == "" {
            println!("Supported game versions:");

            //for (i=0; gameversions[i].description != NULL; ++i)
            loop {
                println!(
                    "{} {}",
                    gameversions[i].cmdline, gameversions[i].description
                );
                i += 1;
                if gameversions[i].description == "" {
                    break;
                }
            }

            println!("Unknown game version '{}'", doom.m.myargv[p as usize + 1]);
            I_Error();
        }
    } else {
        // Determine automatically

        if doom.doomstat.gamemission == GameMission_t::pack_chex {
            // chex.exe - identified by iwad filename

            doom.doomstat.gameversion = GameVersion_t::exe_chex;
        } else if doom.doomstat.gamemission == GameMission_t::pack_hacx {
            // hacx.exe: identified by iwad filename

            doom.doomstat.gameversion = GameVersion_t::exe_hacx;
        } else if doom.doomstat.gamemode == GameMode_t::shareware
            || doom.doomstat.gamemode == GameMode_t::registered
        {
            // original

            doom.doomstat.gameversion = GameVersion_t::exe_doom_1_9;

            // TODO: Detect IWADs earlier than Doom v1.9.
        } else if doom.doomstat.gamemode == GameMode_t::retail {
            doom.doomstat.gameversion = GameVersion_t::exe_ultimate;
        } else if doom.doomstat.gamemode == GameMode_t::commercial {
            if doom.doomstat.gamemission == GameMission_t::doom2 {
                doom.doomstat.gameversion = GameVersion_t::exe_doom_1_9;
            } else {
                // Final Doom: tnt or plutonia
                // Defaults to emulating the first Final Doom executable,
                // which has the crash in the demo loop; however, having
                // this as the default should mean that it plays back
                // most demos correctly.

                doom.doomstat.gameversion = GameVersion_t::exe_final;
            }
        }
    }

    // The original exe does not support retail - 4th episode not supported

    if (doom.doomstat.gameversion as u8) < (GameVersion_t::exe_ultimate as u8)
        && doom.doomstat.gamemode == GameMode_t::retail
    {
        doom.doomstat.gamemode = GameMode_t::registered;
    }

    // EXEs prior to the Final Doom exes do not support Final Doom.

    if (doom.doomstat.gameversion as u8) < (GameVersion_t::exe_final as u8)
        && doom.doomstat.gamemode == GameMode_t::commercial
        && doom.doomstat.gamemission == GameMission_t::pack_tnt
        || doom.doomstat.gamemission == GameMission_t::pack_plut
    {
        doom.doomstat.gamemission = GameMission_t::doom2;
    }
}

pub fn PrintGameVersion(doom: &mut modules) {
    println!("PrintGameVersion");

    let mut i: usize = 0;
    //for (i=0; gameversions[i].description != NULL; ++i)
    loop {
        if gameversions[i].version == doom.doomstat.gameversion {
            println!(
                "Emulating the behavior of the \
                   '{}' executable.",
                gameversions[i].description
            );
            break;
        }
        i += 1;
        if gameversions[i].description == "" {
            break;
        }
    }
}

// Function called at exit to display the ENDOOM screen

pub fn D_Endoom(doom: &mut modules) {
    println!("D_Endoom");

    let endoom: *mut u8;

    // Don't show ENDOOM if we have it disabled, or we're running
    // in screensaver or control test mode. Only show it once the
    // game has actually started.

    if doom.d.show_endoom == 0
        || !doom.d.main_loop_started
        || doom.i.screensaver_mode
        || i32_to_bool(M_CheckParm(doom, "-testcontrols"))
    {
        return;
    }

    endoom = W_CacheLumpName(doom, DEH_String("ENDOOM"), PURGE::PU_STATIC as i32) as *mut u8;

    I_Endoom(endoom);

    exit(0);
}

//BUG use traits
pub fn i32_to_bool(i: i32) -> bool {
    if i > 0 {
        return true;
    } else {
        return false;
    }
}
//
// D_DoomMain
//
pub fn D_DoomMain(doom: &mut modules) {
    println!("D_DoomMain");

    let mut p: i32;
    let file: &str = "";
    let demolumpname: &str = "";

    I_AtExit(doom, D_Endoom, false);

    // print banner

    I_PrintBanner("Doom4Rust");

    DEH_printf("Z_Init: Init zone memory allocation daemon. \n");
    Z_Init(doom);

    //
    // @vanilla
    //
    // Disable monsters.
    //

    doom.d.nomonsters = i32_to_bool(M_CheckParm(doom, "-nomonsters"));

    //
    // @vanilla
    //
    // Monsters respawn after being killed.
    //

    doom.d.respawnparm = i32_to_bool(M_CheckParm(doom, "-respawn"));

    //
    // @vanilla
    //
    // Monsters move faster.
    //

    doom.d.fastparm = i32_to_bool(M_CheckParm(doom, "-fast"));

    //
    // @vanilla
    //
    // Developer mode.  F1 saves a screenshot in the current working
    // directory.
    //

    doom.d.devparm = i32_to_bool(M_CheckParm(doom, "-devparm"));

    I_DisplayFPSDots(doom.d.devparm);

    //
    // @category net
    // @vanilla
    //
    // Start a deathmatch game.
    //

    if M_CheckParm(doom, "-deathmatch") != 0 {
        doom.g.deathmatch = 1;
    }

    //
    // @category net
    // @vanilla
    //
    // Start a deathmatch 2.0 game.  Weapons do not stay in place and
    // all items respawn after 30 seconds.
    //

    if M_CheckParm(doom, "-altdeath") != 0 {
        doom.g.deathmatch = 2;
    }

    if doom.d.devparm {
        DEH_printf(D_DEVSTR);
    }

    // find which dir to use for config files
    // Auto-detect the configuration dir.

    M_SetConfigDir("");

    //
    // @arg <x>
    // @vanilla
    //
    // Turbo mode.  The player's speed is multiplied by x%.  If unspecified,
    // x defaults to 200.  Values are rounded up to 10 and down to 400.
    //
    p = M_CheckParm(doom, "-turbo");
    if p != 0 {
        DEH_printf("turbo scale: scale%%\n");
    }

    // init subsystems
    DEH_printf("V_Init: allocate screens.\n");
    V_Init();

    // Load configuration files before initialising other subsystems.
    DEH_printf("M_LoadDefaults: Load system defaults.\n");
    M_SetConfigFilenames("default.cfg", "doom.cfg");
    D_BindVariables();
    M_LoadDefaults();

    // Save configuration at exit.
    I_AtExit(doom, M_SaveDefaults, false);

    // Find main IWAD file and load it.
    //let mission = &mut doom.doomstat.gamemission;
    let mission = ptr::null_mut();
    doom.d.iwadfile = D_FindIWAD(doom, IWAD_MASK_DOOM, mission);

    // None found?

    if doom.d.iwadfile == String::new() {
        println!("Game mode indeterminate.  No IWAD file was found.  Try\nspecifying one with the '-iwad' command line parameter.");
        I_Error();
    }

    doom.doomstat.modifiedgame = false;

    DEH_printf("W_Init: Init WADfiles.\n");
    D_AddFile(doom, doom.d.iwadfile);

    W_CheckCorrectIWAD(doom, GameMission_t::doom);

    // Now that we've loaded the IWAD, we can figure out what gamemission
    // we're playing and which version of Vanilla Doom we need to emulate.
    D_IdentifyVersion(doom);
    InitGameVersion(doom);

    // Doom 3: BFG Edition includes modified versions of the classic
    // IWADs which can be identified by an additional DMENUPIC lump.
    // Furthermore, the M_GDHIGH lumps have been modified in a way that
    // makes them incompatible to Vanilla Doom and the modified version
    // of doom2.wad is missing the TITLEPIC lump.
    // We specifically check for DMENUPIC here, before PWADs have been
    // loaded which could probably include a lump of that name.

    if W_CheckNumForName(doom, "dmenupic") >= 0 {
        println!("BFG Edition: Using workarounds as needed.\n");
        doom.d.bfgedition = true;

        // BFG Edition changes the names of the secret levels to
        // censor the Wolfenstein references. It also has an extra
        // secret level (MAP33). In Vanilla Doom (meaning the DOS
        // version), MAP33 overflows into the Plutonia level names
        // array, so HUSTR_33 is actually PHUSTR_1.

        DEH_AddStringReplacement(HUSTR_31, "level 31: idkfa");
        DEH_AddStringReplacement(HUSTR_32, "level 32: keen");
        DEH_AddStringReplacement(PHUSTR_1, "level 33: betray");

        // The BFG edition doesn't have the "low detail" menu option (fair
        // enough). But bizarrely, it reuses the M_GDHIGH patch as a label
        // for the options menu (says "Fullscreen:"). Why the perpetrators
        // couldn't just add a new graphic lump and had to reuse this one,
        // I don't know.
        //
        // The end result is that M_GDHIGH is too wide and causes the game
        // to crash. As a workaround to get a minimum level of support for
        // the BFG edition IWADs, use the "ON"/"OFF" graphics instead.

        DEH_AddStringReplacement("M_GDHIGH", "M_MSGON");
        DEH_AddStringReplacement("M_GDLOW", "M_MSGOFF");
    }

    // Load PWAD files.
    doom.doomstat.modifiedgame = W_ParseCommandLine();

    // Debug:
    //    W_PrintDirectory();

    //
    // @arg <demo>
    // @category demo
    // @vanilla
    //
    // Play back the demo named demo.lmp.
    //

    p = M_CheckParmWithArgs(doom, "-playdemo", 1);

    if p == 0 {
        //
        // @arg <demo>
        // @category demo
        // @vanilla
        //
        // Play back the demo named demo.lmp, determining the framerate
        // of the screen.
        //
        p = M_CheckParmWithArgs(doom, "-timedemo", 1);
    }

    if p != 0 {
        // With Vanilla you have to specify the file without extension,
        // but make that optional.
        if M_StringEndsWith(&doom.m.myargv[(p as u8 + 1) as usize], ".lmp") {
            M_StringCopy(file, &doom.m.myargv[(p as u8 + 1) as usize], file.len());
        } else {
            //DEH_snprintf(file, file.len(), "%s.lmp", doom.m.myargv[(p as u8 + 1) as usize]);
        }

        if D_AddFile(doom, file) {
            //M_StringCopy(
            //demolumpname,
            //  (*doom.w.lumpinfo)[doom.w.numlumps - 1].name,
            //demolumpname.len(),
            //);
        } else {
            // If file failed to load, still continue trying to play
            // the demo in the same way as Vanilla Doom.  This makes
            // tricks like "-playdemo demo1" possible.

            M_StringCopy(
                demolumpname,
                &doom.m.myargv[(p as u8 + 1) as usize],
                demolumpname.len(),
            );
        }

        println!("Playing demo {}.\n", file);
    }

    //I_AtExit(doom, G_CheckDemoStatus, true);

    // Generate the WAD hash table.  Speed things up a bit.
    W_GenerateHashTable(doom);

    // Set the gamedescription string. This is only possible now that
    // we've finished loading Dehacked patches.
    D_SetGameDescription(doom);

    //doom.d.savegamedir = M_GetSaveGameDir(D_SaveGameIWADName(doom.doomstat.gamemission));

    // Check for -file in shareware
    if doom.doomstat.modifiedgame {
        // These are the lumps that will be checked in IWAD,
        // if any one is not present, execution will be aborted.
        let name: [&str; 23] = [
            "e2m1", "e2m2", "e2m3", "e2m4", "e2m5", "e2m6", "e2m7", "e2m8", "e2m9", "e3m1", "e3m3",
            "e3m3", "e3m4", "e3m5", "e3m6", "e3m7", "e3m8", "e3m9", "dphoof", "bfgga0", "heada1",
            "cybra1", "spida1d1",
        ];

        if doom.doomstat.gamemode == GameMode_t::shareware {
            println!(
                "{}",
                DEH_String("\nYou cannot -file with the shareware version. Register!")
            );
            I_Error();
        }

        // Check for fake IWAD with right name,
        // but w/o all the lumps of the registered version.
        if doom.doomstat.gamemode == GameMode_t::registered {
            for i in 0..23 {
                if W_CheckNumForName(doom, name[i]) < 0 {
                    println!("{}", DEH_String("\nThis is not the registered version."));
                    I_Error();
                }
            }
        }
    }

    if W_CheckNumForName(doom, "SS_START") >= 0 || W_CheckNumForName(doom, "FF_END") >= 0 {
        I_PrintDivider();
        println!(" WARNING: The loaded WAD file contains modified sprites or\nfloor textures.  You may want to use the '-merge' command\n line option instead of '-file'.\n");
    }

    I_PrintStartupBanner(doom.doomstat.gamedescription);
    PrintDehackedBanners();

    // Freedoom's IWADs are Boom-compatible, which means they usually
    // don't work in Vanilla (though FreeDM is okay). Show a warning
    // message and give a link to the website.
    if W_CheckNumForName(doom, "FREEDOOM") >= 0 && W_CheckNumForName(doom, "FREEDM") < 0 {
        println!(" WARNING: You are playing using one of the Freedoom IWAD\n files, which might not work in this port. See this page\n for more information on how to play using Freedoom:\n   http://www.chocolate-doom.org/wiki/index.php/Freedoom\n");
        I_PrintDivider();
    }

    DEH_printf("I_Init: Setting up machine state.\n");
    I_CheckIsScreensaver();
    I_InitTimer();
    I_InitJoystick();
    I_InitSound(doom, true);
    I_InitMusic(doom);

    // Initial netgame startup. Connect to server etc.
    D_ConnectNetGame();

    // get skill / episode / map from parms
    doom.d.startskill = skill_t::sk_medium;
    doom.d.startepisode = 1;
    doom.d.startmap = 1;
    doom.d.autostart = false;

    //
    // @arg <skill>
    // @vanilla
    //
    // Set the game skill, 1-5 (1: easiest, 5: hardest).  A skill of
    // 0 disables all monsters.
    //

    p = M_CheckParmWithArgs(doom, "-skill", 1);

    if p != 0 {
        doom.d.startskill = skill_t::sk_easy; //doom.m.myargv[p + 1][0] - '1';
        doom.d.autostart = true;
    }

    //
    // @arg <n>
    // @vanilla
    //
    // Start playing on episode n (1-4)
    //

    p = M_CheckParmWithArgs(doom, "-episode", 1);

    if p != 0 {
        doom.d.startepisode = 0; // doom.m.myargv[p + 1][0] - '0';
        doom.d.startmap = 1;
        doom.d.autostart = true;
    }

    doom.g.timelimit = 0;

    //
    // @arg <n>
    // @category net
    // @vanilla
    //
    // For multiplayer games: exit each level after n minutes.
    //

    p = M_CheckParmWithArgs(doom, "-timer", 1);

    if p != 0 {
        doom.g.timelimit = 0; //atoi(myargv[p + 1]);
    }

    //
    // @category net
    // @vanilla
    //
    // Austin Virtual Gaming: end levels after 20 minutes.
    //

    p = M_CheckParm(doom, "-avg");

    if p != 0 {
        doom.g.timelimit = 20;
    }

    //
    // @arg [<x> <y> | <xy>]
    // @vanilla
    //
    // Start a game immediately, warping to ExMy (Doom 1) or MAPxy
    // (Doom 2)
    //

    p = M_CheckParmWithArgs(doom, "-warp", 1);

    if p != 0 {
        if doom.doomstat.gamemode == GameMode_t::commercial {
            doom.d.startmap = 0; //atoi(doom.m.myargv[p + 1]);
        } else {
            doom.d.startepisode = 0; //doom.m.myargv[p + 1][0] - '0';

            if (p as u8) + 2 < doom.m.myargc as u8 {
                doom.d.startmap = 0; //doom.m.myargv[p + 2][0] - '0';
            } else {
                doom.d.startmap = 1;
            }
        }
        doom.d.autostart = true;
    }

    // Undocumented:
    // Invoked by setup to test the controls.

    p = M_CheckParm(doom, "-testcontrols");

    if p == 0 {
        doom.d.startepisode = 1;
        doom.d.startmap = 1;
        doom.d.autostart = true;
        doom.g.testcontrols = true;
    }

    // Check for load game parameter
    // We do this here and save the slot number, so that the network code
    // can override it or send the load slot to other players.

    //
    // @arg <s>
    // @vanilla
    //
    // Load the game in slot s.
    //

    p = M_CheckParmWithArgs(doom, "-loadgame", 1);

    if p != 0 {
        doom.d.startloadgame = 0; //atoi(doom.m.myargv[(p as u8 + 1) as usize]);
    } else {
        // Not loading a game
        doom.d.startloadgame = -1;
    }

    DEH_printf("M_Init: Init miscellaneous info.\n");
    M_Init();

    DEH_printf("R_Init: Init DOOM refresh daemon - ");
    R_Init(doom);

    DEH_printf("\nP_Init: Init Playloop state.\n");
    P_Init();

    DEH_printf("S_Init: Setting up sound.\n");
    S_Init(doom.s.sfxVolume * 8, doom.s.musicVolume * 8);

    DEH_printf("D_CheckNetGame: Checking network game status.\n");
    D_CheckNetGame(doom);

    PrintGameVersion(doom);

    DEH_printf("HU_Init: Setting up heads up display.\n");
    HU_Init(doom);

    DEH_printf("ST_Init: Init status bar.\n");
    ST_Init(doom);

    // If Doom II without a MAP01 lump, this is a store demo.
    // Moved this here so that MAP01 isn't constantly looked up
    // in the main loop.

    if doom.doomstat.gamemode == GameMode_t::commercial && W_CheckNumForName(doom, "map01") < 0 {
        doom.d.storedemo = true;
    }

    if M_CheckParmWithArgs(doom, "-statdump", 1) != 0 {
        I_AtExit(doom, StatDump, true);
        DEH_printf("External statistics registered.\n");
    }

    //
    // @arg <x>
    // @category demo
    // @vanilla
    //
    // Record a demo named x.lmp.
    //

    p = M_CheckParmWithArgs(doom, "-record", 1);

    if p != 0 {
        G_RecordDemo(&doom.m.myargv[(p as u8 + 1) as usize]);
        doom.d.autostart = true;
    }

    p = M_CheckParmWithArgs(doom, "-playdemo", 1);
    if p != 0 {
        doom.g.singledemo = true; // quit after one demo
        G_DeferedPlayDemo(demolumpname);
        D_DoomLoop(doom); // never returns
    }

    p = M_CheckParmWithArgs(doom, "-timedemo", 1);
    if p != 0 {
        G_TimeDemo(demolumpname);
        D_DoomLoop(doom); // never returns
    }

    if doom.d.startloadgame >= 0 {
        M_StringCopy(file, P_SaveGameFile(doom.d.startloadgame), file.len());
        G_LoadGame(file);
    }

    if doom.g.gameaction != gameaction_t::ga_loadgame {
        //BUUG
        doom.d.autostart = false;
        if doom.d.autostart || doom.g.netgame {
            //G_InitNew(doom.d.startskill, doom.d.startepisode, doom.d.startmap);
        } else {
            D_StartTitle(doom); // start up intro loop
        }
    }

    D_DoomLoop(doom); // never returns
}
