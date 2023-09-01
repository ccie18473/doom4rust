#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// i_system.h
/////////////////////////////
//
// DESCRIPTION:
//	System specific interface stuff.
//

pub type atexit_func_t = fn();

fn func(doom: &mut modules) {}

/////////////////////////////
// i_system.c
/////////////////////////////
//
// DESCRIPTION:
//

pub const DEFAULT_RAM: i32 = 6; /* MiB */
pub const MIN_RAM: i32 = 6; /* MiB */

pub type atexit_listentry_t = atexit_listentry_s;

pub struct atexit_listentry_s {
    pub func: fn(&mut modules),
    pub run_on_error: bool,
    pub next: *mut atexit_listentry_t,
}

impl atexit_listentry_s {
    pub fn new() -> Self {
        Self {
            func,
            run_on_error: false,
            next: ptr::null_mut(),
        }
    }
}

pub fn I_AtExit(doom: &mut modules, func: fn(&mut modules), run_on_error: bool) {
    println!("I_AtExit");

    //entry = malloc(sizeof(*entry));
    let mut entry1 = atexit_listentry_s::new();
    let entry: *mut atexit_listentry_t = &mut entry1;

    unsafe {
        (*entry).func = func;
        (*entry).run_on_error = run_on_error;
        (*entry).next = doom.i.exit_funcs;
        doom.i.exit_funcs = entry;
    }
}

pub fn I_Tactile(on: i32, off: i32, total: i32) {}

// Zone memory auto-allocation function that allocates the zone size
// by trying progressively smaller zone sizes until one is found that
// works.

pub fn AutoAllocMemory(size: &mut i32, default_ram: &mut i32, min_ram: &mut i32) -> *mut u8 {
    println!("AutoAllocMemory");

    return ptr::null_mut();
}

pub fn I_ZoneBase(doom: &mut modules, size: &mut i32) -> *mut u8 {
    println!("I_ZoneBase");

    return ptr::null_mut();
}

pub fn I_PrintBanner(msg: &str) {
    println!("I_PrintBanner");

    let spaces = 35 - (msg.len() / 2);

    for _i in 0..spaces {
        print!("{}", ' ');
    }

    println!("{}", msg);
}

pub fn I_PrintDivider() {
    println!("I_PrintDivider");

    for i in 0..75 {
        print!("=");
    }

    print!("\n");
}

pub fn I_PrintStartupBanner(gamedescription: &str) {
    println!("I_PrintStartupBanner");

    I_PrintDivider();
    I_PrintBanner(gamedescription);
    I_PrintDivider();

    println!(
        "{} is free software, covered by the GNU General Public\n\
    License.  There is NO warranty; not even for MERCHANTABILITY or FITNESS\n\
    FOR A PARTICULAR PURPOSE. You are welcome to change and distribute\n\
    copies under certain conditions. See the source for more information.",
        PACKAGE_NAME
    );

    I_PrintDivider();
}

//
// I_ConsoleStdout
//
// Returns true if stdout is a real console, false if it is a file
//

pub fn I_ConsoleStdout() -> bool {
    println!("I_ConsoleStdout");

    return false;
}

//
// I_Init
//
/*
void I_Init (void)
{
    I_CheckIsScreensaver();
    I_InitTimer();
    I_InitJoystick();
}
void I_BindVariables(void)
{
    I_BindVideoVariables();
    I_BindJoystickVariables();
    I_BindSoundVariables();
}
*/

//
// I_Quit
//

pub fn I_Quit() {
    println!("I_Quit");
}

//
// I_Error
//

pub fn I_Error() {
    println!("I_Error");

    exit(1);
}

//
// Read Access Violation emulation.
//
// From PrBoom+, by entryway.
//

// C:\>debug
// -d 0:0
//
// DOS 6.22:
// 0000:0000  (57 92 19 00) F4 06 70 00-(16 00)
// DOS 7.1:
// 0000:0000  (9E 0F C9 00) 65 04 70 00-(16 00)
// Win98:
// 0000:0000  (9E 0F C9 00) 65 04 70 00-(16 00)
// DOSBox under XP:
// 0000:0000  (00 00 00 F1) ?? ?? ?? 00-(07 00)

pub const DOS_MEM_DUMP_SIZE: usize = 10;

pub const mem_dump_dos622: [u8; DOS_MEM_DUMP_SIZE] =
    [0x57, 0x92, 0x19, 0x00, 0xF4, 0x06, 0x70, 0x00, 0x16, 0x00];

pub const mem_dump_win98: [u8; DOS_MEM_DUMP_SIZE] =
    [0x9E, 0x0F, 0xC9, 0x00, 0x65, 0x04, 0x70, 0x00, 0x16, 0x00];

pub const mem_dump_dosbox: [u8; DOS_MEM_DUMP_SIZE] =
    [0x00, 0x00, 0x00, 0xF1, 0x00, 0x00, 0x00, 0x00, 0x07, 0x00];

//pub const mem_dump_custom: [u8;DOS_MEM_DUMP_SIZE];

pub const dos_mem_dump: [u8; DOS_MEM_DUMP_SIZE] = mem_dump_dos622;

pub fn I_GetMemoryValue(offset: u32, value: *mut libc::c_void, size: i32) -> bool {
    println!("I_GetMemoryValue");

    return false;
}
