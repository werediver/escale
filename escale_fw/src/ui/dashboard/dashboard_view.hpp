#ifndef UI_DASHBOARD_DASHBOARD_VIEW_HPP
#define UI_DASHBOARD_DASHBOARD_VIEW_HPP

#include "../view.hpp"
#include <fmt/core.h>
#include <fmt/format.h>
#include <functional> // std::function

namespace UI
{

  struct DashboardViewModel final
  {
    float weight;

    bool operator==(const DashboardViewModel &other) const;
    bool operator!=(const DashboardViewModel &other) const;
  };

  enum class DashboardViewAction
  {
    Tare,
    Calibrate,
  };

  template <typename State>
  class DashboardView final : public BaseView<State, DashboardViewModel, DashboardViewAction>
  {
  public:
    using ViewModelFactory = DashboardViewModel (*)(const State &);
    using ActionDispatcher = std::function<void(DashboardViewAction)>;

    DashboardView(
        ViewModelFactory makeViewModel,
        ActionDispatcher dispatch)
        : BaseView<State, DashboardViewModel, DashboardViewAction>{makeViewModel, dispatch} {}

  private:
    void render(const DashboardViewModel &viewModel, AppHAL::Display &display) const override
    {
      auto s1 = fmt::format(FMT_STRING("w={:6.1f}"), viewModel.weight);

      display.clearBuffer();
      display.drawStr({0, 10}, s1);
      display.sendBuffer();
    }

    void handleInputEvent(
        const DashboardViewModel &,
        const ButtonEvent &buttonEvent,
        ActionDispatcher dispatch) const override
    {
      switch (buttonEvent.buttonTag)
      {
      case UI::ButtonEvent::ButtonTag::A:
        if (buttonEvent.type == UI::ButtonEvent::Type::ButtonDown)
          dispatch(DashboardViewAction::Tare);
        break;
      case UI::ButtonEvent::ButtonTag::B:
        if (buttonEvent.type == UI::ButtonEvent::Type::ButtonDown)
          dispatch(DashboardViewAction::Calibrate);
        break;
      }
    }
  };

}

#endif
