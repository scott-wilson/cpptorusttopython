#include "cpptorusttopython/cpp/lib.h"
#include "cpptorusttopython/src/lib.rs.h"

namespace cpptorusttopython {
Something::Something(int8_t value) {
    this->value = value;
}

Something::~Something() {}

std::unique_ptr<Something> get_something() {
  std::unique_ptr<Something> something(new Something(3));

  return something;
}

int8_t get_something_value(const Something &something) {
    return something.value;
}

void set_something_value(Something &something, int8_t value) {
    something.value = value;
}
} // namespace cpptorusttopython
