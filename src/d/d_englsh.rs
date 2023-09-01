#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

/////////////////////////////
// d_english.h
/////////////////////////////
//
// DESCRIPTION:
//	Printed strings for translation.
//	English language support (default).
//

//
//	Printed strings for translation
//

//
// D_Main.C
//
pub const D_DEVSTR: &str = "Development mode ON.\n";
pub const D_CDROM: &str = "CD-ROM Version: default.cfg from c:\\doomdata\n";

//
//	M_Menu.C
//
pub const PRESSKEY: &str = "press a key.";
pub const PRESSYN: &str = "press y or n.";
pub const QUITMSG: &str = "are you sure you want to\nquit this great game?";
pub const LOADNET: &str = "you can't do load while in a net game!\n\npress a key.";
pub const QLOADNET: &str = "you can't quickload during a netgame!\n\npress a key.";
pub const QSAVESPOT: &str = "you haven't picked a quicksave slot yet!\n\npress a key.";
pub const SAVEDEAD: &str = "you can't save if you aren't playing!\n\npress a key.";
pub const QSPROMPT: &str = "quicksave over your game named\n\n'%s'?\n\npress y or n.";
pub const QLPROMPT: &str = "do you want to quickload the game named\n\n'%s'?\n\npress y or n.";

pub const NEWGAME: &str = "you can't start a new game\n\
while in a network game.\n\npress a key.";

pub const NIGHTMARE: &str = "are you sure? this skill level\n\
isn't even remotely fair.\n\npress y or n.";

pub const SWSTRING: &str = "this is the shareware version of doom.\n\n\
you need to order the entire trilogy.\n\npress a key.";

pub const MSGOFF: &str = "Messages OFF";
pub const MSGON: &str = "Messages ON";
pub const NETEND: &str = "you can't end a netgame!\n\npress a key.";
pub const ENDGAME: &str = "are you sure you want to end the game?\n\npress y or n.";

pub const DOSY: &str = "(press y to quit to dos.)";

pub const DETAILHI: &str = "High detail";
pub const DETAILLO: &str = "Low detail";
pub const GAMMALVL0: &str = "Gamma correction OFF";
pub const GAMMALVL1: &str = "Gamma correction level 1";
pub const GAMMALVL2: &str = "Gamma correction level 2";
pub const GAMMALVL3: &str = "Gamma correction level 3";
pub const GAMMALVL4: &str = "Gamma correction level 4";
pub const EMPTYSTRING: &str = "empty slot";

//
//	P_inter.C
//
pub const GOTARMOR: &str = "Picked up the armor.";
pub const GOTMEGA: &str = "Picked up the MegaArmor!";
pub const GOTHTHBONUS: &str = "Picked up a health bonus.";
pub const GOTARMBONUS: &str = "Picked up an armor bonus.";
pub const GOTSTIM: &str = "Picked up a stimpack.";
pub const GOTMEDINEED: &str = "Picked up a medikit that you REALLY need!";
pub const GOTMEDIKIT: &str = "Picked up a medikit.";
pub const GOTSUPER: &str = "Supercharge!";

pub const GOTBLUECARD: &str = "Picked up a blue keycard.";
pub const GOTYELWCARD: &str = "Picked up a yellow keycard.";
pub const GOTREDCARD: &str = "Picked up a red keycard.";
pub const GOTBLUESKUL: &str = "Picked up a blue skull key.";
pub const GOTYELWSKUL: &str = "Picked up a yellow skull key.";
pub const GOTREDSKULL: &str = "Picked up a red skull key.";

pub const GOTINVUL: &str = "Invulnerability!";
pub const GOTBERSERK: &str = "Berserk!";
pub const GOTINVIS: &str = "Partial Invisibility";
pub const GOTSUIT: &str = "Radiation Shielding Suit";
pub const GOTMAP: &str = "Computer Area Map";
pub const GOTVISOR: &str = "Light Amplification Visor";
pub const GOTMSPHERE: &str = "MegaSphere!";

pub const GOTCLIP: &str = "Picked up a clip.";
pub const GOTCLIPBOX: &str = "Picked up a box of bullets.";
pub const GOTROCKET: &str = "Picked up a rocket.";
pub const GOTROCKBOX: &str = "Picked up a box of rockets.";
pub const GOTCELL: &str = "Picked up an energy cell.";
pub const GOTCELLBOX: &str = "Picked up an energy cell pack.";
pub const GOTSHELLS: &str = "Picked up 4 shotgun shells.";
pub const GOTSHELLBOX: &str = "Picked up a box of shotgun shells.";
pub const GOTBACKPACK: &str = "Picked up a backpack full of ammo!";

