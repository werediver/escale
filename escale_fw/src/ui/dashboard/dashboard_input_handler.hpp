#ifndef __DASHBOARD_INPUT_HANDLER_HPP__
#define __DASHBOARD_INPUT_HANDLER_HPP__

#include <cstdint>
#include "../../app_input.hpp"

class DashboardInputHandler final : public AppInput::InputHandler
{
public:
  DashboardInputHandler(std::int32_t &n) : n{n} {}

  void onButtonDown(AppInput::ButtonTag buttonTag) override;

private:
  std::int32_t &n;
};

#endif
