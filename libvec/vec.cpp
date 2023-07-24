#include "vec.h"
#include <stdio.h>

MyClass::MyClass() { printf("Constructor\n"); }

void MyClass::myMethod() { printf(" My method\n"); }

ZFCounter::ZFCounter() { printf("ZFCounter contructor\n"); };

void ZFCounter::increment() {
  count++;
  printf("ZFCounter count : %d\n", count);
}
