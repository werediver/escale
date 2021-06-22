#ifndef __STATE_HPP__
#define __STATE_HPP__

#include <cstdint>

enum Mode
{
  ModeNormal,
  ModeNau7802NotFound,
  ModeHalt
};

struct State
{
  Mode mode;
  std::int32_t n;
  float w;
};

#endif
