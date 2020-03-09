//! # Art
//!
//! A library for modeling artistic concepts.

pub use self::kinds::{PrimaryColor, SecondaryColor};
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    #[derive(Debug)]
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    #[derive(Debug)]
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create a secondary color.
    ///
    /// # Examples
    ///
    /// ```
    /// let c1 = my_art::kinds::PrimaryColor::Red;
    /// let c2 = my_art::kinds::PrimaryColor::Blue;
    ///
    /// match my_art::utils::mix(c1, c2) {
    ///     Ok(color) => {
    ///         match color {
    ///             my_art::kinds::SecondaryColor::Purple => {}
    ///             _ => panic!(format!(
    ///                 "Incorrect color! Expected {:?}. Got {:?}",
    ///                 my_art::kinds::SecondaryColor::Purple, color
    ///             ))
    ///         }
    ///     }
    ///     Err(e) => panic!(format!("Unexpected error: {:?}", e))
    /// };
    /// ```
    pub fn mix(
        c1: PrimaryColor, c2: PrimaryColor,
    ) -> Result<SecondaryColor, &'static str> {
        let result = match c1 {
            PrimaryColor::Red => {
                match c2 {
                    PrimaryColor::Yellow => SecondaryColor::Orange,
                    PrimaryColor::Blue => SecondaryColor::Purple,
                    _ => return Err("Invalid primary color combination.")
                }
            }
            PrimaryColor::Yellow => {
                match c2 {
                    PrimaryColor::Red => SecondaryColor::Orange,
                    PrimaryColor::Blue => SecondaryColor::Green,
                    _ => return Err("Invalid primary color combination.")
                }
            }
            PrimaryColor::Blue => {
                match c2 {
                    PrimaryColor::Red => SecondaryColor::Purple,
                    PrimaryColor::Yellow => SecondaryColor::Green,
                    _ => return Err("Invalid primary color combination.")
                }
            }
        };

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::kinds::{PrimaryColor, SecondaryColor};

    #[test]
    fn purple() {
        let c1 = PrimaryColor::Red;
        let c2 = PrimaryColor::Blue;

        match utils::mix(c1, c2) {
            Ok(color) => {
                match color {
                    SecondaryColor::Purple => {}
                    _ => panic!(format!(
                        "Incorrect color! Expected {:?}. Got {:?}",
                        SecondaryColor::Purple, color
                    ))
                }
            }
            Err(e) => panic!(format!("Unexpected error: {:?}", e))
        };
    }

    #[test]
    fn incorrect_color_combination() {
        let c1 = PrimaryColor::Yellow;
        let c2 = PrimaryColor::Yellow;

        assert!(utils::mix(c1, c2).is_err());
    }
}
