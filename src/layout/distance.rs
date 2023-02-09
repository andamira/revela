// revela::layout::distance
//
//!
//

// IMPROVE WAIT when trait Ord::clamp can be used in const
#![allow(clippy::manual_clamp)]

/// Enforcement of boundaries for horizontal & vertical distances.
pub struct Distance;

impl Distance {
    // These make sure sizes and positions have a good safety margin.
    pub const MAX: i32 = i32::MAX / 2;
    pub const MIN: i32 = i32::MIN / 2;

    /// Clamps an i32 distance to MIN..MAX.
    #[inline(always)]
    pub const fn clamp(d: i32) -> i32 {
        if d < Self::MIN {
            Self::MIN
        } else if d > Self::MAX {
            Self::MAX
        } else {
            d
        }
    }

    /// Clamps an i32 distance to 0..MAX.
    #[inline]
    pub const fn clamp_non_negative(d: i32) -> i32 {
        if d < 0 {
            0
        } else if d > Self::MAX {
            Self::MAX
        } else {
            d
        }
    }
    /// Clamps an i32 distance to 1..MAX.
    #[inline]
    pub const fn clamp_positive(d: i32) -> i32 {
        if d < 1 {
            1
        } else if d > Self::MAX {
            Self::MAX
        } else {
            d
        }
    }

    /* from/to u32 (for notcurses, tiny-skia)*/

    /// Clamps a u32 distance to i32 0..MAX.
    #[inline]
    pub const fn clamp_from_u32(d: u32) -> i32 {
        if d > Self::MAX as u32 {
            Self::MAX
        } else {
            d as i32
        }
    }
    /// Clamps a u32 distance to i32 1..MAX.
    #[inline]
    pub const fn clamp_positive_from_u32(d: u32) -> i32 {
        if d == 0 {
            1
        } else if d > Self::MAX as u32 {
            Self::MAX
        } else {
            d as i32
        }
    }

    //

    /// Clamps an i32 distance to u32 0..MAX.
    #[inline]
    pub const fn clamp_to_u32(d: i32) -> u32 {
        Self::clamp_non_negative(d) as u32
    }
    /// Clamps an i32 distance to u32 1..MAX.
    #[inline]
    pub const fn clamp_positive_to_u32(d: i32) -> u32 {
        Self::clamp_positive(d) as u32
    }

    /* from/to i16 (for sdl) */

    /// Clamps an i16 distance to i32 MIN..MAX.
    #[inline(always)]
    pub const fn clamp_from_i16(d: i16) -> i32 {
        d as i32
    }
    /// Clamps an i16 distance to i32 0..MAX.
    #[inline(always)]
    pub const fn clamp_non_negative_from_i16(d: i16) -> i32 {
        if d < 0 {
            0
        } else {
            d as i32
        }
    }
    /// Clamps an i16 distance to i32 1..MAX.
    #[inline(always)]
    pub const fn clamp_positive_from_i16(d: i16) -> i32 {
        if d < 1 {
            1
        } else {
            d as i32
        }
    }

    //

    /// Clamps an i32 distance to i16 MIN..MAX.
    #[inline]
    pub const fn clamp_to_i16(d: i32) -> i16 {
        if d < i16::MIN as i32 {
            i16::MIN
        } else if d > i16::MAX as i32 {
            i16::MAX
        } else {
            d as i16
        }
    }
    /// Clamps an i32 distance to i16 0..MAX.
    #[inline]
    pub const fn clamp_non_negative_to_i16(d: i32) -> i16 {
        if d < 0 {
            0
        } else if d > i16::MAX as i32 {
            i16::MAX
        } else {
            d as i16
        }
    }
    /// Clamps an i32 distance to i16 1..MAX.
    #[inline]
    pub const fn clamp_positive_to_i16(d: i32) -> i16 {
        if d < 1 {
            1
        } else if d > i16::MAX as i32 {
            i16::MAX
        } else {
            d as i16
        }
    }

    /* from/to usize (machine word size & slice len)*/