pub const GOTBFG9000: &str = "You got the BFG9000!  Oh, yes.";
pub const GOTCHAINGUN: &str = "You got the chaingun!";
pub const GOTCHAINSAW: &str = "A chainsaw!  Find some meat!";
pub const GOTLAUNCHER: &str = "You got the rocket launcher!";
pub const GOTPLASMA: &str = "You got the plasma gun!";
pub const GOTSHOTGUN: &str = "You got the shotgun!";
pub const GOTSHOTGUN2: &str = "You got the super shotgun!";

//
// P_Doors.C
//
pub const PD_BLUEO: &str = "You need a blue key to activate this object";
pub const PD_REDO: &str = "You need a red key to activate this object";
pub const PD_YELLOWO: &str = "You need a yellow key to activate this object";
pub const PD_BLUEK: &str = "You need a blue key to open this door";
pub const PD_REDK: &str = "You need a red key to open this door";
pub const PD_YELLOWK: &str = "You need a yellow key to open this door";

//
//	G_game.C
//
pub const GGSAVED: &str = "game saved.";

//
//	HU_stuff.C
//
pub const HUSTR_MSGU: &str = "[Message unsent]";

pub const HUSTR_E1M1: &str = "E1M1: Hangar";
pub const HUSTR_E1M2: &str = "E1M2: Nuclear Plant";
pub const HUSTR_E1M3: &str = "E1M3: Toxin Refinery";
pub const HUSTR_E1M4: &str = "E1M4: Command Control";
pub const HUSTR_E1M5: &str = "E1M5: Phobos Lab";
pub const HUSTR_E1M6: &str = "E1M6: Central Processing";
pub const HUSTR_E1M7: &str = "E1M7: Computer Station";
pub const HUSTR_E1M8: &str = "E1M8: Phobos Anomaly";
pub const HUSTR_E1M9: &str = "E1M9: Military Base";

pub const HUSTR_E2M1: &str = "E2M1: Deimos Anomaly";
pub const HUSTR_E2M2: &str = "E2M2: Containment Area";
pub const HUSTR_E2M3: &str = "E2M3: Refinery";
pub const HUSTR_E2M4: &str = "E2M4: Deimos Lab";
pub const HUSTR_E2M5: &str = "E2M5: Command Center";
pub const HUSTR_E2M6: &str = "E2M6: Halls of the Damned";
pub const HUSTR_E2M7: &str = "E2M7: Spawning Vats";
pub const HUSTR_E2M8: &str = "E2M8: Tower of Babel";
pub const HUSTR_E2M9: &str = "E2M9: Fortress of Mystery";

pub const HUSTR_E3M1: &str = "E3M1: Hell Keep";
pub const HUSTR_E3M2: &str = "E3M2: Slough of Despair";
pub const HUSTR_E3M3: &str = "E3M3: Pandemonium";
pub const HUSTR_E3M4: &str = "E3M4: House of Pain";
pub const HUSTR_E3M5: &str = "E3M5: Unholy Cathedral";
pub const HUSTR_E3M6: &str = "E3M6: Mt. Erebus";
pub const HUSTR_E3M7: &str = "E3M7: Limbo";
pub const HUSTR_E3M8: &str = "E3M8: Dis";
pub const HUSTR_E3M9: &str = "E3M9: Warrens";

pub const HUSTR_E4M1: &str = "E4M1: Hell Beneath";
pub const HUSTR_E4M2: &str = "E4M2: Perfect Hatred";
pub const HUSTR_E4M3: &str = "E4M3: Sever The Wicked";
pub const HUSTR_E4M4: &str = "E4M4: Unruly Evil";
pub const HUSTR_E4M5: &str = "E4M5: They Will Repent";
pub const HUSTR_E4M6: &str = "E4M6: Against Thee Wickedly";
pub const HUSTR_E4M7: &str = "E4M7: And Hell Followed";
pub const HUSTR_E4M8: &str = "E4M8: Unto The Cruel";
pub const HUSTR_E4M9: &str = "E4M9: Fear";

