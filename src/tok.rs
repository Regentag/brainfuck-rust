use std::collections::HashMap;
use std::collections::HashSet;

/// BrainFuck Tokens
#[derive(Clone)]
pub enum BFToken
{
	PNext,
	PPrev,
	VAdd1,
	VSub1,
	VWrit,
	VRead,
	LBeg,
	LEnd
}

/// BrainFuck's default token table.
/// http://esolangs.org/wiki/Brainfuck
pub fn default_ttable() -> HashMap<&'static str,BFToken>
{
	let mut ttable = HashMap::new();
	ttable.insert( ">", BFToken::PNext );
	ttable.insert( "<", BFToken::PPrev );
	ttable.insert( "+", BFToken::VAdd1 );
	ttable.insert( "-", BFToken::VSub1 );
	ttable.insert( ".", BFToken::VWrit );
	ttable.insert( ",", BFToken::VRead );
	ttable.insert( "[", BFToken::LBeg );
	ttable.insert( "]", BFToken::LEnd );

	ttable
}

/// 토큰에 사용되는 문자들의 집합을 생성한다.
fn token_chars( ttable: &HashMap<&str,BFToken> ) -> HashSet<char>
{
	let mut chars = HashSet::new();

	for tokstr in ttable.keys()
	{
		for c in tokstr.chars()
		{
			chars.insert( c );
		}
	}

	chars
}

/// 주어진 BrainFuck 코드를 파싱하여
/// 실행을 위한 토큰 목록으로 만든다.
pub fn parse( code: &str, ttable: &HashMap<&str,BFToken> ) -> Vec<BFToken>
{
	let tok_chars = token_chars( &ttable );
	let mut tmp_tok_str : String = String::new();
	let mut tok_vec : Vec<BFToken> = Vec::new();

	for c in code.chars()
	{
		// 토큰에 사용되는 문자열이라면?
		// 아니라면 무시하고 넘어간다.
		if tok_chars.contains( &c )
		{
			tmp_tok_str.push( c );

			if ttable.contains_key( &tmp_tok_str[..] )
			{
				// 명령어를 찾으면
				let tok = ttable.get(
					&tmp_tok_str[..]
				).unwrap(); 
				tok_vec.push( tok.clone() );
				tmp_tok_str = String::new();
			}
		}
	}

	tok_vec
}