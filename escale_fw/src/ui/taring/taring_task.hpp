#ifndef UI_TARING_TARING_TASK_HPP
#define UI_TARING_TARING_TASK_HPP

#include "../../run_loop/task.hpp"
#include "../view_stack_task.hpp"
#include "taring_view.hpp"

#include <deque>

namespace UI
{

  template <typename State>
  class TaringTask final : public RunLoop::Task<State>
  {
  private:
    enum class Action
    {
      Init,
      Terminate,
      Tare
    };

  public:
    using Tare = void (*)(std::uint8_t sampleCount);

    TaringTask(Tare tare) : tare{tare} {}

    void run(RunLoop::RunLoop<State> &runLoop, State &state) override
    {
      if (!actions.empty())
      {
        const Action action = actions.front();
        actions.pop_front();

        switch (action)
        {
        case Action::Init:
        {
          auto pViewStack = runLoop.template find<ViewStackTask<State>>();
          if (pViewStack)
            (*pViewStack).push_back(std::make_shared<TaringView<State>>());
          break;
        }
        case Action::Terminate:
        {
          auto pViewStack = runLoop.template find<ViewStackTask<State>>();
          if (pViewStack)
            (*pViewStack).template remove<TaringView<State>>();
          runLoop.remove(this);
          break;
        }
        case Action::Tare:
          tare(64);
          actions.push_back(Action::Terminate);
          break;
        }
      }
    }

  private:
    std::deque<Action> actions{Action::Init, Action::Tare};
    Tare tare;
  };

}

#endif
