Brainfuck-Rust

브레인퍽 언어 인터프리터의 러스트 구현체.

브레인퍽(Brainfuck) 언어는 1993년 Urban Müller가 개발한 Esolang입니다.
자세한 설명은 [링크](https://esolangs.org/wiki/Brainfuck)를 참고하시기
바랍니다.
  
Brainfuck-Rust는 현재 세가지 언어의 실행을 지원합니다:
 * Brainfuck

 * Ook!

   Ook!은 Brainfuck의 방언으로 오랑우탄이 읽고 쓸 수 있으며, 기억하기 쉬운 단순한
   명령어들로, 예를들면 "원숭이"처럼 긴 단어를 쓸 필요가 없도록 제작된 언어입니다.
   바나나는 맛있다고 합니다.

   http://www.dangermouse.net/esoteric/ook.html

 * Nyaruko

   Nyaruko는 일본에서 제작된 Brainfuck의 방언입니다.
	 いつも ニコニコ あなたの隣に 這いよる混沌 ニャルラトホテプ言語 ですっ☆

	 https://github.com/masarakki/nyaruko_lang

  
Brainfuck-Rust는 현재 Hello, World!와 Echo를 실행 가능합니다.
(다른 Brainfuck 코드들은 아직 테스트 되지 않았습니다.)

Update 2015-06-10
  * [foriequal0](https://github.com/foriequal0)님께서 Stdin으로부터 사용자의
	입력을 받는 기능을 구현해 주셨습니다.
