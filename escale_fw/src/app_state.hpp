#ifndef __APP_STATE_HPP__
#define __APP_STATE_HPP__

#include <cstdint>
#include "state.hpp"

enum AppModeTag
{
  AppModeTagNormal,
  AppModeTagNau7802NotFound,
  AppModeTagHalt
};

using AppMode = State<AppModeTag>;

class AppModeNormal final : public AppMode
{
  AppModeNormal() : AppMode{AppModeTagNormal} {}
};

struct AppState
{
  AppModeTag mode;
  std::int32_t n;
  float w;
};

#endif
