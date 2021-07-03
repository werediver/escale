#include "dashboard_input_handler.hpp"

void DashboardInputHandler::handleInputEvent(const AppInput::ButtonEvent &buttonEvent)
{
  switch (buttonEvent.buttonTag)
  {
  case AppInput::ButtonEvent::ButtonTagA:
    if (buttonEvent.type == AppInput::ButtonEvent::TypeButtonDown)
      n += 1;
    break;
  case AppInput::ButtonEvent::ButtonTagB:
    if (buttonEvent.type == AppInput::ButtonEvent::TypeButtonDown)
      n -= 1;
    break;
  }
}
