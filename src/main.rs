use std::{thread, time};
use std::io::{Write, stdout};
use crossterm::{cursor, terminal, ExecutableCommand};
use chrono::Utc;
use clap::Parser;


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
}

fn main() {
    let mut stdout = stdout();
    let args = Args::parse();

    let size_factor = args.size.unwrap_or(1);
    let keep_open = args.keep_open.unwrap_or(false);

    if keep_open {
        keep_refreshing(&mut stdout, size_factor)
    } else {
        print_clock(&mut stdout, size_factor)
    }
}

fn keep_refreshing(mut stdout: &std::io::Stdout, size_factor: u32) {
    stdout.execute(cursor::Hide).unwrap();

    // When the user leave, make the cursor back
    ctrlc::set_handler(move || {
        let mut stdout = std::io::stdout();
        stdout.execute(cursor::Show).unwrap();
        std::process::exit(0);
    }).expect("Error setting Ctrl-C handler");

    loop {
        print_clock(stdout, size_factor);
        thread::sleep(time::Duration::from_millis(1000));

        stdout.execute(cursor::MoveUp(5* size_factor as u16)).unwrap();
        stdout.execute(terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap();
    }
}

fn print_clock(mut stdout: &std::io::Stdout, size_factor: u32) {    
    let time = get_time();
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

            writeln!(stdout, "{}", raw_str).unwrap();
        }
    }
}

fn get_time() -> Vec<u32> {
    let now_str = Utc::now().format("%H:%M:%S").to_string();

    // Return the time as a list of digit
     now_str.chars().map(|char| {
        if char.is_ascii_digit() {
            return char.to_digit(10).unwrap();
        } else if char == ':' {
            return 10;
        }

        0
    }).collect()
}