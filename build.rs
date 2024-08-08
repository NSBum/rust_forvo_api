use vergen::{vergen, Config};

fn main() {
    // Generate the default 'cargo:' instruction output
    let config = Config::default();
    vergen(config).expect("Unable to generate version information");
}

