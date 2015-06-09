use std::env;
use std::fs::File;
use std::io::Read;
use tok::*;
use exec::*;

mod tok;
mod exec;

fn main() {
	let args : Vec<_> = env::args().collect();
	let mut code_file: &str;
	let mut tok_table: &str = "brainfuck";

	if args.len() == 2
	{
		code_file = &args[1][..];
	}
	else if args.len() == 3
	{
		tok_table = &args[1][..];
		code_file = &args[2][..];
	}
	else
	{
		println!( "error: Invalid arguments." );
		println!( "usage bf [language] [code file]" );
		println!( "" );
		println!( "Languages:" );
		println!( "  brainfuck : Brainfuck (default)" );
		println!( "  ook       : Ook!" );
		println!( "  nyaruko   : Nyaruko" );

		return;
	}

	let tt = match tok_table
	{
		"brainfuck" => default_ttable(),
		"ook"       => ook_ttable(),
		"nyaruko"   => nyaruko_ttable(),
		_ =>
		{
			println!( "Invalid language!" );
			return;
		}
	};

	let code = load_code( &code_file );
	if code.is_ok()
	{
		let code_str = &code.unwrap()[..];

		let program = tok::parse( &code_str, &tt );
		let check = exec::check_program( &program );
		if check.is_ok()
		{
			exec::exec( &program );
		}
		else
		{
			println!("Error: {}", check.err().unwrap());
		}
	}
	else
	{
		println!("Error: {}", code.err().unwrap());
	}
}

fn load_code( file: &str ) -> Result<String,&str>
{
	let mut f = match File::open( file )
	{
		Ok(f) => f,
		Err(_) =>
		{
			return Err("Can't open code file.");
		}
	};

	let mut code: String = String::new();
	let read = f.read_to_string( &mut code );
	if read.is_ok()
	{
		Ok( code )
	}
	else
	{
		Err("Can't read code file.")
	}
}