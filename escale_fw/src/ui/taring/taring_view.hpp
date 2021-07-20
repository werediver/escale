#ifndef UI_TARING_TARING_VIEW_HPP
#define UI_TARING_TARING_VIEW_HPP

#include "../../app_hal/display/display.hpp"
#include "../../unit.hpp"
#include "../view.hpp"

namespace UI
{

  using TaringViewModel = Unit;
  using TaringAction = Unit;

  template <typename State>
  class TaringView final : public BaseView<State, TaringViewModel, TaringAction>
  {
  public:
    using ActionDispatcher = std::function<void(TaringAction)>;

    TaringView()
        : BaseView<State, TaringViewModel, TaringAction>(
              [](auto)
              { return Unit(); },
              [](auto) {}) {}

  private:
    void render(const TaringViewModel &, AppHAL::Display &display) const override
    {
      display.clearBuffer();
      display.drawStr({0, 10}, "Taring...");
      display.sendBuffer();
    }

    void handleInputEvent(const TaringViewModel &, const InputEvent &, ActionDispatcher) const override {}
  };

}

#endif
