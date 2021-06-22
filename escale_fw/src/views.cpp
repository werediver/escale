#include <fmt/core.h>
#include <fmt/format.h>
#include "display.hpp"

#include "views.hpp"

void displayDashboardView(const DashboardViewModel &viewModel, Display &display)
{
  auto s1 = fmt::format(FMT_STRING("n={}"), viewModel.n);
  auto s2 = fmt::format(FMT_STRING("w={:8.3f}"), viewModel.w);

  display.clearBuffer();
  display.drawStr({0, 10}, s1);
  display.drawStr({0, 20}, s2);
  display.sendBuffer();
}

void displayErrorView(const ErrorViewModel &viewModel, Display &display)
{
  const std::string msg = [&]()
  {
    switch (viewModel.error)
    {
    case ErrorViewModel::ErrorNAU7802NotFound:
      return "E: NAU7802 not found";
    }
    throw std::invalid_argument(
        fmt::format(FMT_STRING("Error code {} is invalid"), viewModel.error));
  }();

  display.clearBuffer();
  display.drawStr({0, 10}, msg);
  display.sendBuffer();
}
