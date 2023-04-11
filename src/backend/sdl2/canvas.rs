// revela::backend::sdl2::canvas
//
//!
//
// https://docs.rs/sdl2/latest/sdl2/struct.Sdl.html
// https://docs.rs/sdl2/latest/sdl2/struct.VideoSubsystem.html
// https://docs.rs/sdl2/latest/sdl2/video/struct.WindowBuilder.html
// https://docs.rs/sdl2/latest/sdl2/video/struct.Window.html
// https://docs.rs/sdl2/latest/sdl2/render/struct.CanvasBuilder.html
// https://docs.rs/sdl2/latest/sdl2/render/type.WindowCanvas.html
// https://docs.rs/sdl2/latest/sdl2/render/struct.Canvas.html

use core::fmt;

use crate::all::{
    Canvas, Position, RevelaError as Error, RevelaResult as Result, Sdl2Backend, Size,
};

use acolor::Srgba8;
#[cfg(feature = "depura")]
use depura::*;

#[cfg(not(feature = "safe"))]
use core::ptr;

use sdl2::{
    gfx::primitives::DrawRenderer,
    pixels::{Color as Sdl2Color, PixelFormatEnum},
    render::{BlendMode, Canvas as InnerSdl2Canvas},
    video::Window,
};
#[cfg(not(feature = "safe"))]
use sdl2::{
    libc::{c_int, c_void},
    sys::SDL_RenderReadPixels,
};

#[cfg(not(feature = "safe"))]
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

// TODO
// - https://docs.rs/sdl2/latest/sdl2/render/struct.Canvas.html#method.read_pixels
//
// - https://docs.rs/sdl2/latest/src/sdl2/render.rs.html#1441-1478
// - https://doc.rust-lang.org/std/vec/struct.Vec.html#method.set_len

/// `sdl2` [`Canvas`].
pub struct Sdl2Canvas {
    pub canvas: InnerSdl2Canvas<Window>, // Can't clone a window

    buffer: Vec<u8>,
    // buffer_gpu: Texture, // Maybe?
    width: u32,
    height: u32,
}

// impl fmt::Display for Sdl2Canvas {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}×{}({})", self.w(), self.h(), self.num_pixels())
//     }
// }
impl fmt::Debug for Sdl2Canvas {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Sdl2Canvas {{ {}×{} ({}) }}",
            self.w(),
            self.h(),
            self.num_pixels()
        )
    }
}

/// # constructors
impl Sdl2Canvas {
    /// Creates a new sdl canvas with the provided options
    pub fn new(
        sdl: &Sdl2Backend,
        size: impl Into<Size>,

        // TEMP
        hidden: bool,
        opengl: bool,
    ) -> Result<Self> {
        let size = size.into();
        let (w_u32, h_u32) = size.as_tuple_u32();
        let (w_usize, h_usize) = size.as_tuple_usize();

        let num_pixels = w_usize * h_usize;
        let num_bytes = num_pixels * 4;

        #[cfg(feature = "depura")]
        info![target: ">sdl2>canvas", "canvas size: {w_usize}×{h_usize}={num_pixels}"];
        #[cfg(feature = "depura")]
        info![target: ">sdl2>canvas", "opengl:{opengl}; hidden:{hidden}"];

        /* initialize buffer */

        // slower safe version
        #[cfg(feature = "safe")]
        let buffer = vec![0; num_bytes];

        // faster unsafe version
        #[cfg(not(feature = "safe"))]
        let buffer = {
            let mut v: Vec<u8> = Vec::with_capacity(num_bytes);
            for (_n, item) in v.spare_capacity_mut().iter_mut().enumerate() {
                item.write(0);
            }
            // SAFETY: All elements were initialized.
            unsafe { v.set_len(num_bytes) }
            v
        };

        /* configure sdl window canvas */

        // let video_subsys = sdl.video()?;
        // let window = video_subsys

        // TODO THINK about having multiple windows? save window handle?

        let mut window = sdl.sdl.video()?.window("ui-0", w_u32, h_u32);
        if hidden {
            window.hidden();
        }
        if opengl {
            window.opengl();
        }
        // window.position_centered();

        let window = window.build().map_err(|e| e.to_string())?;

        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        Ok(Self {
            buffer,
            canvas,
            width: w_u32,
            height: w_u32,
        })
    }
}

