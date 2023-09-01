#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// i_video.h
/////////////////////////////
//
// DESCRIPTION:
//	System specific interface stuff.
//

// Screen width and height.

pub const SCREENWIDTH: i32 = 320;
pub const SCREENHEIGHT: i32 = 200;

// Screen width used for "squash" scale functions

pub const SCREENWIDTH_4_3: i32 = 256;

// Screen height used for "stretch" scale functions.

pub const SCREENHEIGHT_4_3: i32 = 240;

pub const MAX_MOUSE_BUTTONS: u8 = 8;

type InitMode_t = fn(palette: *mut u8);
type DrawScreen_t = fn(x1: i32, y1: i32, x2: i32, y2: i32);

pub struct screen_mode_t {
    // Screen width and height
    pub width: i32,
    pub height: i32,

    // Initialisation function to call when using this mode.
    // Called with a pointer to the Doom palette.
    //
    // If NULL, no init function is called.
    pub InitMode: InitMode_t,

    // Function to call to draw the screen from the source buffer.
    // Return true if draw was successful.
    pub DrawScreen: DrawScreen_t,

    // If true, this is a "poor quality" mode.  The autoadjust
    // code should always attempt to use a different mode to this
    // mode in fullscreen.
    //
    // Some notes about what "poor quality" means in this context:
    //
    // The aspect ratio correction works by scaling up to the larger
    // screen size and then drawing pixels on the edges between the
    // "virtual" pixels so that an authentic blocky look-and-feel is
    // achieved.
    //
    // For a mode like 640x480, you can imagine the grid of the
    // "original" pixels spaced out, with extra "blurry" pixels added
    // in the space between them to fill it out. However, when you're
    // running at a resolution like 320x240, this is not the case. In
    // the small screen case, every single pixel has to be a blurry
    // interpolation of two pixels from the original image.
    //
    // If you run in 320x240 and put your face up close to the screen
    // you can see this: it's particularly visible in the small yellow
    // status bar numbers for example. Overall it still looks "okay"
    // but there's an obvious - albeit small - deterioration in
    // quality.
    //
    // Once you get to 640x480, all the original pixels are there at
    // least once and it's okay (the higher the resolution, the more
    // accurate it is). When I first wrote the code I was expecting
    // that even higher resolutions would be needed before it would
    // look acceptable, but it turned out to be okay even at 640x480.
    pub poor_quality: bool,
}

type grabmouse_callback_t = fn() -> bool;

/////////////////////////////
// i_video.c
/////////////////////////////
//
// DESCRIPTION:
//	DOOM graphics stuff for FBs, UNIX.
//

pub const SDL_RESX: u32 = 320 * 3; //FIXME: Do scaling via arg in video interface
pub const SDL_RESY: u32 = 200 * 3;

pub struct FB_BitField {
    pub offset: u32, /* beginning of bitfield	*/
    pub length: u32, /* length of bitfield		*/
}

impl FB_BitField {
    pub fn new() -> Self {
        Self {
            offset: 0,
            length: 0,
        }
    }
}

pub struct FB_ScreenInfo {
    pub xres: u32, /* visible resolution		*/
    pub yres: u32,
    pub xres_virtual: u32, /* virtual resolution		*/
    pub yres_virtual: u32,

    pub bits_per_pixel: u32, /* guess what			*/

    /* >1 = FOURCC			*/
    pub red: FB_BitField,   /* bitfield in s_Fb mem if true color, */
    pub green: FB_BitField, /* else only length is significant */
    pub blue: FB_BitField,
    pub transp: FB_BitField, /* transparency			*/
}

