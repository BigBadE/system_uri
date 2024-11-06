// Copyright 2018 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under the MIT license <LICENSE-MIT
// http://opensource.org/licenses/MIT> or the Modified BSD license <LICENSE-BSD
// https://opensource.org/licenses/BSD-3-Clause>, at your option. This file may not be copied,
// modified, or distributed except according to those terms. Please review the Licences for the
// specific language governing permissions and limitations relating to use of the SAFE Network
// Software.

use crate::app::App;

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::path::Path;
use anyhow::Error;
use winreg::enums::HKEY_CURRENT_USER;
use winreg::RegKey;

fn to_wide_chars(s: &str) -> Vec<u16> {
    OsStr::new(s)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect::<Vec<_>>()
}

// as described at https://msdn.microsoft.com/en-us/library/aa767914(v=vs.85).aspx
/// Register the given App for the given schemes.
///
/// `app` should contain all fields necessary for registering URIs on all systems. `schemes` should
/// provide a list of schemes (the initial part of a URI, like `https`).
pub fn install(app: &App, schemes: &[String]) -> Result<(), Error> {
    // but we can't write on root, we'll have to do it for the current user only
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    for protocol in schemes {
        let base_path = Path::new("Software").join("Classes").join(protocol);
        let (key, _) = hkcu.create_subkey(&base_path)?;
        // set our app name as the for reference
        key.set_value("", &app.name)?;
        key.set_value("URL Protocol", &"")?;

        let (command_key, _) =
            hkcu.create_subkey(&base_path.join("shell").join("open").join("command"))?;
        command_key.set_value("", &format!("{} \"%1\"", app.exec))?
    }
    Ok(())
}