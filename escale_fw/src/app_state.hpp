#ifndef APP_STATE_HPP
#define APP_STATE_HPP

#include <cstdint>

struct AppState
{
  std::int32_t readCount;
  /// Raw weight reading before applying a zero-offset and a calibration factor,
  /// but possibly filtered.
  std::int32_t rawWeigh;
  std::int32_t zeroOffset;
  float calibrationFactor;
  float weight;
};

#endif
