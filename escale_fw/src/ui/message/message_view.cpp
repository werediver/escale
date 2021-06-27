#include <fmt/core.h>
#include <fmt/format.h>
#include "../../app_hal/display/display.hpp"

#include "message_view.hpp"

void displayMessageView(const MessageViewModel &viewModel, AppHAL::Display &display)
{
  const std::string msg = [&]()
  {
    switch (viewModel.message)
    {
    case MessageViewModel::MessageTaring:
      return "Taring...";
    case MessageViewModel::MessageCalibrating:
      return "Calibrating 100 g weight...";
    case MessageViewModel::MessageNAU7802NotFound:
      return "E: NAU7802 not found";
    }
    throw std::invalid_argument(
        fmt::format(FMT_STRING("Error code {} is invalid"), viewModel.message));
  }();

  display.clearBuffer();
  display.drawStr({0, 10}, msg);
  display.sendBuffer();
}
