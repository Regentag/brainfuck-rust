/**
	BrainFuck 실행 모듈

	이 모듈은 Thomas Cort <tom@brainfuck.ca>의
	BF2Java.java 구현을 참고하여 개발되었습니다.
	http://esoteric.sange.fi/brainfuck/impl/compilers/BF2Java.java
*/
use tok::BFToken;
use std::result;
use std::io;
use std::io::{Read, Result};

/// 프로그램이 올바른지 검사한다.
pub fn check_program( program: &Vec<BFToken> ) -> result::Result<(),&str>
{
	if program.len() == 0
	{
		return Err("No executable commands.");
	}

	let mut loop_check = 0;
	for command in program
	{
		match *command
		{
			BFToken::LBeg => { loop_check += 1; },
			BFToken::LEnd => { loop_check -= 1; },		
			_ => {}
		}
	}

	if loop_check == 0
	{
		Ok(())
	}
	else
	{
		Err("Loop not match.")
	}
}

/// 명령어를 실행한다.
pub fn exec( program: &Vec<BFToken> )
{
	let mut mem: [u8; 32768] = [0; 32768];
	let mut pointer: usize = 0;
	let mut pc: usize = 0;
	let mut stack = Vec::new();

	loop
	{
		if pc >= program.len()
		{
			// End of code.
			return;
		}
		
		let command = program[ pc ].clone();
		match command
		{
			BFToken::PNext =>
			{
				pointer += 1;
			},
			BFToken::PPrev =>
			{
				if pointer != 0 {
					pointer -= 1;
				} else {
					panic!("Mem Pointer Underflow!");
				}
			},
			BFToken::VAdd1 =>
			{
				mem[ pointer ] = mem[ pointer ] + 1;
			},
			BFToken::VSub1 =>
			{
				mem[ pointer ] = mem[ pointer ] - 1;
			},
			BFToken::VWrit =>
			{
				print!( "{}", (mem[ pointer ] as char) );
			},
			BFToken::VRead =>
			{
				let mut buf: [u8;1] = [0];
				match io::stdin().take(1).read(&mut buf) {
					Ok(n) if n == 1 => mem[pointer] = buf[0],
					_ => (),
				}
			},
			BFToken::LBeg =>
			{
				stack.push( pc );
				
				// 포인터가 가리키는 바이트의 값이 0이 되면 ]로 이동한다.
				// == 명령을 읽어서 버린다.
				if mem[ pointer ] == 0
				{
					loop
					{
						match program[ pc+1 ]
						{
							BFToken::LEnd => break,
							_             => pc += 1
						}
					}
				}
			},
			BFToken::LEnd =>
			{
				let saved_pc = stack.pop().unwrap();
				
				// 포인터가 가리키는 바이트의 값이 0이 아니면 [로 이동한다.
				if mem[ pointer ] != 0
				{
					pc = saved_pc -1;
				}
			}
		}
		
		pc += 1;	// 다음 명령
	}
}
