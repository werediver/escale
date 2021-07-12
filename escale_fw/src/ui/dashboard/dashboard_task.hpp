#ifndef UI_DASHBOARD_DASHBOARD_TASK_HPP
#define UI_DASHBOARD_DASHBOARD_TASK_HPP

#include "../../run_loop/run_loop.hpp"
#include "../view_stack_task.hpp"
#include "dashboard_view.hpp"

namespace UI
{

  template <typename State>
  class DashboardTask final : public RunLoop::Task<State>
  {
  private:
    enum Action
    {
      ActionInit,
      ActionInc,
      ActionDec
    };

  public:
    using ViewModelFactory = typename DashboardView<State>::ViewModelFactory;

    DashboardTask(ViewModelFactory makeViewModel)
        : makeViewModel{makeViewModel},
          actions{ActionInit} {}

    void run(RunLoop::RunLoop<State> &runLoop, State &state) override
    {
      if (!actions.empty())
      {
        const Action action = actions.back();
        actions.pop_back();

        switch (action)
        {
        case ActionInit:
        {
          std::weak_ptr weakSelf{runLoop.find(this)};
          auto viewStack = runLoop.template find<ViewStackTask<State>>();
          if (viewStack)
          {
            (*viewStack)
                .push_back(std::make_shared<DashboardView<State>>(
                    makeViewModel,
                    [weakSelf](DashboardAction action)
                    {
                      if (auto self = weakSelf.lock())
                        self->handleAction(action);
                    }));
          }
          break;
        }
        case ActionInc:
          state.n += 1;
          break;
        case ActionDec:
          state.n -= 1;
          break;
        }
      }
    }

    void handleAction(DashboardAction action)
    {
      switch (action)
      {
      case DashboardActionIncrementN:
        actions.push_back(ActionInc);
        break;
      case DashboardActionDecrementN:
        actions.push_back(ActionDec);
        break;
      }
    }

  private:
    ViewModelFactory makeViewModel;
    std::vector<Action> actions;
  };

}

#endif
