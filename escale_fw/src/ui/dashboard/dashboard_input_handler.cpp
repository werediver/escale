#include "dashboard_input_handler.hpp"

void DashboardInputHandler::onButtonDown(AppInput::ButtonTag buttonTag)
{
  switch (buttonTag)
  {
  case AppInput::ButtonA:
    n += 1;
    break;
  case AppInput::ButtonB:
    n -= 1;
    break;
  }
}
