#include "../../app_hal/display/display.hpp"

#include "dashboard_view.hpp"

namespace UI
{

  bool DashboardViewModel::operator==(const DashboardViewModel &other) const
  {
    return n == other.n && weight == other.weight;
  }

  bool DashboardViewModel::operator!=(const DashboardViewModel &other) const
  {
    return !(*this == other);
  }

}