use clap::{command, crate_authors, crate_description, Arg, ArgAction};

fn main() {
    let matches = command!()
        .author(crate_authors!())
        .about(crate_description!())
        .help_template(
            "{name} {version}\n{author}\n{about}\nUSAGE:\n{usage}\n\nOPTIONS:\n{options}",
        )
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print new line")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let text = matches
        .get_many::<String>("text")
        .unwrap()
        .map(|val| val.as_str())
        .collect::<Vec<_>>()
        .join(" ");
    let omit_newline = matches.get_flag("omit_newline");
    let ending = if omit_newline { "" } else { "\n" };
    print!("{text}{ending}");
}