pub const HUSTR_1: &str = "level 1: entryway";
pub const HUSTR_2: &str = "level 2: underhalls";
pub const HUSTR_3: &str = "level 3: the gantlet";
pub const HUSTR_4: &str = "level 4: the focus";
pub const HUSTR_5: &str = "level 5: the waste tunnels";
pub const HUSTR_6: &str = "level 6: the crusher";
pub const HUSTR_7: &str = "level 7: dead simple";
pub const HUSTR_8: &str = "level 8: tricks and traps";
pub const HUSTR_9: &str = "level 9: the pit";
pub const HUSTR_10: &str = "level 10: refueling base";
pub const HUSTR_11: &str = "level 11: 'o' of destruction!";

pub const HUSTR_12: &str = "level 12: the factory";
pub const HUSTR_13: &str = "level 13: downtown";
pub const HUSTR_14: &str = "level 14: the inmost dens";
pub const HUSTR_15: &str = "level 15: industrial zone";
pub const HUSTR_16: &str = "level 16: suburbs";
pub const HUSTR_17: &str = "level 17: tenements";
pub const HUSTR_18: &str = "level 18: the courtyard";
pub const HUSTR_19: &str = "level 19: the citadel";
pub const HUSTR_20: &str = "level 20: gotcha!";

pub const HUSTR_21: &str = "level 21: nirvana";
pub const HUSTR_22: &str = "level 22: the catacombs";
pub const HUSTR_23: &str = "level 23: barrels o' fun";
pub const HUSTR_24: &str = "level 24: the chasm";
pub const HUSTR_25: &str = "level 25: bloodfalls";
pub const HUSTR_26: &str = "level 26: the abandoned mines";
pub const HUSTR_27: &str = "level 27: monster condo";
pub const HUSTR_28: &str = "level 28: the spirit world";
pub const HUSTR_29: &str = "level 29: the living end";
pub const HUSTR_30: &str = "level 30: icon of sin";

pub const HUSTR_31: &str = "level 31: wolfenstein";
pub const HUSTR_32: &str = "level 32: grosse";

pub const PHUSTR_1: &str = "level 1: congo";
pub const PHUSTR_2: &str = "level 2: well of souls";
pub const PHUSTR_3: &str = "level 3: aztec";
pub const PHUSTR_4: &str = "level 4: caged";
pub const PHUSTR_5: &str = "level 5: ghost town";
pub const PHUSTR_6: &str = "level 6: baron's lair";
pub const PHUSTR_7: &str = "level 7: caughtyard";
pub const PHUSTR_8: &str = "level 8: realm";
pub const PHUSTR_9: &str = "level 9: abattoire";
pub const PHUSTR_10: &str = "level 10: onslaught";
pub const PHUSTR_11: &str = "level 11: hunted";

pub const PHUSTR_12: &str = "level 12: speed";
pub const PHUSTR_13: &str = "level 13: the crypt";
pub const PHUSTR_14: &str = "level 14: genesis";
pub const PHUSTR_15: &str = "level 15: the twilight";
pub const PHUSTR_16: &str = "level 16: the omen";
pub const PHUSTR_17: &str = "level 17: compound";
pub const PHUSTR_18: &str = "level 18: neurosphere";
pub const PHUSTR_19: &str = "level 19: nme";
pub const PHUSTR_20: &str = "level 20: the death domain";

pub const PHUSTR_21: &str = "level 21: slayer";
pub const PHUSTR_22: &str = "level 22: impossible mission";
pub const PHUSTR_23: &str = "level 23: tombstone";
pub const PHUSTR_24: &str = "level 24: the final frontier";
pub const PHUSTR_25: &str = "level 25: the temple of darkness";
pub const PHUSTR_26: &str = "level 26: bunker";
pub const PHUSTR_27: &str = "level 27: anti-christ";
pub const PHUSTR_28: &str = "level 28: the sewers";
pub const PHUSTR_29: &str = "level 29: odyssey of noises";
pub const PHUSTR_30: &str = "level 30: the gateway of hell";

pub const PHUSTR_31: &str = "level 31: cyberden";
pub const PHUSTR_32: &str = "level 32: go 2 it";

pub const THUSTR_1: &str = "level 1: system control";
pub const THUSTR_2: &str = "level 2: human bbq";
pub const THUSTR_3: &str = "level 3: power control";
pub const THUSTR_4: &str = "level 4: wormhole";
pub const THUSTR_5: &str = "level 5: hanger";
pub const THUSTR_6: &str = "level 6: open season";
pub const THUSTR_7: &str = "level 7: prison";
pub const THUSTR_8: &str = "level 8: metal";
pub const THUSTR_9: &str = "level 9: stronghold";
pub const THUSTR_10: &str = "level 10: redemption";
pub const THUSTR_11: &str = "level 11: storage facility";

