//! Traits and common data types used for manifests.
//!
//! Manifests live in their various crates (peripherals, games, etc).

#[macro_use]
extern crate derive_builder;

use std::collections::HashMap;
// use std::fmt::Display;
use std::path::PathBuf;

use common::data::key::ValidKey;
use common::data::serialization::{Streamable as StreamableTrait, Tomlable as TomlableTrait};

// use serde::{Deserialize, Serialize};

pub mod examples;
pub mod game;
mod meta;
pub mod peripheral;
pub mod range;
mod specs;
mod vendor;

// TODO: [implementation] Is this the best place for these
/// The game defined name of a player's io
/// such as "power_ups" to describe 5 card slots
/// or "inventory" to describe a players set of rolled dice of swords
pub type GameDefinedGroup = ValidKey;

/// the peripheral-defined name for a group of specs it implements
pub type PeripheralDefinedGroup = ValidKey;

/// The map that connects the various pieces together
/// The is saying that this part of the peripheral is owned by a specific player
/// and is used by the game in this specific way
pub type AssignmentsMap =
    HashMap<PlayerIndex, HashMap<GameDefinedGroup, (u8, PeripheralDefinedGroup)>>;

/// A player type such as "GameMaster" or "Campaigner"
/// @defined by the game manifest
pub type PlayerType = String;

/// The number of the player in a type. For instance the 3rd GameMaster or Campaigner 0
pub type PlayerIndex = u8;

// ^^ TODO: [implementation] Is this the best place for these

/// Trait for representing a manifest with serialization and streaming capabilities.
///
/// This trait provides functionalities for validating, locking, and saving manifest data.
/// It integrates `TomlableTrait` and `StreamableTrait` to support serialization to TOML format
/// and streaming capabilities, respectively.
pub trait Manifest: TomlableTrait + StreamableTrait {
    /// Validates the manifest.
    ///
    /// This method checks if the manifest meets certain criteria and returns a `Result`.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the manifest passes all validation checks.
    /// * `Err(Vec<String>)` if there are validation errors, with a vector containing
    ///   descriptions of each error.
    fn validate(&self) -> Result<(), Vec<String>>;

    /// Creates lock files associated with the manifest.
    ///
    /// This method is intended to generate a set of lock files. Each lock file is a TOML file
    /// that contains all the pieces of the various lock files, expanded and processed.
    /// This is what is streamed and used.
    ///
    /// # Returns
    /// A `HashMap<String, String>` where each key is a filename and each value is the TOML
    /// string content of the lock file.
    fn lock(&self) -> HashMap<String, String> {
        todo!("TODO: [manifests] Implement locking of manifests");
    }

    /// Saves this manifest to a specified file.
    ///
    /// This method is responsible for persisting the manifest data to a file at the given path.
    ///
    /// # Arguments
    /// * `path` - A `PathBuf` representing the file path where the manifest should be saved.
    ///
    /// # Returns
    /// * `Ok(())` if the save operation is successful.
    /// * `Err(())` if the save operation fails.
    fn save(&self, _path: PathBuf) -> Result<(), ()> {
        todo!("TODO: [manifests] Implement saving of manifests");
    }
}
