//! Unit-aware numeric value parsing.
//!
//! In the physical sciences, we usually express numeric values as a pair:
//! (number, unit). Some examples:
//!
//! - 5 meters
//! - 1e6 m/s (meters per second)
//! - 42 kg * N (kilogram-newtons)
//!
//! In these examples, we have:
//! - Some sort of number -- be it written longhand, or in scientific notation
//!   - 5
//!   - 1e6
//!   - 42
//!
//! - A unit, some algebraic expression combining some particular tokens.
//!   - meters
//!   - m/s
//!   - kg * N
//!
//! Note that units can be written longhand -- as with "kilogram" -- or they can
//! be written in a variety of shorthands. "kilogram" can shorten to "kg", for
//! example. But Don't trust me to get this right, it's actually defined in a
//! [reasonably-central
//! place](http://ewh.ieee.org/soc/ias/pub-dept/abbreviation.pdf) -- though
//! that's also not the entire picture. We'll let that slide for now.
//!
//! So, altogether, this crate provides a library which will parse numeric
//! values with units specified!

// Keep our module system private, re-export the interface we want the world
// to see.

mod unit_parser;