pub const THUSTR_12: &str = "level 12: crater";
pub const THUSTR_13: &str = "level 13: nukage processing";
pub const THUSTR_14: &str = "level 14: steel works";
pub const THUSTR_15: &str = "level 15: dead zone";
pub const THUSTR_16: &str = "level 16: deepest reaches";
pub const THUSTR_17: &str = "level 17: processing area";
pub const THUSTR_18: &str = "level 18: mill";
pub const THUSTR_19: &str = "level 19: shipping/respawning";
pub const THUSTR_20: &str = "level 20: central processing";

pub const THUSTR_21: &str = "level 21: administration center";
pub const THUSTR_22: &str = "level 22: habitat";
pub const THUSTR_23: &str = "level 23: lunar mining project";
pub const THUSTR_24: &str = "level 24: quarry";
pub const THUSTR_25: &str = "level 25: baron's den";
pub const THUSTR_26: &str = "level 26: ballistyx";
pub const THUSTR_27: &str = "level 27: mount pain";
pub const THUSTR_28: &str = "level 28: heck";
pub const THUSTR_29: &str = "level 29: river styx";
pub const THUSTR_30: &str = "level 30: last call";

pub const THUSTR_31: &str = "level 31: pharaoh";
pub const THUSTR_32: &str = "level 32: caribbean";

pub const HUSTR_CHATMACRO1: &str = "I'm ready to kick butt!";
pub const HUSTR_CHATMACRO2: &str = "I'm OK.";
pub const HUSTR_CHATMACRO3: &str = "I'm not looking too good!";
pub const HUSTR_CHATMACRO4: &str = "Help!";
pub const HUSTR_CHATMACRO5: &str = "You suck!";
pub const HUSTR_CHATMACRO6: &str = "Next time, scumbag...";
pub const HUSTR_CHATMACRO7: &str = "Come here!";
pub const HUSTR_CHATMACRO8: &str = "I'll take care of it.";
pub const HUSTR_CHATMACRO9: &str = "Yes";
pub const HUSTR_CHATMACRO0: &str = "No";

pub const HUSTR_TALKTOSELF1: &str = "You mumble to yourself";
pub const HUSTR_TALKTOSELF2: &str = "Who's there?";
pub const HUSTR_TALKTOSELF3: &str = "You scare yourself";
pub const HUSTR_TALKTOSELF4: &str = "You start to rave";
pub const HUSTR_TALKTOSELF5: &str = "You've lost it...";

pub const HUSTR_MESSAGESENT: &str = "[Message Sent]";

// The following should NOT be changed unless it seems
// just AWFULLY necessary

pub const HUSTR_PLRGREEN: &str = "Green: ";
pub const HUSTR_PLRINDIGO: &str = "Indigo: ";
pub const HUSTR_PLRBROWN: &str = "Brown: ";
pub const HUSTR_PLRRED: &str = "Red: ";

pub const HUSTR_KEYGREEN: char = 'g';
pub const HUSTR_KEYINDIGO: char = 'i';
pub const HUSTR_KEYBROWN: char = 'b';
pub const HUSTR_KEYRED: char = 'r';

//
//	AM_map.C
//

pub const AMSTR_FOLLOWON: &str = "Follow Mode ON";
pub const AMSTR_FOLLOWOFF: &str = "Follow Mode OFF";

pub const AMSTR_GRIDON: &str = "Grid ON";
pub const AMSTR_GRIDOFF: &str = "Grid OFF";

pub const AMSTR_MARKEDSPOT: &str = "Marked Spot";
pub const AMSTR_MARKSCLEARED: &str = "All Marks Cleared";

//
//	ST_stuff.C
//

pub const STSTR_MUS: &str = "Music Change";
pub const STSTR_NOMUS: &str = "IMPOSSIBLE SELECTION";
pub const STSTR_DQDON: &str = "Degreelessness Mode On";
pub const STSTR_DQDOFF: &str = "Degreelessness Mode Off";

pub const STSTR_KFAADDED: &str = "Very Happy Ammo Added";
pub const STSTR_FAADDED: &str = "Ammo (no keys) Added";