/// # methods
impl Sdl2Canvas {
    /* general */

    /// Returns the canvas width.
    #[inline]
    pub const fn w(&self) -> u32 {
        self.width
    }

    /// Returns the canvas height.
    #[inline]
    pub const fn h(&self) -> u32 {
        self.height
    }

    /// Returns the total number of pixels.
    #[inline]
    pub const fn num_pixels(&self) -> usize {
        self.width as usize * self.height as usize
    }

    /* color */

    /* drawing methods */
    // https://docs.rs/sdl2/latest/sdl2/render/struct.Canvas.html#impl-3

    /// Sets the color used for draw operations (Rect, Line & Clear).
    #[inline]
    pub fn set_color(&mut self, color: Sdl2Color) {
        self.canvas.set_draw_color(color);
    }

    /// Gets the color used for drawing operations (Rect, Line & Clear).
    #[inline]
    pub fn get_color(&self) -> Sdl2Color {
        self.canvas.draw_color()
    }

    ///
    #[inline]
    pub fn blend_mode(&self) -> BlendMode {
        self.canvas.blend_mode()
    }

    ///
    #[inline]
    pub fn set_blend_mode(&mut self, blend_mode: BlendMode) {
        self.canvas.set_blend_mode(blend_mode)
    }

    // /// Fills the canvas with a color.
    // pub fn fill(&mut self, color: impl Into<Color>) {
    //     #[cfg(feature = "depura")]
    //     trace_time![target: ">sdl2>canvas", "canvas.fill()"];
    //     self.canvas.set_draw_color(color.into());
    //     self.canvas.clear();
    // }

    /// Clears the canvas with the previously selected color
    #[inline]
    pub fn clear(&mut self) {
        self.canvas.clear();
    }

    /// Updates the screen with any rendering performed since the previous call.
    //
    // SDL’s rendering functions operate on a backbuffer; that is, calling a
    // rendering function such as draw_line() does not directly put a line on
    // the screen, but rather updates the backbuffer. As such, you compose your
    // entire scene and present the composed backbuffer to the screen as a
    // complete picture.
    #[inline]
    pub fn present(&mut self) {
        self.canvas.present()
    }

    // MAYBE NOT POSSIBLE?
    //
    // /// gets the pixel at the given coordinates.
    // pub fn pixel(&self) -> Color {
    //     todo![]
    // }

    /// sets the pixel at the given coordinates.
    #[inline]
    pub fn set_pixel(&mut self, x: i16, y: i16, color: Sdl2Color) -> Result<()> {
        Ok(self.canvas.pixel(x, y, color)?)
    }

    ///
    pub fn line(&mut self, x0: i16, y0: i16, x1: i16, y1: i16, color: Sdl2Color) -> Result<()> {
        Ok(self.canvas.line(x0, y0, x1, y1, color)?)
    }

    ///
    pub fn thick_line(
        &mut self,
        x0: i16,
        y0: i16,
        x1: i16,
        y1: i16,
        width: u8,
        color: Sdl2Color,
    ) -> Result<()> {
        Ok(self.canvas.thick_line(x0, y0, x1, y1, width, color)?)
    }

    // ///
    // pub fn line(&mut self, x0: i16, y0: i16, x1: i16, y1: i16, color: Color) -> Result<()> {
    //     self.canvas
    //         .line(x0, y0, x1, y1, color)
    //         .map_err(|e| e.into())
    // }

    /* exporting */

    /// Reads the pixels into a new vector.
    //
    // 8-10ms
    pub fn to_new_vec(&mut self) -> Result<Vec<u8>> {
        #[cfg(feature = "depura")]
        info_time![target: ">sdl2>canvas", "to_new_vec()"];
        self.canvas
            // .read_pixels(None, PixelFormatEnum::RGBA8888)
            .read_pixels(None, PixelFormatEnum::ABGR8888)
            .map_err(|e| e.into())
    }

