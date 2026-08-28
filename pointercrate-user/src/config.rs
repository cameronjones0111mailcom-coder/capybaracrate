
use std::{fs::File, io::Read};

use pointercrate_core::util::from_env_or_default;

pub(crate) fn secret() -> Vec<u8> {
    // Use the SECRET environment variable when running on a platform
    // such as Render where a secret file may not exist.
    if let Ok(secret) = std::env::var("SECRET") {
        return secret.into_bytes();
    }

    // Fall back to the SECRET_FILE environment variable or .secret file.
    let path: String = from_env_or_default("SECRET_FILE", ".secret".into());

    match File::open(path) {
        Ok(file) => file.bytes().collect::<Result<Vec<u8>, _>>().unwrap(),
        Err(err) if cfg!(debug_assertions) => {
            // Needed for integration tests/CI.
            log::error!(
                "Failed to read secret, using an unsecure default since this is a debug build - {:?}",
                err
            );

            vec![0x0; 64]
        }
        Err(err) => panic!("Unable to open secret file: {:?}", err),
    }
}

pub fn google_client_id() -> String {
    std::env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID is not set")
}
