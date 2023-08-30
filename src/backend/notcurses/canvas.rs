// revela::backend::notcurses::canvas
//
//!
//

use core::fmt;

use acolor::all::Srgba8;

#[cfg(feature = "depura")]
use depura::*;

use notcurses::{Blitter, Plane, Rgba, Visual};

use crate::all::{Canvas, NotcursesBackend, Position, RevelaResult as Result, Size};

/// `notcurses` [`Canvas`] layer.
pub struct NotcursesCanvas {
    pub plane: Plane,
    pub visual: Visual,

    // THINK IMPROVE: integrate with the Layout system.
    width: u32,
    height: u32,
}

// - show ratio E.g.: 640×360(16:9×40)[230_400]
// impl fmt::Display for NotcursesCanvas {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}×{}({})", self.w(), self.h(), self.num_pixels())
//     }
// }
impl fmt::Debug for NotcursesCanvas {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "NotcursesCanvas {{ {}×{} ({}) }}",
            self.w(),
            self.h(),
            self.num_pixels()
        )
    }
}

/// # constructors
//
// TODO: add more constructors, maybe a Builder
//
impl NotcursesCanvas {
    /// Creates a new pixel canvas with the provided size and color
    ///
    /// NOTE: gracefully degrades to the best blitter available,
    /// pixels by default
    //
    // CHECK when pixels are not supported
    //
    // - check pixel size requested is available in the screen?
    // - check if pixel is supported and return error?
    //   - maybe return error result
    //   - or return the canvas with chosen blitter and metadata
    //   - or create new specialized methods:
    //     - new_pixel
    //     - new_blitter
    pub fn new(nc: &mut NotcursesBackend, size: impl Into<Size>) -> Result<Self> {
        let size = size.into();
        let (w_usize, h_usize) = size.as_tuple_usize();
        let (w_u32, h_u32) = size.as_tuple_u32();

        let num_pixels = w_usize * h_usize;
        let num_bytes = num_pixels * 4;

        #[cfg(feature = "depura")]
        trace![target: ">nc>canvas", "canvas size: {w_usize}×{h_usize}={num_pixels}"];

        /* prepare vec buffer (mainly for sdl) */
        // TODO
        // 1. create Visual::new() in notcurses. (all at 0) (& with_color)
        // 2. new constructor: from_buffer / from_rgba_slice

        // slower
        #[cfg(not(feature = "unsafe"))]
        let buffer = vec![255; num_bytes];

        // faster
        #[cfg(feature = "unsafe")]
        let buffer = {
            let mut v: Vec<u8> = Vec::with_capacity(num_bytes);
            for (_n, item) in v.spare_capacity_mut().iter_mut().enumerate() {
                item.write(255);
            }
            // SAFETY: All elements were initialized.
            unsafe { v.set_len(num_bytes) }
            v
        };

        /* prepare visual */

        // TODO: from provided buffer
        let mut visual = Visual::from_rgba(buffer.as_slice(), (w_u32, h_u32))?;

        // IMPROVE:
        visual.set_interpolate(false);
        // visual.set_degrade(false); // fails if there's no pixel support
        visual.set_blitter(Blitter::Pixel); // remove set_blitter_pixel method?

        // Blit the visual to a new plane (MAYBE move this to a separate method?)
        let plane = visual.blit(nc.mut_inner())?;

        // maybe more post-processing, in other `new_` methods. e.g. scaling…

        Ok(Self {
            width: w_u32,
            height: h_u32,

            plane,
            visual,
        })
    }
}

// MAYBE divide impl blocks in notcurses, tiny-skia

/// # methods
impl NotcursesCanvas {
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

    // set_color
    // get_color
    // blend_mode
    // set_blend_mode
    //
    // clear | ?
    // present | render?

    // set_pixel
    // line
    // thick_line

    /* exporting */

    // ? from Visual?

    /// Loads an RGBA8 buffer into a new visual.
    ///
    //
    // IMPROVE name? recreate_visual, …
    // pub fn reload_external_buffer()
    #[inline]
    pub fn reload_from_rgba8(&mut self, rgba_buffer: &[u8]) -> Result<()> {
        // TODO
        // let _visual = Visual::from_rgba(self.buffer.as_slice(), (self.width, self.height))?;

        debug_assert_eq![
            rgba_buffer.len(),
            self.width as usize * self.height as usize * 4
        ];
        self.visual = Visual::from_rgba(rgba_buffer, (self.width, self.height))?;

        // TODO: apply options from a different method, save preferences somewhere
        // options
        self.visual.set_interpolate(false);
        // visual.set_degrade(false); // fails if there's no pixel support
        self.visual.set_blitter(Blitter::Pixel); // remove set_blitter_pixel method?

        // let plane = visual.blit(nc)?; // MAYBE NOT
        Ok(())
    }

    /// Loads an RGB8 buffer into a new visual.
    ///
    #[inline]
    pub fn reload_from_rgb8(&mut self, rgb_buffer: &[u8]) -> Result<()> {
        debug_assert_eq![
            rgb_buffer.len(),
            self.width as usize * self.height as usize * 3
        ];
        self.visual = Visual::from_rgb(rgb_buffer, (self.width, self.height), 255)?;
        self.visual.set_interpolate(false);
        // visual.set_degrade(false); // fails if there's no pixel support
        self.visual.set_blitter(Blitter::Pixel); // remove set_blitter_pixel method?
        Ok(())
    }

    /// Blits the canvas.
    #[inline]
    pub fn blit(&mut self, nc: &mut NotcursesBackend) -> Result<()> {
        Ok(self.visual.blit_plane(nc.mut_inner(), &mut self.plane)?)
    }

    /// Renders the canvas from the already blitted plane.
    #[inline]
    pub fn render(&mut self) -> Result<()> {
        Ok(self.plane.render()?)
    }

    /// Blits & Renders the canvas.
    #[inline]
    pub fn blit_render(&mut self, nc: &mut NotcursesBackend) -> Result<()> {
        self.visual.blit_plane(nc.mut_inner(), &mut self.plane)?;
        Ok(self.plane.render()?)
    }

    /// gets a pixel from the notcurses Visual.
    pub fn get_pixel(&self, x: u32, y: u32) -> Result<Rgba> {
        Ok(self.visual.get_pixel(x, y)?)
    }

    /// sets a pixel from the notcurses Visual.
    pub fn set_pixel(&mut self, x: u32, y: u32, pixel: impl Into<Rgba>) -> Result<()> {
        Ok(self.visual.set_pixel(x, y, pixel.into())?)
    }
}

impl Canvas for NotcursesCanvas {
    type Color = Srgba8;

    fn get_pixel(&self, p: Position) -> Result<Self::Color> {
        let t = p.as_tuple_u32();
        self.get_pixel(t.0, t.1)
            .map(|c| Self::Color::from_array(c.into()))
    }

    fn set_pixel(&mut self, p: Position, c: Self::Color) -> Result<()> {
        let t = p.as_tuple_u32();
        self.set_pixel(t.0, t.1, Self::Color::to_array(c))
    }
}
