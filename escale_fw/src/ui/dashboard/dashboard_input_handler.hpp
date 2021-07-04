#ifndef __DASHBOARD_INPUT_HANDLER_HPP__
#define __DASHBOARD_INPUT_HANDLER_HPP__

#include <cstdint>
#include "../input.hpp"

class DashboardInputHandler final : public UI::InputHandler
{
public:
  DashboardInputHandler(std::int32_t &n) : n{n} {}

  void handleInputEvent(const UI::InputEvent &inputEvent) override;

private:
  std::int32_t &n;
};

#endif
