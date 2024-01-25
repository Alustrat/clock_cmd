use std::{thread, time};
use std::io::{Write, stdout};
use crossterm::{cursor, terminal, ExecutableCommand};
use chrono::{Utc, Local};
use clap::Parser;
use chrono_tz::{Tz, TZ_VARIANTS};


const NUMBER_MATRIX: [[[bool; 3]; 5]; 11] = [
    [ // 0
        [true, true, true],
        [true, false, true],
        [true, false, true],
        [true, false, true],
        [true, true, true],
    ],[ // 1
        [true, true, false],
        [false, true, false],
        [false, true, false],
        [false, true, false],
        [true, true, true],
    ],[ // 2
        [true, true, true],
        [false, false, true],
        [true, true, true],
        [true, false, false],
        [true, true, true],
    ],[ // 3
        [true, true, true],
        [false, false, true],
        [true, true, true],
        [false, false, true],
        [true, true, true],
    ],[ // 4
        [true, false, true],
        [true, false, true],
        [true, true, true],
        [false, false, true],
        [false, false, true],
    ],[ // 5
        [true, true, true],
        [true, false, false],
        [true, true, true],
        [false, false, true],
        [true, true, true],
    ],[ // 6
        [true, true, true],
        [true, false, false],
        [true, true, true],
        [true, false, true],
        [true, true, true],
    ],[ // 7
        [true, true, true],
        [false, false, true],
        [false, false, true],
        [false, false, true],
        [false, false, true],
    ],[ // 8
        [true, true, true],
        [true, false, true],
        [true, true, true],
        [true, false, true],
        [true, true, true],
    ],[ // 9
        [true, true, true],
        [true, false, true],
        [true, true, true],
        [false, false, true],
        [false, false, true],
    ],[ // :
        [false, false, false],
        [false, true, false],
        [false, false, false],
        [false, true, false],
        [false, false, false],
    ]
];

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short='s', long)]
    size: Option<u32>,

    #[arg(short='f', long=None, action=clap::ArgAction::SetTrue)]
    keep_open: Option<bool>,

    #[arg(long="timezone", short='t')]
    timezone: Option<String>,

    #[arg(short=None, long, action=clap::ArgAction::SetTrue)]
    utc: Option<bool>,
}

fn parse_timezone(tz: String) -> Tz {
    let lowered_tz = tz.to_lowercase();

    match TZ_VARIANTS.iter().find(|timezone| timezone.name().to_lowercase().contains(&lowered_tz)) {
        Some(v) => *v,
        None => {
            eprintln!("Unknown timezone");
            std::process::exit(1);
        }
    }
}

enum OffsetTypes {
    Utc,
    Local,
    Timezone(Tz),
}


fn main() {
    let mut stdout = stdout();
    let args = Args::parse();

    let size_factor = args.size.unwrap_or(1);
    let keep_open = args.keep_open.unwrap_or(false);

    let offset: OffsetTypes;
    if args.utc.unwrap_or(false) {
        offset = OffsetTypes::Utc;
    } else if args.timezone.is_some() {
        offset = OffsetTypes::Timezone(parse_timezone(args.timezone.unwrap()));
    } else {
        offset = OffsetTypes::Local;
    }


    if keep_open {
        keep_refreshing(&mut stdout, size_factor, &offset);
    } else {
        print_clock(&mut stdout, size_factor, &offset);
    }
}

fn keep_refreshing(mut stdout: &std::io::Stdout, size_factor: u32, offset: &OffsetTypes) {
    stdout.execute(cursor::Hide).unwrap();

    // When the user leave, make the cursor back
    ctrlc::set_handler(move || {
        let mut stdout = std::io::stdout();
        stdout.execute(cursor::Show).unwrap();
        std::process::exit(0);
    }).expect("Error setting Ctrl-C handler");

    loop {
        let lines = print_clock(stdout, size_factor, offset);
        thread::sleep(time::Duration::from_millis(1000));

        stdout.execute(cursor::MoveUp(lines)).unwrap();
        stdout.execute(terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap();
    }
}

fn print_clock(mut stdout: &std::io::Stdout, size_factor: u32, offset: &OffsetTypes) -> u16 {    
    let mut lines: Vec<String> = vec![];
    let time = get_time(offset);
    let time_matrix: Vec<_> = time.iter().map(|v| NUMBER_MATRIX[*v as usize]).collect();

    for i in 0..5 {
        for _ in 0..size_factor {
            let mut raw_str: String = String::new();

            for number_matrix in &time_matrix {
                for value in &number_matrix[i as usize] {
                    if *value {
                        raw_str += &"█".repeat(size_factor as usize);
                    } else {
                        raw_str += &" ".repeat(size_factor as usize);
                    }
                }
                raw_str += &" ".repeat(size_factor as usize);
            }

            lines.push(raw_str);
        }
    }

    match offset {
        OffsetTypes::Timezone(v) => {
            lines.push("".to_string());
            lines.push(format!("Timezone={}", v.name().to_string()))
        },
        _ => ()
    };

    for line in &lines {
        writeln!(stdout, "{}", line).unwrap();
    }

    lines.len() as u16
}

fn get_time(offset: &OffsetTypes) -> Vec<u32> {
    let time_str = match offset {
        OffsetTypes::Utc => {
            let time = Utc::now();
            time.format("%H:%M:%S").to_string()
        },
        OffsetTypes::Timezone(v) => {
            let time: chrono::prelude::DateTime<Tz> = Utc::now().with_timezone(&v);
            time.format("%H:%M:%S").to_string()
        },
        OffsetTypes::Local => {
            let time = Local::now();
            time.format("%H:%M:%S").to_string()
        },
    };

    // Return the time as a list of digit
     time_str.chars().map(|char| {
        if char.is_ascii_digit() {
            return char.to_digit(10).unwrap();
        }

        if char == ':' {
            return 10;
        }

        0
    }).collect()
}