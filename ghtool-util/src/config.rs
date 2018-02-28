//
//  config.rs
//  ghtool-util
//
//  Created by Søren Mortensen on 28/02/2018.
//  Copyright © 2018 Søren Mortensen. All rights reserved.
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//

//! The `config` module implements functionality for reading configuration information from a file in the user's home
//! directory.

use toml;
use std::env;
use std::fs::File;
use std::path::PathBuf;
use std::error::Error;
use std::fmt;
use std::io;
use std::io::Read;

/// The user's configuration, including their [`Config`](struct.Config.html) and values loaded from flags at runtime.
#[derive(Debug)]
pub struct RuntimeConfig {
    /// Stored configuration such as the access token.
    pub config: Config,
    /// The value of the -c flag (whether the destination repository should have its labels cleared before copying).
    pub clear: bool,
    /// The value of the -y flag (whether to assume yes to all prompts and run non-interactively).
    pub assume_yes: bool,
}

impl RuntimeConfig {
    pub fn access_token(&self) -> &String {
        &self.config.access_token
    }
}

/// The user's configuration, loaded from disk where it is stored in TOML format.
#[derive(Debug, Deserialize)]
pub struct Config {
    pub access_token: String,
}

impl Config {
    pub fn try_load() -> Result<Config, ConfigError> {
        config_contents().and_then(|contents| {
            toml::from_str::<Config>(&contents[..]).map_err(|err| ConfigError::ParseError(err))
        })
    }

    pub fn file_exists() -> bool {
        match config_path() {
            Some(path) => path.exists(),
            None => false,
        }
    }
}

/// Get the path to the user's ghlabelcpy config file, or `None` if it isn't possible to determine their home directory.
fn config_path() -> Option<PathBuf> {
    env::home_dir().map(|mut path| {
        path.push(".config/ghtool/config.toml");
        path
    })
}

/// Get a reference to the user's ghlabelcpy config file, if possible.
fn config_file() -> Result<File, ConfigError> {
    config_path()
        .ok_or(ConfigError::FileMissing)
        .and_then(|path| File::open(path).map_err(|err| ConfigError::IoError(err)))
}

/// Get the contents of the user's ghlabelcpy config file as a `String`, if possible.
fn config_contents() -> Result<String, ConfigError> {
    config_file().and_then(|mut file| {
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|err| ConfigError::IoError(err))?;
        Ok(contents)
    })
}

/// Errors that arise in the process of reading the user's config file.
#[derive(Debug)]
pub enum ConfigError {
    FileMissing,
    IoError(io::Error),
    ParseError(toml::de::Error),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfigError::FileMissing => write!(f, "FileMissing"),
            ConfigError::IoError(ref err) => write!(f, "IoError: {}", err),
            ConfigError::ParseError(ref err) => write!(f, "ParseError: {}", err),
        }
    }
}

impl Error for ConfigError {
    fn description(&self) -> &str {
        match *self {
            ConfigError::FileMissing => "The configuration file does not exist.",
            ConfigError::IoError(_) => "An IO error occurred.",
            ConfigError::ParseError(_) => "Unable to parse configuration file.",
        }
    }
}

