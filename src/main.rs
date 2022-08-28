use std::{
    env,
    process::exit,
    io::{self, Write}
};
use regex::Regex;

fn main() {
	// Set defaults (hardcoded)
	let buy_markup = 0.10;
	let sell_markup = 0.20;

	let min_price = 1;
	let max_price = 100;

	// check if arguments are given
	let args: Vec<String> = env::args().collect();

	/*if args.len() == 2 && args[1].starts_with("-") {
		match args[1] {
			"-h" => help(min_price, max_price, buy_markup, sell_markup),
			"-i" => todo!(),
			_ => (),
		}
	}*/

	match args.len() {
		1 => interactive_mode(min_price, max_price, buy_markup, sell_markup), // Enter interactive mode
		2 => price_calc(args[1].parse::<i32>().unwrap(), buy_markup, sell_markup), // Only use card-value
		3 => price_calc_multiple(args[1].parse::<i32>().unwrap(), args[2].parse::<i32>().unwrap(), buy_markup, sell_markup), // Use min and max
		4 => price_calc(args[1].parse::<i32>().unwrap(), args[2].parse::<f64>().unwrap(), args[3].parse::<f64>().unwrap()), // Use card and markup (no min/max)
		5 => price_calc_multiple(args[1].parse::<i32>().unwrap(), args[2].parse::<i32>().unwrap(), args[3].parse::<f64>().unwrap(), args[4].parse::<f64>().unwrap()), // Use all Values
		_ => help(min_price, max_price, buy_markup, sell_markup),
	}
}

fn interactive_mode(min_price: i32, max_price: i32, buy_markup: f64, sell_markup: f64) {
	let interactive_command_help = "h";
	let interactive_command_exit = "e";
    let range_separator = "-";
    let re_range = Regex::new(r"\d+-\d+").unwrap();

	println!("Interactive Mode, enter a digit, a range, or an interacive command.
Interactive commands are:
{}: help
{}: exit", interactive_command_help, interactive_command_exit);

    loop {
	    // Process user input
	    print!("[]> ");
        // Flush IO output to ensure the line is printed before we read.
        io::stdout().flush().unwrap();

	    let mut interactive_command = String::new();

	    io::stdin()
	    	.read_line(&mut interactive_command)
	    	.expect("[ERR]: Failed to read input.");

        // Deal with OS-specific behavior where we sometimes get a newline
        if interactive_command.ends_with("\n") {
            interactive_command = interactive_command.strip_suffix("\n").unwrap().to_string();
        }

        // Parse user commands
        match interactive_command.as_str() {
            "h" => help(min_price, max_price, buy_markup, sell_markup),
            "e" => exit(0),
            _ => {
                if re_range.is_match(&interactive_command) {
                    // Parse range
                    println!("[DBG]: Found range: {interactive_command}");

                    let (range_start, range_end) = interactive_command
                        .trim()
                        .split_once(range_separator)
                        .unwrap();

                    let range_start: i32 = range_start.parse().unwrap();
                    let range_end: i32 = range_end.parse().unwrap();

                    price_calc_multiple(range_start, range_end, buy_markup, sell_markup);
                } else {

                    let command_as_number: i32 = match interactive_command.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("[ERR]: Input is not a known command or number");
                            continue;    
                        }
                    };

                    price_calc(command_as_number, buy_markup, sell_markup);

                }
            }
        }
    }
}

fn price_calc(value: i32, buy_markup: f64, sell_markup: f64) {
	println!("| {:^5} | {:^5} | {:^5} |", "Card", "Buy", "Sell");
	println!("|-------|-------|-------|");

	let val_float = value as f64;
	let buy_price: f64 = val_float * buy_markup + val_float;
	let sell_price: f64 = val_float * sell_markup + val_float;

    // Convert Floats back to integers
    let buy_price_int: i32 = buy_price.round() as i32;
    let sell_price_int: i32 = sell_price.round() as i32;

	println!("| {:^5} | {:^5} | {:^5} |", value, buy_price_int, sell_price_int);

}

fn price_calc_multiple(min_value: i32, max_value: i32, buy_markup: f64, sell_markup: f64) {
	println!("| {:^5} | {:^5} | {:^5} |", "Card", "Buy", "Sell");
	println!("|-------|-------|-------|");

	for i in min_value .. max_value + 1 {
		let i_float = i as f64;
		let buy_price: f64 = i_float * buy_markup + i_float;
		let sell_price: f64 = i_float * sell_markup + i_float;

        // Convert Floats back to integers
        let buy_price_int: i32 = buy_price.round() as i32;
        let sell_price_int: i32 = sell_price.round() as i32;

		println!("| {:^5} | {:^5} | {:^5} |", i, buy_price_int, sell_price_int);
	}
}

fn help(min_value: i32, max_value: i32, buy_markup: f64, sell_markup: f64) {
	println!("Waifu buy and sell price calculator.
This tool quickly generates the buy and sell price of a waifu/husbando given the base kakera value

Usage:
	waifucalc card-value
	waifucalc min-value max-value
	waifucalc min-value max-value buy-markup sell-markup

Interactive Mode Usage:
    card-value: calculate buy and sell prices
    min-value-max-value: calculate prices with range

    Examples:
        []> 17
        | Card  |  Buy  | Sell  |
        |-------|-------|-------|
        |  17   |  19   |  20   |

        []> 15-17
        | Card  |  Buy  | Sell  |
        |-------|-------|-------|
        |  15   |  17   |  18   |
        |  16   |  18   |  19   |
        |  17   |  19   |  20   |

Interactive Mode commands:
    `e` to exit
    `h` for this text

Defaults:
	min-value: {}
	max-value: {}
	buy-markup: {}
	sell-markup: {}", min_value, max_value, buy_markup, sell_markup);
}