pub const STSTR_NCON: &str = "No Clipping Mode ON";
pub const STSTR_NCOFF: &str = "No Clipping Mode OFF";

pub const STSTR_BEHOLD: &str = "inVuln, Str, Inviso, Rad, Allmap, or Lite-amp";
pub const STSTR_BEHOLDX: &str = "Power-up Toggled";

pub const STSTR_CHOPPERS: &str = "... doesn't suck - GM";
pub const STSTR_CLEV: &str = "Changing Level...";

//
//	F_Finale.C
//
pub const E1TEXT: &str = "Once you beat the big badasses and\n\
clean out the moon base you're supposed\n\
to win, aren't you? Aren't you? Where's\n\
your fat reward and ticket home? What\n\
the hell is this? It's not supposed to\n\
end this way!\n\
\n \
It stinks like rotten meat, but looks\n\
like the lost Deimos base.  Looks like\n\
you're stuck on The Shores of Hell.\n\
The only way out is through.\n\
\n\
To continue the DOOM experience, play\n\
The Shores of Hell and its amazing\n\
sequel, Inferno!\n";

pub const E2TEXT: &str = "You've done it! The hideous cyber-\n\
demon lord that ruled the lost Deimos\n\
moon base has been slain and you\n\
are triumphant! But ... where are\n\
you? You clamber to the edge of the\n\
moon and look down to see the awful\n\
truth.\n \
\n\
Deimos floats above Hell itself!\n\
You've never heard of anyone escaping\n\
from Hell, but you'll make the bastards\n\
sorry they ever heard of you! Quickly,\n\
you rappel down to  the surface of\n\
Hell.\n\
\n \
Now, it's on to the final chapter of\n\
DOOM! -- Inferno.";

pub const E3TEXT: &str = "The loathsome spiderdemon that\n\
masterminded the invasion of the moon\n\
bases and caused so much death has had\n\
its ass kicked for all time.\n\
\n\
A hidden doorway opens and you enter.\n\
You've proven too tough for Hell to\n\
contain, and now Hell at last plays\n\
fair -- for you emerge from the door\n\
to see the green fields of Earth!\n\
Home at last.\n \
\n\
You wonder what's been happening on\n\
Earth while you were battling evil\n\
unleashed. It's good that no Hell-\n\
spawn could have come through that\n\
door with you ...";

pub const E4TEXT: &str = "the spider mastermind must have sent forth\n\
its legions of hellspawn before your\n\
final confrontation with that terrible\n\
beast from hell.  but you stepped forward\n\
and brought forth eternal damnation and\n\
suffering upon the horde as a true hero\n\
would in the face of something so evil.\n\
\n\
besides, someone was gonna pay for what\n\
happened to daisy, your pet rabbit.\n\
\n\
but now, you see spread before you more\n\
potential pain and gibbitude as a nation\n\
of demons run amok among our cities.\n\
\n\
next stop, hell on earth!";

// after level 6, put this:

pub const C1TEXT: &str = "YOU HAVE ENTERED DEEPLY INTO THE INFESTED\n \
STARPORT. BUT SOMETHING IS WRONG. THE\n \
MONSTERS HAVE BROUGHT THEIR OWN REALITY\n \
WITH THEM, AND THE STARPORT'S TECHNOLOGY\n \
IS BEING SUBVERTED BY THEIR PRESENCE.\n \
\n\
AHEAD, YOU SEE AN OUTPOST OF HELL, A\n \
FORTIFIED ZONE. IF YOU CAN GET PAST IT,\n \
YOU CAN PENETRATE INTO THE HAUNTED HEART\n \
OF THE STARBASE AND FIND THE CONTROLLING\n \
SWITCH WHICH HOLDS EARTH'S POPULATION\n \
HOSTAGE.";

// After level 11, put this:

pub const C2TEXT: &str = "YOU HAVE WON! YOUR VICTORY HAS ENABLED\n \
HUMANKIND TO EVACUATE EARTH AND ESCAPE\n\
THE NIGHTMARE.  NOW YOU ARE THE ONLY\n\
HUMAN LEFT ON THE FACE OF THE PLANET.\n\
CANNIBAL MUTATIONS, CARNIVOROUS ALIENS,\n\
AND EVIL SPIRITS ARE YOUR ONLY NEIGHBORS.\n\
YOU SIT BACK AND WAIT FOR DEATH, CONTENT\n\
THAT YOU HAVE SAVED YOUR SPECIES.\n\
\n\
BUT THEN, EARTH CONTROL BEAMS DOWN A\n\
MESSAGE FROM SPACE: \"SENSORS HAVE LOCATED\n\
THE SOURCE OF THE ALIEN INVASION. IF YOU\n\
GO THERE, YOU MAY BE ABLE TO BLOCK THEIR\n\
ENTRY.  THE ALIEN BASE IS IN THE HEART OF\n\
YOUR OWN HOME CITY, NOT FAR FROM THE\n\
STARPORT.\" SLOWLY AND PAINFULLY YOU GET\n\
UP AND RETURN TO THE FRAY.";

// After level 20, put this:

pub const C3TEXT: &str = "YOU ARE AT THE CORRUPT HEART OF THE CITY,\n\
SURROUNDED BY THE CORPSES OF YOUR ENEMIES.\n\
YOU SEE NO WAY TO DESTROY THE CREATURES'\n\
ENTRYWAY ON THIS SIDE, SO YOU CLENCH YOUR\n\
TEETH AND PLUNGE THROUGH IT.\n\
\n\
THERE MUST BE A WAY TO CLOSE IT ON THE\n\
OTHER SIDE. WHAT DO YOU CARE IF YOU'VE\n\
GOT TO GO THROUGH HELL TO GET TO IT?";

// After level 29, put this:

pub const C4TEXT: &str = "THE HORRENDOUS VISAGE OF THE BIGGEST\n\
DEMON YOU'VE EVER SEEN CRUMBLES BEFORE\n\
YOU, AFTER YOU PUMP YOUR ROCKETS INTO\n\
HIS EXPOSED BRAIN. THE MONSTER SHRIVELS\n\
UP AND DIES, ITS THRASHING LIMBS\n\
DEVASTATING UNTOLD MILES OF HELL'S\n\
SURFACE.\n\
\n\
YOU'VE DONE IT. THE INVASION IS OVER.\n\
EARTH IS SAVED. HELL IS A WRECK. YOU\n\
WONDER WHERE BAD FOLKS WILL GO WHEN THEY\n\
DIE, NOW. WIPING THE SWEAT FROM YOUR\n\
FOREHEAD YOU BEGIN THE LONG TREK BACK\n\
HOME. REBUILDING EARTH OUGHT TO BE A\n\
LOT MORE FUN THAN RUINING IT WAS.\n";

// Before level 31, put this:

pub const C5TEXT: &str = "CONGRATULATIONS, YOU'VE FOUND THE SECRET\n\
LEVEL! LOOKS LIKE IT'S BEEN BUILT BY\n\
HUMANS, RATHER THAN DEMONS. YOU WONDER\n\
WHO THE INMATES OF THIS CORNER OF HELL\n\
WILL BE.";

// Before level 32, put this:

pub const C6TEXT: &str = "CONGRATULATIONS, YOU'VE FOUND THE\n\
SUPER SECRET LEVEL!  YOU'D BETTER\n\
BLAZE THROUGH THIS ONE!\n";

// after map 06

pub const P1TEXT: &str = "You gloat over the steaming carcass of the\n\
Guardian.  With its death, you've wrested\n\
the Accelerator from the stinking claws\n\
of Hell.  You relax and glance around the\n\
room.  Damn!  There was supposed to be at\n\
least one working prototype, but you can't\n\
see it. The demons must have taken it.\n\
\n\
You must find the prototype, or all your\n\
struggles will have been wasted. Keep\n\
moving, keep fighting, keep killing.\n\
Oh yes, keep living, too.";

// after map 11

pub const P2TEXT: &str = "Even the deadly Arch-Vile labyrinth could\n\
not stop you, and you've gotten to the\n\
prototype Accelerator which is soon\n\
efficiently and permanently deactivated.\n\
\n\
You're good at that kind of thing.";

// after map 20

pub const P3TEXT: &str = "You've bashed and battered your way into\n\
the heart of the devil-hive.  Time for a\n\
Search-and-Destroy mission, aimed at the\n\
Gatekeeper, whose foul offspring is\n\
cascading to Earth.  Yeah, he's bad. But\n\
you know who's worse!\n\
\n\
Grinning evilly, you check your gear, and\n\
get ready to give the bastard a little Hell\n\
of your own making!";

// after map 30

