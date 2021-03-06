//
//  util/repo.rs
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

use regex::Regex;

use std::fmt;

/// The path to a GitHub repository, in the form "user/repository".
#[derive(Clone)]
pub struct Repo {
    /// The username of the repository owner.
    pub user: String,
    /// The name of the repository.
    pub repo: String,
}

impl Repo {
    /// Attemps to parse a `String` to create a `Repo`. The string should be in the format "user/repository".
    pub fn from_string(string: &str) -> Option<Repo> {
        // A regular expression for matching "user/repo"-style repository paths.
        let repo_path = Regex::new(r"^([A-Za-z0-9\-_]+)/([A-Za-z0-9\-_]+)").unwrap();

        repo_path.captures(&string[..]).and_then(|captures| {
            match (captures.get(1), captures.get(2)) {
                (Some(user), Some(repo)) => Some(Repo {
                    user: user.as_str().to_owned(),
                    repo: repo.as_str().to_owned(),
                }),
                _ => None,
            }
        })
    }
}

impl fmt::Display for Repo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.user, self.repo)
    }
}
