#include "stdio.h"
// common multiple
// Considering
// 1. number % n == 0
// 2. number % m == 0
int solution(int number, int n, int m) {
  return number % n == 0 && number % m == 0 ? 1 : 0;
  // if (number % n != 0 && number % m != 0)
  //   return 0;
  // Having more evaluation than the above
  // if (number % n + number % m == 0)
  //   return 0;
  // return 1;
}

// execution
int main(void) {
  printf("result : %d\n", solution(10, 5, 2));
  return 0;
}
