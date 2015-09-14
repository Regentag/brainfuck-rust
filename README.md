Brainfuck-Rust

브레인퍽 언어 인터프리터의 러스트 구현체.

브레인퍽(Brainfuck) 언어는 1993년 Urban Müller가 개발한 Esolang입니다.
자세한 설명은 [링크](https://esolangs.org/wiki/Brainfuck)를 참고하시기
바랍니다.

Brainfuck-Rust는 현재 Hello, World!와 Echo를 실행 가능합니다.
(다른 Brainfuck 코드들은 아직 테스트 되지 않았습니다.)

Update 2015-06-10
  * [foriequal0](https://github.com/foriequal0)님께서 Stdin으로부터 사용자의
	입력을 받는 기능을 구현해 주셨습니다.
  
Update 2015-09-14
  * Ook!, Nyaruko 구현 삭제.

TODO:
 * Parser 개선하기(...)
 * Stdin 대신 미리 주어진 입력값으로 실행할 수 있도록 하는 기능 추가(Web Interpreter 처럼).