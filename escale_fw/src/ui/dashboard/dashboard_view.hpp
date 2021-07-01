#ifndef __DASHBOARD_VIEW_HPP__
#define __DASHBOARD_VIEW_HPP__

#include <cstdint>

struct DashboardViewModel final
{
  std::int32_t n;
  float weight;

  bool operator==(const DashboardViewModel &other) const;
  bool operator!=(const DashboardViewModel &other) const;
};

namespace AppHAL
{
  struct Display;
}

void renderDashboardView(const DashboardViewModel &viewModel, AppHAL::Display &display);

#endif
