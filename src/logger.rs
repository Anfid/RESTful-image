use clap::load_yaml;
use clap::App;
use stderrlog;

pub fn init() -> Configuration {
    let yaml = load_yaml!("args.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    let verbosity = match matches.occurrences_of("verbose") {
        v @ 0...3 => (v + 1) as usize,
        _ => {
            println!("Maximum level of verbosity is -vvv");
            4
        }
    };
    let quiet = matches.is_present("quiet");

    stderrlog::new()
        .color(stderrlog::ColorChoice::Auto)
        .timestamp(stderrlog::Timestamp::Second)
        .verbosity(verbosity)
        .quiet(quiet)
        .init()
        .expect("Initializing second instance of stderrlog");
}
