#include "bar.h"
#include "foo.h"
#include <stdio.h>

int main(void) {
  printf("Main");
  foo();
  bar();
  return 0;
}
