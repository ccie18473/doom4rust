#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// net_defs.h
/////////////////////////////

// Absolute maximum number of "nodes" in the game.  This is different to
// NET_MAXPLAYERS, as there may be observers that are not participating
// (eg. left/right monitors)

pub const MAXNETNODES: u8 = 16;

// The maximum number of players, multiplayer/networking.
// This is the maximum supported by the networking code; individual games
// have their own values for MAXPLAYERS that can be smaller.

pub const NET_MAXPLAYERS: usize = 8;

// Maximum length of a player's name.

pub const MAXPLAYERNAME: u8 = 30;

// Networking and tick handling related.

pub const BACKUPTICS: i32 = 128;

pub type net_module_t = _net_module_s;
pub type net_packet_t = _net_packet_s;
pub type net_addr_t = _net_addr_s;
//pub type net_context_t = _net_context_s;

pub struct _net_packet_s {
    pub data: *mut u8,
    pub len: i32,
    pub alloced: usize,
    pub pos: u32,
}

pub struct _net_module_s {
    // Initialize this module for use as a client

    //boolean (*InitClient)(void);

    // Initialize this module for use as a server

    //boolean (*InitServer)(void);

    // Send a packet

    //void (*SendPacket)(net_addr_t *addr, net_packet_t *packet);

    // Check for new packets to receive
    //
    // Returns true if packet received

    //boolean (*RecvPacket)(net_addr_t **addr, net_packet_t **packet);

    // Converts an address to a string

    //void (*AddrToString)(net_addr_t *addr, char *buffer, int buffer_len);

    // Free back an address when no longer in use

    //void (*FreeAddress)(net_addr_t *addr);

    // Try to resolve a name to an address

    //net_addr_t *(*ResolveAddress)(char *addr);
}

// net_addr_t

pub struct _net_addr_s {
    pub module: *mut net_module_t,
    pub handle: *mut libc::c_void,
}

// magic number sent when connecting to check this is a valid client

pub const NET_MAGIC_NUMBER: u32 = 3436803284;

// header field value indicating that the packet is a reliable packet

pub const NET_RELIABLE_PACKET: u32 = 1 << 15;

// packet types

pub enum net_packet_type_t {
    NET_PACKET_TYPE_SYN,
    NET_PACKET_TYPE_ACK,
    NET_PACKET_TYPE_REJECTED,
    NET_PACKET_TYPE_KEEPALIVE,
    NET_PACKET_TYPE_WAITING_DATA,
    NET_PACKET_TYPE_GAMESTART,
    NET_PACKET_TYPE_GAMEDATA,
    NET_PACKET_TYPE_GAMEDATA_ACK,
    NET_PACKET_TYPE_DISCONNECT,
    NET_PACKET_TYPE_DISCONNECT_ACK,
    NET_PACKET_TYPE_RELIABLE_ACK,
    NET_PACKET_TYPE_GAMEDATA_RESEND,
    NET_PACKET_TYPE_CONSOLE_MESSAGE,
    NET_PACKET_TYPE_QUERY,
    NET_PACKET_TYPE_QUERY_RESPONSE,
    NET_PACKET_TYPE_LAUNCH,
}

pub enum net_master_packet_type_t {
    NET_MASTER_PACKET_TYPE_ADD,
    NET_MASTER_PACKET_TYPE_ADD_RESPONSE,
    NET_MASTER_PACKET_TYPE_QUERY,
    NET_MASTER_PACKET_TYPE_QUERY_RESPONSE,
    NET_MASTER_PACKET_TYPE_GET_METADATA,
    NET_MASTER_PACKET_TYPE_GET_METADATA_RESPONSE,
    NET_MASTER_PACKET_TYPE_SIGN_START,
    NET_MASTER_PACKET_TYPE_SIGN_START_RESPONSE,
    NET_MASTER_PACKET_TYPE_SIGN_END,
    NET_MASTER_PACKET_TYPE_SIGN_END_RESPONSE,
}

// Settings specified when the client connects to the server.

pub struct net_connect_data_t {
    pub gamemode: i32,
    pub gamemission: i32,
    pub lowres_turn: i32,
    pub drone: i32,
    pub max_players: i32,
    pub is_freedoom: i32,
    pub wad_sha1sum: sha1_digest_t,
    pub deh_sha1sum: sha1_digest_t,
    pub player_class: i32,
}

// Game settings sent by client to server when initiating game start,
// and received from the server by clients when the game starts.

pub struct net_gamesettings_t {
    pub ticdup: i32,
    pub extratics: i32,
    pub deathmatch: i32,
    pub episode: i32,
    pub nomonsters: i32,
    pub fast_monsters: i32,
    pub respawn_monsters: i32,
    pub map: i32,
    pub skill: i32,
    pub gameversion: i32,
    pub lowres_turn: i32,
    pub new_sync: i32,
    pub timelimit: i32,
    pub loadgame: i32,
    pub random: i32, // [Strife only]

    // These fields are only used by the server when sending a game
    // start message:
    pub num_players: i32,
    pub consoleplayer: i32,

    // Hexen player classes:
    pub player_classes: [i32; NET_MAXPLAYERS],
}

impl net_gamesettings_t {
    pub fn new() -> Self {
        Self {
            ticdup: 0,
            extratics: 0,
            deathmatch: 0,
            episode: 0,
            nomonsters: 0,
            fast_monsters: 0,
            respawn_monsters: 0,
            map: 0,
            skill: 0,
            gameversion: 0,
            lowres_turn: 0,
            new_sync: 0,
            timelimit: 0,
            loadgame: 0,
            random: 0, // [Strife only]
            num_players: 0,
            consoleplayer: 0,
            player_classes: [0; NET_MAXPLAYERS],
        }
    }
}

pub const NET_TICDIFF_FORWARD: u8 = 1 << 0;
pub const NET_TICDIFF_SIDE: u8 = 1 << 1;
pub const NET_TICDIFF_TURN: u8 = 1 << 2;
pub const NET_TICDIFF_BUTTONS: u8 = 1 << 3;
pub const NET_TICDIFF_CONSISTANCY: u8 = 1 << 4;
pub const NET_TICDIFF_CHATCHAR: u8 = 1 << 5;
pub const NET_TICDIFF_RAVEN: u8 = 1 << 6;
pub const NET_TICDIFF_STRIFE: u8 = 1 << 7;

pub struct net_ticdiff_t {
    pub diff: u32,
    pub cmd: ticcmd_t,
}

// Complete set of ticcmds from all players

pub struct net_full_ticcmd_t {
    pub latency: i32,
    pub seq: u32,
    pub playeringame: [bool; NET_MAXPLAYERS],
    pub cmds: [net_ticdiff_t; NET_MAXPLAYERS],
}

// Data sent in response to server queries

pub struct net_querydata_t<'a> {
    pub version: &'a str,
    pub server_state: i32,
    pub num_players: i32,
    pub max_players: i32,
    pub gamemode: i32,
    pub gamemission: i32,
    pub description: &'a str,
}

// Data sent by the server while waiting for the game to start.

pub struct net_waitdata_t {
    pub num_players: i32,
    pub num_drones: i32,
    pub ready_players: i32,
    pub max_players: i32,
    pub is_controller: i32,
    pub consoleplayer: i32,
    pub player_names: [[char; NET_MAXPLAYERS]; MAXPLAYERNAME as usize],
    pub player_addrs: [[char; NET_MAXPLAYERS]; MAXPLAYERNAME as usize],
    pub wad_sha1sum: sha1_digest_t,
    pub deh_sha1sum: sha1_digest_t,
    pub is_freedoom: i32,
}
