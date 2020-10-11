use clap::{Arg, App, ArgMatches};
use bauxite::{Alignment, BoxBuilder};

fn extract_arg_value<'a>(matches: &'a ArgMatches, value_name: &'a str) -> Result<&'a str, String> {
    if !matches.is_present(value_name) {
        return Err(format!("Expected value for {}", value_name))
    }

    Ok(matches.value_of(value_name).unwrap())
}

fn parse_usize(matches: &ArgMatches, value_name: &str) -> Result<usize, String> {
    Ok(match extract_arg_value(matches, value_name)?.parse::<usize>() {
        Ok(value) => value,
        Err(_) => return Err(format!("Unable to parse value of {} as integer value", value_name)),
    })
}

fn main() -> Result<(), String> {
    let matches = App::new("Bauxite")
        .version("1.0")
        .author("Dock O'Neal")
        .about("Wrap text in boxes")
        .arg(Arg::with_name("padding")
             .long("padding")
             .value_name("padding")
             .takes_value(true)
             .help("Sets overal box padding."))
        .arg(Arg::with_name("padding-top")
             .long("padding-top")
             .value_name("padding-top")
             .takes_value(true)
             .help("Set padding on the top of the box."))
        .arg(Arg::with_name("padding-right")
             .long("padding-right")
             .value_name("padding-right")
             .takes_value(true)
             .help("Set padding on right side of the box."))
        .arg(Arg::with_name("padding-left")
             .long("padding-left")
             .value_name("padding-left")
             .takes_value(true)
             .help("Set padding on the left side of the box."))
        .arg(Arg::with_name("padding-bottom")
             .long("padding-bottom")
             .value_name("padding-bottom")
             .takes_value(true)
             .help("Set padding on the bottom of the box."))
        .arg(Arg::with_name("alignment-left")
             .long("align-left")
             .value_name("align-left")
             .takes_value(false)
             .help("Sets box alignment to the left"))
        .arg(Arg::with_name("alignment-right")
             .long("align-right")
             .value_name("align-right")
             .takes_value(false)
             .help("Sets box alignment to the right"))
        .arg(Arg::with_name("max-wdith")
             .long("max-width")
             .value_name("max-width")
             .takes_value(true)
             .help("Sets the maximum width of the messagae inside the box before the message should wrap"))
        .arg(Arg::with_name("message")
             .help("Message to be wrapped in a box")
             .index(1)
             .required(true))
        .get_matches();

    let message = matches.value_of("message").unwrap();

    let mut box_builder = BoxBuilder::new(String::from(message));

    if matches.is_present("padding") {
        box_builder = box_builder.padding(parse_usize(&matches, "padding")?);
    }

    if matches.is_present("padding-top") {
        box_builder = box_builder.padding_top(parse_usize(&matches, "padding-top")?);
    }

    if matches.is_present("padding-right") {
        box_builder = box_builder.padding_right(parse_usize(&matches, "padding-right")?);
    }

    if matches.is_present("padding-left") {
        box_builder = box_builder.padding_left(parse_usize(&matches, "padding-left")?);
    }

    if matches.is_present("padding-bottom") {
        box_builder = box_builder.padding_bottom(parse_usize(&matches, "padding-bottom")?);
    }

    if matches.is_present("alignment-left") {
        box_builder = box_builder.alignment(Alignment::Left);
    }

    print!("{}", box_builder);
    Ok(())
}
