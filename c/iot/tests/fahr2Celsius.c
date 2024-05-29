#include <stdio.h> // C언어 만든애가 만든 C header .h 파일
// int <- integer (정수)
// 함수(function) -> 1 input, 1 output
// void : 존재하지 않는다
// 영어가 중요한 이유, 부동소수점 => floating point
// Linux(C) => main 처음에 찾아라
// a.out => 실행 => ./a.out
// int <- integer(정수)

int main(void) {
  int fahr; // 변수
  fahr = 100;

  double celsious; // double floating point
  celsious = 5.0 / 9.0 * (fahr - 32);

  // % : 형식 지정자 <- format specifier
  // print format
  // printf란 함수가 없는데, 어디서 가져와야 할까? 아니면 만들까?
  // \ : escape sequence
  // %d : decimal
  // %f : float
  printf("celsious : %f\n", celsious);

  // 사용자한테 키보드입력을 받아야 할때
  // scanf("%d", &fahr);
  // fprintf(stdout, "");

  return 0; // UNIX 0 => 정상종료 됐다
}

// 1. 여기서 만들진 않을 것
// 2. 가장 처음에 함

// 문자열 name1(void) { return "문자열"; }

// Ken Tompson : PDP-7
// UniCS => UNiplexed Information Computing System
// single-user-single-pg
// 운영체제 = UniCS = 여러개의 프로그램을, 하나의 컴퓨터에서 실행
// multi-user-multi-program
//
//
// C언어가 만들어진 이유
// 운영체제(UNIX)를 만들려고, assembly 언어 (CPU 제조사) , 기계어
// 10010100 => copy
// copy "wefj" memory1 => Cword
// 10010100 => mov (copy)
// 10010100 => mov (copy)
