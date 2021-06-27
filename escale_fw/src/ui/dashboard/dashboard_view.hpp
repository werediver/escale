#ifndef __DASHBOARD_VIEW_HPP__
#define __DASHBOARD_VIEW_HPP__

#include <cstdint>

struct DashboardViewModel final
{
  std::int32_t n;
  float weight;
};

namespace AppHAL
{
  struct Display;
}

void displayDashboardView(const DashboardViewModel &viewModel, AppHAL::Display &display);

#endif
