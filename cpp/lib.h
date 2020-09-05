#pragma once

#include "rust/cxx.h"
#include <memory>

namespace cpptorusttopython {
class Something {
private:
public:
  int8_t value;
  Something(int8_t);
  ~Something();
};

std::unique_ptr<Something> get_something();

int8_t get_something_value(const Something &something);
void set_something_value(Something &something, int8_t value);
} // namespace cpptorusttopython