pub const P4TEXT: &str = "The Gatekeeper's evil face is splattered\n\
all over the place.  As its tattered corpse\n\
collapses, an inverted Gate forms and\n\
sucks down the shards of the last\n\
prototype Accelerator, not to mention the\n\
few remaining demons.  You're done. Hell\n\
has gone back to pounding bad dead folks \n\
instead of good live ones.  Remember to\n\
tell your grandkids to put a rocket\n\
launcher in your coffin. If you go to Hell\n\
when you die, you'll need it for some\n\
final cleaning-up ...";

// before map 31

pub const P5TEXT: &str = "You've found the second-hardest level we\n\
got. Hope you have a saved game a level or\n\
two previous.  If not, be prepared to die\n\
aplenty. For master marines only.";

// before map 32

pub const P6TEXT: &str = "Betcha wondered just what WAS the hardest\n\
level we had ready for ya?  Now you know.\n\
No one gets out alive.";

pub const T1TEXT: &str = "You've fought your way out of the infested\n\
experimental labs.   It seems that UAC has\n\
once again gulped it down.  With their\n\
high turnover, it must be hard for poor\n\
old UAC to buy corporate health insurance\n\
nowadays..\n\
\n\
Ahead lies the military complex, now\n\
swarming with diseased horrors hot to get\n\
their teeth into you. With luck, the\n\
complex still has some warlike ordnance\n\
laying around.";

pub const T2TEXT: &str = "You hear the grinding of heavy machinery\n\
ahead.  You sure hope they're not stamping\n\
out new hellspawn, but you're ready to\n\
ream out a whole herd if you have to.\n\
They might be planning a blood feast, but\n\
you feel about as mean as two thousand\n\
maniacs packed into one mad killer.\n\
\n\
You don't plan to go down easy.";

pub const T3TEXT: &str = "The vista opening ahead looks real damn\n\
familiar. Smells familiar, too -- like\n\
fried excrement. You didn't like this\n\
place before, and you sure as hell ain't\n\
planning to like it now. The more you\n\
brood on it, the madder you get.\n\
Hefting your gun, an evil grin trickles\n\
onto your face. Time to take some names.";

pub const T4TEXT: &str = "Suddenly, all is silent, from one horizon\n\
to the other. The agonizing echo of Hell\n\
fades away, the nightmare sky turns to\n\
blue, the heaps of monster corpses start \n\
to evaporate along with the evil stench \n\
that filled the air. Jeeze, maybe you've\n\
done it. Have you really won?\n\
\n\
Something rumbles in the distance.\n\
A blue light begins to glow inside the\n\
ruined skull of the demon-spitter.";

pub const T5TEXT: &str = "What now? Looks totally different. Kind\n\
of like King Tut's condo. Well,\n\
whatever's here can't be any worse\n\
than usual. Can it?  Or maybe it's best\n\
to let sleeping gods lie..";

pub const T6TEXT: &str = "Time for a vacation. You've burst the\n\
bowels of hell and by golly you're ready\n\
for a break. You mutter to yourself,\n\
Maybe someone else can kick Hell's ass\n\
next time around. Ahead lies a quiet town,\n\
with peaceful flowing water, quaint\n\
buildings, and presumably no Hellspawn.\n\
\n\
As you step off the transport, you hear\n\
the stomp of a cyberdemon's iron shoe.";

//
// Character cast strings F_FINALE.C
//
pub const CC_ZOMBIE: &str = "ZOMBIEMAN";
pub const CC_SHOTGUN: &str = "SHOTGUN GUY";
pub const CC_HEAVY: &str = "HEAVY WEAPON DUDE";
pub const CC_IMP: &str = "IMP";
pub const CC_DEMON: &str = "DEMON";
pub const CC_LOST: &str = "LOST SOUL";
pub const CC_CACO: &str = "CACODEMON";
pub const CC_HELL: &str = "HELL KNIGHT";
pub const CC_BARON: &str = "BARON OF HELL";
pub const CC_ARACH: &str = "ARACHNOTRON";
pub const CC_PAIN: &str = "PAIN ELEMENTAL";
pub const CC_REVEN: &str = "REVENANT";
pub const CC_MANCU: &str = "MANCUBUS";
pub const CC_ARCH: &str = "ARCH-VILE";
pub const CC_SPIDER: &str = "THE SPIDER MASTERMIND";
pub const CC_CYBER: &str = "THE CYBERDEMON";
pub const CC_HERO: &str = "OUR HERO";
