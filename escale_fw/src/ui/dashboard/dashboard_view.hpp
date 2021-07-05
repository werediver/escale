#ifndef __DASHBOARD_VIEW_HPP__
#define __DASHBOARD_VIEW_HPP__

#include <cstdint>
#include <fmt/core.h>
#include <fmt/format.h>
#include "../view.hpp"

namespace UI
{

  struct DashboardViewModel final
  {
    std::int32_t n;
    float weight;

    bool operator==(const DashboardViewModel &other) const;
    bool operator!=(const DashboardViewModel &other) const;
  };

  enum DashboardAction
  {
    DashboardActionIncrementN,
    DashboardActionDecrementN,
  };

  template <typename State>
  class DashboardView final : public SomeView<State, DashboardViewModel, DashboardAction>
  {
  public:
    using ViewModelFactory = DashboardViewModel (*)(const State &);
    using ActionDispatcher = void (*)(DashboardAction);

    DashboardView(
        ViewModelFactory makeViewModel,
        ActionDispatcher dispatch)
        : SomeView<State, DashboardViewModel, DashboardAction>{makeViewModel, dispatch} {}

  private:
    void render(const DashboardViewModel &viewModel, AppHAL::Display &display) const override
    {
      auto s1 = fmt::format(FMT_STRING("w={:8.3f}"), viewModel.weight);
      auto s2 = fmt::format(FMT_STRING("{:3d}"), viewModel.n);

      display.clearBuffer();
      display.drawStr({0, 10}, s1);
      display.drawStr({100, 10}, s2);
      display.sendBuffer();
    }

    void handleInputEvent(const DashboardViewModel &, const ButtonEvent &buttonEvent, ActionDispatcher dispatch) const override
    {
      switch (buttonEvent.buttonTag)
      {
      case UI::ButtonEvent::ButtonTagA:
        if (buttonEvent.type == UI::ButtonEvent::TypeButtonDown)
          dispatch(DashboardActionIncrementN);
        break;
      case UI::ButtonEvent::ButtonTagB:
        if (buttonEvent.type == UI::ButtonEvent::TypeButtonDown)
          dispatch(DashboardActionDecrementN);
        break;
      }
    }
  };

}

#endif
