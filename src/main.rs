//
//  main.rs
//  ghtool
//
//  Created by Søren Mortensen on 28/02/2018.
//  Copyright © 2018 Søren Mortensen.
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

pub extern crate ghtool_label as label;
pub extern crate ghtool_util as util;

extern crate clap;

fn main() {
    let _ = details::app().get_matches();
}

/// Details about this app.
mod details {
    use clap::{App, Arg};
    use label;

    /// This command's app definition.
    pub fn app() -> App<'static, 'static> {
        App::new( name())
            .version(version())
            .author(author())
            .about(description())
            .args(&args()[..])
            .subcommand(
                label::details::app()
            )
    }

    /// This app's name.
    fn name() -> &'static str { env!("CARGO_PKG_NAME") }

    /// This app's version.
    fn version() -> &'static str { env!("CARGO_PKG_VERSION") }

    /// This app's author(s).
    fn author() -> &'static str { env!("CARGO_PKG_AUTHORS") }

    /// This app's description.
    fn description() -> &'static str { env!("CARGO_PKG_DESCRIPTION") }

    /// This app's arguments.
    fn args() -> Vec<Arg<'static, 'static>> {
        vec![]
    }
}
