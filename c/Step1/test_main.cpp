#include <cstdio>

int add(int a, int b) { return a + b; }

int main() {
  auto r = add(1, 2);
  char s1[10];

  printf("Put your name: ");
  scanf("%s", s1);
  printf("Hello, %s!\n", s1);

  return 0;
}