    /// Clamps a usize distance to i32 0..MAX.
    #[inline]
    pub const fn clamp_from_usize(d: usize) -> i32 {
        if d > Self::MAX as usize {
            Self::MAX
        } else {
            d as i32
        }
    }
    /// Clamps a usize distance to i32 1..MAX.
    #[inline]
    pub const fn clamp_positive_from_usize(d: usize) -> i32 {
        if d == 0 {
            1
        } else if d > Self::MAX as usize {
            Self::MAX
        } else {
            d as i32
        }
    }

    //

    /// Clamps an i32 distance to usize 0..MAX.
    #[inline]
    pub const fn clamp_to_usize(d: i32) -> usize {
        Self::clamp_non_negative(d) as usize
    }
    /// Clamps an i32 distance to usize 1..MAX.
    #[inline]
    pub const fn clamp_positive_to_usize(d: i32) -> usize {
        Self::clamp_positive(d) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::Distance as D;

    #[test]
    fn clamping() {
        /* i32 */

        assert_eq![99, D::clamp(99)];
        assert_eq![-99, D::clamp(-99)];
        assert_eq![D::MAX, D::clamp(i32::MAX)];
        assert_eq![D::MIN, D::clamp(i32::MIN)];
        //
        assert_eq![D::MAX, D::clamp_non_negative(i32::MAX)];
        assert_eq![0, D::clamp_non_negative(i32::MIN)];
        //
        assert_eq![D::MAX, D::clamp_positive(i32::MAX)];
        assert_eq![1, D::clamp_positive(i32::MIN)];

        /* from/to u32 (for notcurses, tiny-skia) */

        assert_eq![D::MAX, D::clamp_from_u32(u32::MAX)];
        assert_eq![0, D::clamp_from_u32(u32::MIN)];
        assert_eq![D::MAX, D::clamp_positive_from_u32(u32::MAX)];
        assert_eq![1, D::clamp_positive_from_u32(u32::MIN)];
        //
        assert_eq![D::MAX as u32, D::clamp_to_u32(i32::MAX)];
        assert_eq![0_u32, D::clamp_to_u32(i32::MIN)];
        assert_eq![D::MAX as u32, D::clamp_positive_to_u32(i32::MAX)];
        assert_eq![1_u32, D::clamp_positive_to_u32(i32::MIN)];

        /* from/to i16 (for sdl) */

        assert_eq![i16::MAX as i32, D::clamp_from_i16(i16::MAX)];
        assert_eq![i16::MIN as i32, D::clamp_from_i16(i16::MIN)];
        assert_eq![i16::MAX as i32, D::clamp_non_negative_from_i16(i16::MAX)];
        assert_eq![0_i32, D::clamp_non_negative_from_i16(i16::MIN)];
        assert_eq![i16::MAX as i32, D::clamp_positive_from_i16(i16::MAX)];
        assert_eq![1_i32, D::clamp_positive_from_i16(i16::MIN)];
        //
        assert_eq![i16::MAX, D::clamp_to_i16(i32::MAX)];
        assert_eq![i16::MIN, D::clamp_to_i16(i32::MIN)];
        assert_eq![i16::MAX, D::clamp_non_negative_to_i16(i32::MAX)];
        assert_eq![0_i16, D::clamp_non_negative_to_i16(i32::MIN)];
        assert_eq![i16::MAX, D::clamp_positive_to_i16(i32::MAX)];
        assert_eq![1_i16, D::clamp_positive_to_i16(i32::MIN)];

        /* from/to usize (machine word & slice len) */
        assert_eq![D::MAX, D::clamp_from_usize(usize::MAX)];
        assert_eq![0, D::clamp_from_usize(usize::MIN)];
        assert_eq![D::MAX, D::clamp_positive_from_usize(usize::MAX)];
        assert_eq![1, D::clamp_positive_from_usize(usize::MIN)];
        //
        assert_eq![D::MAX as usize, D::clamp_to_usize(i32::MAX)];
        assert_eq![0_usize, D::clamp_to_usize(i32::MIN)];
        assert_eq![D::MAX as usize, D::clamp_positive_to_usize(i32::MAX)];
        assert_eq![1_usize, D::clamp_positive_to_usize(i32::MIN)];
    }
}
