#include "calibration_view.hpp"

namespace UI
{

  bool CalibrationViewModel::operator==(const CalibrationViewModel &other) const
  {
    return mode == other.mode && zeroOffset == other.zeroOffset && rawReading == other.rawReading;
  }

  bool CalibrationViewModel::operator!=(const CalibrationViewModel &other) const
  {
    return !(*this == other);
  }

}
