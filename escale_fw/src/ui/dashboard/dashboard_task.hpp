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
    enum class Action
    {
      Init,
      Inc,
      Dec
    };

  public:
    using ViewModelFactory = typename DashboardView<State>::ViewModelFactory;

    DashboardTask(ViewModelFactory makeViewModel)
        : makeViewModel{makeViewModel} {}

    void run(RunLoop::RunLoop<State> &runLoop, State &state) override
    {
      if (!actions.empty())
      {
        const Action action = actions.back();
        actions.pop_back();

        switch (action)
        {
        case Action::Init:
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
        case Action::Inc:
          state.n += 1; // FIXME: Don't assume the type of `state`
          break;
        case Action::Dec:
          state.n -= 1; // FIXME: Don't assume the type of `state`
          break;
        }
      }
    }

    void handleAction(DashboardAction action)
    {
      switch (action)
      {
      case DashboardAction::IncrementN:
        actions.push_back(Action::Inc);
        break;
      case DashboardAction::DecrementN:
        actions.push_back(Action::Dec);
        break;
      }
    }

  private:
    ViewModelFactory makeViewModel;
    std::vector<Action> actions{Action::Init};
  };

}

#endif
