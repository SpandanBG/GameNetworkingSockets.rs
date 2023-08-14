use bindgen;
use std::{env, path::PathBuf};

const GNS_SUBMODULE_DIR: &'static str = "GameNetworkingSockets";
const BINDINGS_FILE: &'static str = "bindings.rs";

const STEAM_GNS_HEADERS: [&'static str; 11] = [
    "isteamnetworkingmessages.h",
    "isteamnetworkingsockets.h",
    "isteamnetworkingutils.h",
    "steam_api_common.h",
    "steamclientpublic.h",
    "steamnetworkingcustomsignaling.h",
    "steamnetworkingsockets_flat.h",
    "steamnetworkingsockets.h",
    "steamnetworkingtypes.h",
    "steamtypes.h",
    "steamuniverse.h",
];

fn main() {
    let header_base_path = match env::current_dir() {
        Ok(path) => path,
        Err(error) => panic!(
            "Error while trying to fetch current directory location {}",
            error.to_string()
        ),
    };

    // Create the path to the header includes of the `GameNetworkingSockets` submodule.
    let header_base_path = header_base_path
        .join(GNS_SUBMODULE_DIR)
        .join("include")
        .join("steam");

    let bindings: bindgen::Builder =
        STEAM_GNS_HEADERS
            .into_iter()
            .fold(bindgen::Builder::default(), |bindings, header_file| {
                let header_path = header_base_path.join(header_file);
                let header_path = match header_path.to_str() {
                    Some(path) => path,
                    None => panic!("Error while finding header file = {}", header_file),
                };
                println!("cargo:rerun-if-changed={}", header_path);
                bindings.header(header_path)
            });

    let bindings = match bindings
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
    {
        Ok(bindings) => bindings,
        Err(error) => panic!(
            "Error while finalizing bindgen build = {}",
            error.to_string()
        ),
    };

    // Set the out path for `bindings` to the `OUT_DIR` env of Cargo
    let out_dir = PathBuf::from(match env::var("OUT_DIR") {
        Ok(out_dir) => out_dir,
        Err(error) => panic!(
            "Error while fetching `OUT_DIR` cargo env = {}",
            error.to_string()
        ),
    });

    bindings
        .write_to_file(out_dir.join(BINDINGS_FILE))
        .expect("Error whilte trying to write bindings");
}