impl FB_ScreenInfo {
    pub fn new() -> Self {
        Self {
            xres: 0,
            yres: 0,
            xres_virtual: 0,
            yres_virtual: 0,
            bits_per_pixel: 0,
            red: FB_BitField::new(),
            green: FB_BitField::new(),
            blue: FB_BitField::new(),
            transp: FB_BitField::new(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct color {
    pub b: u32,
    pub g: u32,
    pub r: u32,
    pub a: u32,
}

pub struct col_t {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

//
// I_SetPalette
//
//#define GFX_RGB565(r, g, b)			((((r & 0xF8) >> 3) << 11) | (((g & 0xFC) >> 2) << 5) | ((b & 0xF8) >> 3))
//#define GFX_RGB565_R(color)			((0xF800 & color) >> 11)
//#define GFX_RGB565_G(color)			((0x07E0 & color) >> 5)
//#define GFX_RGB565_B(color)			(0x001F & color)

pub fn cmap_to_rgb565(out: *mut u16, In: *mut u8, in_pixels: i32) {
    println!("cmap_to_rgb565");
}

pub fn cmap_to_fb(doom: &mut modules, mut out: *mut u8, mut In: *mut u8, in_pixels: i32) {
    println!("cmap_to_fb");

    let mut c: color;
    let mut pix: u32;
    let mut r: u16;
    let mut g: u16;
    let mut b: u16;

    for i in 0..in_pixels {
        unsafe { c = doom.i.colors[*In as usize] }; /* R:8 G:8 B:8 format! */
        r = (c.r >> (8 - doom.i.s_Fb.red.length)) as u16;
        g = (c.g >> (8 - doom.i.s_Fb.green.length)) as u16;
        b = (c.b >> (8 - doom.i.s_Fb.blue.length)) as u16;
        pix = (r as u32) << doom.i.s_Fb.red.offset;
        pix |= (g as u32) << doom.i.s_Fb.green.offset;
        pix |= (b as u32) << doom.i.s_Fb.blue.offset;

        for k in 0..doom.i.fb_scaling {
            for j in 0..doom.i.s_Fb.bits_per_pixel / 8 {
                unsafe {
                    *out = (pix >> (j * 8)) as u8;
                    out = out.add(1);
                };
            }
        }
        unsafe { In = In.add(1) };
    }
}

pub fn I_InitGraphics(doom: &mut modules) {
    println!("I_InitGraphics");

    let i: bool;

    //memset(&s_Fb, 0, sizeof(struct FB_ScreenInfo));
    doom.i.s_Fb.xres = SDL_RESX;
    doom.i.s_Fb.yres = SDL_RESY;
    doom.i.s_Fb.xres_virtual = doom.i.s_Fb.xres;
    doom.i.s_Fb.yres_virtual = doom.i.s_Fb.yres;
    doom.i.s_Fb.bits_per_pixel = 32;

    doom.i.s_Fb.blue.length = 8;
    doom.i.s_Fb.green.length = 8;
    doom.i.s_Fb.red.length = 8;
    doom.i.s_Fb.transp.length = 8;

    doom.i.s_Fb.blue.offset = 0;
    doom.i.s_Fb.green.offset = 8;
    doom.i.s_Fb.red.offset = 16;
    doom.i.s_Fb.transp.offset = 24;

    println!(
        "I_InitGraphics: framebuffer: x_res: {}, y_res: {}, x_virtual: {}, y_virtual: {}, bpp: {}",
        doom.i.s_Fb.xres,
        doom.i.s_Fb.yres,
        doom.i.s_Fb.xres_virtual,
        doom.i.s_Fb.yres_virtual,
        doom.i.s_Fb.bits_per_pixel
    );

    println!("I_InitGraphics: framebuffer: RGBA: {}{}{}{}, red_off: {}, green_off: {}, blue_off: {}, transp_off: {}",
            doom.i.s_Fb.red.length,
            doom.i.s_Fb.green.length,
            doom.i.s_Fb.blue.length,
            doom.i.s_Fb.transp.length,
            doom.i.s_Fb.red.offset,
            doom.i.s_Fb.green.offset,
            doom.i.s_Fb.blue.offset,
            doom.i.s_Fb.transp.offset);

    println!(
        "I_InitGraphics: DOOM screen size: w x h: {} x {}",
        SCREENWIDTH, SCREENHEIGHT
    );

    i = i32_to_bool(M_CheckParmWithArgs(doom, "-scaling", 1));
    if i {
        //i = atoi(myargv[i + 1]);
        doom.i.fb_scaling = i as i32;
        println!("I_InitGraphics: Scaling factor: {}", doom.i.fb_scaling);
    } else {
        doom.i.fb_scaling = doom.i.s_Fb.xres as i32 / SCREENWIDTH;
        if doom.i.s_Fb.yres as i32 / SCREENHEIGHT < doom.i.fb_scaling {
            doom.i.fb_scaling = doom.i.s_Fb.yres as i32 / SCREENHEIGHT;
        }
        println!("I_InitGraphics: Auto-scaling factor: {}", doom.i.fb_scaling);
    }

    /* Allocate screen to draw to */
    doom.i.I_VideoBuffer = Z_Malloc(
        SCREENWIDTH * SCREENHEIGHT,
        PURGE::PU_STATIC as i32,
        ptr::null_mut(),
    ) as *mut u8; // For DOOM to draw on

    doom.i.screenvisible = true;
    /*
    if (SDL_Init(SDL_INIT_VIDEO) < 0)
    {
        printf("I_InitGraphics: SDL_Init failed: %s\n", SDL_GetError());
        atexit(SDL_Quit);
        exit(1);
    }
    else
    {
        window = SDL_CreateWindow("DOOM", SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED,
            SDL_RESX, SDL_RESY, SDL_WINDOW_SHOWN);
        renderer =  SDL_CreateRenderer(window, -1, SDL_RENDERER_ACCELERATED);
        texture = SDL_CreateTexture(renderer, SDL_PIXELFORMAT_RGB888, SDL_TEXTUREACCESS_TARGET,
            SDL_RESX, SDL_RESY);

        fb_SDL = malloc(SDL_RESX * SDL_RESY * sizeof(uint32_t));

        SDL_RenderClear(renderer);
        SDL_RenderPresent(renderer);

        println!("I_InitGraphics: Init SDL video.\n");
    }
    */
    unsafe {
        doom.i.fb_SDL =
            libc::malloc((SDL_RESX * SDL_RESY) as usize * mem::size_of::<u32>()) as *mut u32;
    }

    println!("I_InitGraphics: Init SDL video.");

    I_InitInput();
}

pub fn I_ShutdownGraphics() {
    println!("I_ShutdownGraphics");
}

pub fn I_StartFrame() {
    println!("I_StartFrame");
}

pub fn I_StartTic() {
    println!("I_StartTic");

    I_GetEvent();
}

pub fn I_UpdateNoBlit() {
    println!("I_UpdateNoBlit");
}

//
// I_FinishUpdate
//

pub fn I_FinishUpdate(doom: &mut modules) {
    println!("I_FinishUpdate");

    let mut y: i32;
    let x_offset: i32;
    let y_offset: i32;
    let x_offset_end: i32;
    let mut line_in: *mut u8;
    let mut line_out: *mut u8;

    /* Offsets in case FB is bigger than DOOM */
    /* 600 = s_Fb height, 200 screenheight */
    /* 600 = s_Fb height, 200 screenheight */
    /* 2048 =s_Fb width, 320 screenwidth */
    y_offset = ((doom.i.s_Fb.yres as i32 - (SCREENHEIGHT * doom.i.fb_scaling))
        * doom.i.s_Fb.bits_per_pixel as i32
        / 8)
        / 2;
    x_offset = ((doom.i.s_Fb.xres as i32 - (SCREENWIDTH * doom.i.fb_scaling))
        * doom.i.s_Fb.bits_per_pixel as i32
        / 8)
        / 2; // XXX: siglent FB hack: /4 instead of /2, since it seems to handle the resolution in a funny way
             //x_offset     = 0;
    x_offset_end = ((doom.i.s_Fb.xres as i32 - (SCREENWIDTH * doom.i.fb_scaling))
        * doom.i.s_Fb.bits_per_pixel as i32
        / 8)
        - x_offset;
    /* DRAW SCREEN */
    line_in = doom.i.I_VideoBuffer;
    line_out = doom.i.fb_SDL as *mut u8;

    y = SCREENHEIGHT;

    while y > 0 {
        y -= 1;

        for i in 0..doom.i.fb_scaling {
            unsafe { line_out = line_out.add(x_offset as usize) };
            //cmap_to_rgb565((void*)line_out, (void*)line_in, SCREENWIDTH);
            cmap_to_fb(doom, line_out, line_in, SCREENWIDTH);
            unsafe {
                line_out = line_out.add(
                    ((SCREENWIDTH * doom.i.fb_scaling * (doom.i.s_Fb.bits_per_pixel as i32 / 8))
                        + x_offset_end) as usize,
                );
            };
        }
        unsafe { line_in = line_in.add(SCREENWIDTH as usize) };
    }

    let texture_creator = doom.i.renderer.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::ARGB8888, SDL_RESX, SDL_RESY)
        .map_err(|e| e.to_string())
        .unwrap();
    //BUG
    let slice: &[u8];
    unsafe { slice = slice::from_raw_parts(doom.i.fb_SDL as *mut u8, 2304000) }; //w:960 x h:600 x b:4

    texture
        .update(None, &slice, SDL_RESX as usize * mem::size_of::<u32>())
        .unwrap();

    doom.i.renderer.clear();
    doom.i.renderer.copy(&texture, None, None).unwrap();
    doom.i.renderer.present();

    std::thread::sleep(Duration::from_secs(10));
}

//
// I_ReadScreen
//
pub fn I_ReadScreen(scr: *mut u8) {
    println!("I_ReadScreen");
}

pub fn I_SetPalette(doom: &mut modules, mut palette: *mut u8) {
    println!("I_SetPalette");

    //col_t* c;

    //for (i = 0; i < 256; i++)
    //{
    //	c = (col_t*)palette;

    //	rgb565_palette[i] = GFX_RGB565(gammatable[usegamma][c->r],
    //								   gammatable[usegamma][c->g],
    //								   gammatable[usegamma][c->b]);

    //	palette += 3;
    //}

    /* performance boost:
     * map to the right pixel format over here! */
    unsafe {
        for i in 0..256 {
            doom.i.colors[i].a = 0;
            doom.i.colors[i].r = gammatable[doom.i.usegamma as usize][*palette as usize] as u32;
            palette = palette.add(1);
            doom.i.colors[i].g = gammatable[doom.i.usegamma as usize][*palette as usize] as u32;
            palette = palette.add(1);
            doom.i.colors[i].b = gammatable[doom.i.usegamma as usize][*palette as usize] as u32;
            palette = palette.add(1);
        }
    }
}

// Given an RGB value, find the closest matching palette index.

pub fn I_GetPaletteIndex(r: i32, g: i32, b: i32) -> i32 {
    println!("I_GetPaletteIndex");

    return 0;
}

pub fn I_BeginRead() {
    println!("I_BeginRead");
}

pub fn I_EndRead() {
    println!("I_EndRead");
}

pub fn I_SetWindowTitle(title: &str) {
    println!("I_SetWindowTitle");
}

pub fn I_GraphicsCheckCommandLine() {
    println!("I_GraphicsCheckCommandLine");
}

pub fn I_SetGrabMouseCallback(func: grabmouse_callback_t) {
    println!("I_SetGrabMouseCallback");
}

pub fn I_EnableLoadingDisk() {
    println!("I_EnableLoadingDisk");
}

pub fn I_BindVideoVariables() {
    println!("I_BindVideoVariables");
}

pub fn I_DisplayFPSDots(dots_on: bool) {
    println!("I_DisplayFPSDots");
}

pub fn I_CheckIsScreensaver() {
    println!("I_CheckIsScreensaver");
}
