use std::env;
use std::process;

fn print_help()
{
    println!("Usage: ./raytracer <SCENE_FILE>\n\tSCENE_FILE: scene configuration");
}

pub fn parse_args() -> Option<String>
{
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        print_help();
        process::exit(1);
    }
    let argument: &str = &args[1];
    match argument {
        "-h" | "--help" => {
            print_help();
            None
        },
        _ => { Some(argument.to_string()) }
    }
}
