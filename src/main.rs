use rasmus_og_monopolet;
use rasmus_og_monopolet::Config;

fn main() {
    env_logger::init();

    let config = envy::from_env::<Config>().expect("Couldn't get config");

    rasmus_og_monopolet::run(config);
}
