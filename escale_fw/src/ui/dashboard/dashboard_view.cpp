#include <fmt/core.h>
#include <fmt/format.h>
#include "../../app_hal/display/display.hpp"

#include "dashboard_view.hpp"

bool DashboardViewModel::operator==(const DashboardViewModel &other) const
{
  return n == other.n && weight == other.weight;
}

bool DashboardViewModel::operator!=(const DashboardViewModel &other) const
{
  return !(*this == other);
}

void renderDashboardView(const DashboardViewModel &viewModel, AppHAL::Display &display)
{
  auto s1 = fmt::format(FMT_STRING("w={:8.3f}"), viewModel.weight);
  auto s2 = fmt::format(FMT_STRING("{:3d}"), viewModel.n);

  display.clearBuffer();
  display.drawStr({0, 10}, s1);
  display.drawStr({100, 10}, s2);
  display.sendBuffer();
}
