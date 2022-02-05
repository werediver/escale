#ifndef UI_CALIBRATION_CALIBRATION_UI_HPP
#define UI_CALIBRATION_CALIBRATION_UI_HPP

#include "../view.hpp"
#include "../../app_hal/display/display.hpp"
#include <fmt/core.h>
#include <fmt/format.h>
#include <functional> // std::function

namespace UI
{

  enum class CalibrationViewMode
  {
    Instruct,
    Calibrate
  };

  struct CalibrationViewModel final
  {
    std::int32_t zeroOffset;
    std::int32_t rawReading;
    CalibrationViewMode mode = CalibrationViewMode::Instruct;

    bool operator==(const CalibrationViewModel &other) const;
    bool operator!=(const CalibrationViewModel &other) const;
  };

  enum class CalibrationViewAction
  {
    Calibrate,
    Cancel,
  };

  template <typename State>
  class CalibrationView final : public BaseView<State, CalibrationViewModel, CalibrationViewAction>
  {
  public:
    using ViewModelFactory = CalibrationViewModel (*)(const State &);
    using ActionDispatcher = std::function<void(CalibrationViewAction)>;

    CalibrationView(
        ViewModelFactory makeViewModel,
        ActionDispatcher dispatch)
        : BaseView<State, CalibrationViewModel, CalibrationViewAction>{makeViewModel, dispatch} {}

  private:
    void render(const CalibrationViewModel &viewModel, AppHAL::Display &display) const override
    {
      display.clearBuffer();
      switch (viewModel.mode)
      {
      case CalibrationViewMode::Instruct:
        display.drawStr({0, 10}, "Calibration");
        display.drawStr({0, 20}, "Put 100 g on scale");
        display.drawStr({0, 30}, "and press (>)");
        break;
      case CalibrationViewMode::Calibrate:
        display.drawStr({0, 10}, "Calibrating...");
        break;
      }

      display.drawStr(
          {0, 50},
          fmt::format(
              FMT_STRING("Z: {:d} R: {:6d}"),
              viewModel.zeroOffset,
              viewModel.rawReading));

      display.sendBuffer();
    }

    void handleInputEvent(
        const CalibrationViewModel &,
        const ButtonEvent &buttonEvent,
        ActionDispatcher dispatch) const override
    {
      switch (buttonEvent.buttonTag)
      {
      case UI::ButtonEvent::ButtonTag::A:
        if (buttonEvent.type == UI::ButtonEvent::Type::ButtonDown)
          dispatch(CalibrationViewAction::Cancel);
        break;
      case UI::ButtonEvent::ButtonTag::B:
        if (buttonEvent.type == UI::ButtonEvent::Type::ButtonDown)
          dispatch(CalibrationViewAction::Calibrate);
        break;
      }
    }
  };

}

#endif
