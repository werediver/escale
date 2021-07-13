#ifndef APP_STATE_HPP
#define APP_STATE_HPP

#include <cstdint>

enum class AppModeTag
{
  Normal,
  NAU7802NotFound,
  Halt
};

struct AppState
{
  AppModeTag mode;
  std::int32_t n;
  float w;
};

#endif
