#ifndef __VIEWS_HPP__
#define __VIEWS_HPP__

#include <cstdint>

struct DashboardViewModel final
{
  std::int32_t n;
  float w;
};

struct ErrorViewModel final
{
  enum Error
  {
    ErrorNAU7802NotFound
  };

  Error error;
};

class Display;

void displayDashboardView(const DashboardViewModel &viewModel, Display &display);

void displayErrorView(const ErrorViewModel &viewModel, Display &display);

#endif
