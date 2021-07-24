#ifndef UI_DASHBOARD_DASHBOARD_TASK_HPP
#define UI_DASHBOARD_DASHBOARD_TASK_HPP

#include "../../run_loop/run_loop.hpp"
#include "../../unit.hpp"
#include "../taring/taring_task.hpp"
#include "../view_stack_task.hpp"
#include "dashboard_view.hpp"

#include <deque>

namespace UI
{

  using DashboardTaskState = Unit;

  template <typename State>
  class DashboardTask final : public RunLoop::BaseTask<State, DashboardTaskState>
  {
  private:
    enum class Action
    {
      Init,
      Tare,
      Calibrate
    };

  public:
    using TaringTaskFactory = std::shared_ptr<TaringTask<State>> (*)();
    using ViewModelFactory = typename DashboardView<State>::ViewModelFactory;

    DashboardTask(
        TaringTaskFactory makeTaringTask,
        ViewModelFactory makeViewModel)
        : RunLoop::BaseTask<State, DashboardTaskState>{
              [](auto)
              { return Unit(); }},
          makeTaringTask{makeTaringTask}, makeViewModel{makeViewModel} {}

  private:
    void run(RunLoop::RunLoop<State> &runLoop, DashboardTaskState state) override
    {
      if (!actions.empty())
      {
        const Action action = actions.front();
        actions.pop_front();

        switch (action)
        {
        case Action::Init:
        {
          std::weak_ptr weakSelf{runLoop.find(this)};
          auto pViewStack = runLoop.template find<ViewStackTask<State>>();
          if (pViewStack)
          {
            (*pViewStack)
                .push_back(std::make_shared<DashboardView<State>>(
                    makeViewModel,
                    [weakSelf](DashboardAction action)
                    {
                      if (auto self = weakSelf.lock())
                        self->handleDashboardViewAction(action);
                    }));
          }
          break;
        }
        case Action::Tare:
          runLoop.push_back(makeTaringTask());
          break;
        case Action::Calibrate:
          break;
        }
      }
    }

    void handleDashboardViewAction(DashboardAction action)
    {
      switch (action)
      {
      case DashboardAction::Tare:
        actions.push_back(Action::Tare);
        break;
      case DashboardAction::Calibrate:
        actions.push_back(Action::Calibrate);
        break;
      }
    }

    TaringTaskFactory makeTaringTask;
    ViewModelFactory makeViewModel;
    std::deque<Action> actions{Action::Init};
  };

}

#endif