    /// TEMP read_pixels
    pub fn read_pixels(&self) -> Result<Vec<u8>> {
        #[cfg(feature = "depura")]
        info_time![target: ">sdl2>canvas", "read_pixels()"];

        let v = self.canvas.read_pixels(None, PixelFormatEnum::ABGR8888)?;
        #[cfg(feature = "depura")]
        debug![target: ">sdl2>canvas", "{:?}", v.get(0..12)];

        Ok(v)
    }

    /// Reads the pixels into the inner buffer.
    //
    // 8-10ms
    // IMPROVE: safe version
    #[cfg(not(feature = "safe"))] // IMPROVE:safe:to_buffer
    pub fn to_buffer(&mut self) -> Result<()> {
        #[cfg(feature = "depura")]
        info_time![target: ">sdl2>canvas", "to_buffer()"];

        // https://wiki.libsdl.org/SDL_PixelFormatEnum
        //
        // let format = PixelFormatEnum::RGBA8888;
        let format = PixelFormatEnum::ABGR8888; // FIXES byte order !!!?

        // RGB888
        // #[cfg(feature = "depura")]
        // debug![target: ">sdl2>canvas", "default pixel format: {:?}",
        //     self.canvas.default_pixel_format()];

        let width = self.width as usize;
        let pitch = width * format.byte_size_per_pixel(); // * 4

        // FIX: doesn't work while rendering vlc! see old example `canvas-sdl-vlc-0.rs`
        let ret = unsafe {
            // https://docs.rs/sdl2-sys/latest/sdl2_sys/fn.SDL_RenderReadPixels.html
            SDL_RenderReadPixels(
                self.canvas.raw(),
                ptr::null(), // read the entire target
                format as u32,
                self.buffer.as_mut_ptr() as *mut c_void,
                pitch as c_int,
            )
        };

        // FIX: all black ?
        #[cfg(feature = "depura")]
        debug![target: ">sdl2>canvas", "{:?}", self.buffer.get(0..12)];

        if ret == 0 {
            Ok(())
        } else {
            Err(Error::String(sdl2::get_error()))
        }
    }

    /// Returns a shared reference to the inner byte buffer of Rgba pixels.
    ///
    /// You should have called `to_buffer` previously.
    pub fn as_slice(&self) -> &[u8] {
        self.buffer.as_slice()
    }

    /// Saves a screenshot in png format.
    ///
    /// You should have called `to_buffer` previously.
    //
    // IMPROVE:
    // - save to a configurable path
    // - add a timestamp, some metadata
    //
    // DOCS
    // - https://docs.rs/png/0.17.6/png/struct.Encoder.html
    // - https://docs.rs/png/0.17.6/png/struct.Writer.html
    //
    // ISSUES
    // - https://github.com/image-rs/image-png/issues/296 color format
    //
    #[cfg(not(feature = "safe"))] // IMPROVE:safe:to_buffer
    pub fn save_png(&mut self, path_str: &str) -> Result<()> {
        #[cfg(feature = "depura")]
        trace_time![target: ">sdl2>canvas", "save_png()"];

        use std::{fs::File, io::BufWriter, path::Path};

        self.to_buffer()?; // TEMP, IMPROVE use dirty bit or bool arg?

        let path = Path::new(path_str);
        let file = File::create(path)?;
        let w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.width, self.height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header()?;
        writer.write_image_data(&self.buffer)?;

        Ok(())
    }
}

#[cfg(not(feature = "safe"))]
unsafe impl HasRawWindowHandle for Sdl2Canvas {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.canvas.window().raw_window_handle()
    }
}

impl Canvas for Sdl2Canvas {
    type Color = Srgba8;

    fn get_pixel(&self, _p: Position) -> Result<Self::Color> {
        Err(Error::NotSupported)
    }

    fn set_pixel(&mut self, p: Position, c: Self::Color) -> Result<()> {
        let (x, y) = p.as_tuple_i16();
        self.set_pixel(x, y, c.into())
    }
}
