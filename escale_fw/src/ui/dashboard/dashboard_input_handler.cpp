#include "dashboard_input_handler.hpp"

void DashboardInputHandler::handleInputEvent(const UI::ButtonEvent &buttonEvent)
{
  switch (buttonEvent.buttonTag)
  {
  case UI::ButtonEvent::ButtonTagA:
    if (buttonEvent.type == UI::ButtonEvent::TypeButtonDown)
      n += 1;
    break;
  case UI::ButtonEvent::ButtonTagB:
    if (buttonEvent.type == UI::ButtonEvent::TypeButtonDown)
      n -= 1;
    break;
  }
}
