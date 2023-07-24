#pragma once

// bindgen-flags: --with-derive-hash --with-derive-partialeq --with-derive-eq
class MyClass {
public:
  MyClass();
  // void myMethod() {}
  void myMethod();
};

class ZFCounter {
public:
  ZFCounter();
  void increment();

private:
  int count = 0;
};

// void myMethod();