#ifndef UI_DASHBOARD_DASHBOARD_VIEW_HPP
#define UI_DASHBOARD_DASHBOARD_VIEW_HPP

#include "../view.hpp"
#include <cstdint>
#include <fmt/core.h>
#include <fmt/format.h>
#include <functional> // std::function

namespace UI
{

  struct DashboardViewModel final
  {
    std::int32_t n;
    float weight;

    bool operator==(const DashboardViewModel &other) const;
    bool operator!=(const DashboardViewModel &other) const;
  };

  enum class DashboardAction
  {
    Tare,
    Calibrate,
  };

  template <typename State>
  class DashboardView final : public BaseView<State, DashboardViewModel, DashboardAction>
  {
  public:
    using ViewModelFactory = DashboardViewModel (*)(const State &);
    using ActionDispatcher = std::function<void(DashboardAction)>;

    DashboardView(
        ViewModelFactory makeViewModel,
        ActionDispatcher dispatch)
        : BaseView<State, DashboardViewModel, DashboardAction>{makeViewModel, dispatch} {}

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

    void handleInputEvent(
        const DashboardViewModel &,
        const ButtonEvent &buttonEvent,
        ActionDispatcher dispatch) const override
    {
      switch (buttonEvent.buttonTag)
      {
      case UI::ButtonEvent::ButtonTag::A:
        if (buttonEvent.type == UI::ButtonEvent::Type::ButtonDown)
          dispatch(DashboardAction::Tare);
        break;
      case UI::ButtonEvent::ButtonTag::B:
        if (buttonEvent.type == UI::ButtonEvent::Type::ButtonDown)
          dispatch(DashboardAction::Calibrate);
        break;
      }
    }
  };

}

#endif
