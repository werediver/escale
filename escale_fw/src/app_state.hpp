#ifndef APP_STATE_HPP
#define APP_STATE_HPP

#include <cstdint>

enum AppModeTag
{
  AppModeTagNormal,
  AppModeTagNau7802NotFound,
  AppModeTagHalt
};

struct AppState
{
  AppModeTag mode;
  std::int32_t n;
  float w;
};

#endif
