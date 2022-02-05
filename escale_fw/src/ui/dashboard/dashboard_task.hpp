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
    using CalibrationTaskFactory = std::shared_ptr<CalibrationTask<State>> (*)();
    using ViewModelFactory = typename DashboardView<State>::ViewModelFactory;

    DashboardTask(
        TaringTaskFactory makeTaringTask,
        CalibrationTaskFactory makeCalibrationTask,
        ViewModelFactory makeViewModel)
        : RunLoop::BaseTask<State, DashboardTaskState>{
              [](auto)
              { return Unit(); }},
          makeTaringTask{makeTaringTask}, makeCalibrationTask{makeCalibrationTask}, makeViewModel{makeViewModel} {}

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
                    [weakSelf](DashboardViewAction action)
                    {
                      if (auto self = weakSelf.lock())
                        self->handleViewAction(action);
                    }));
          }
          break;
        }
        case Action::Tare:
          runLoop.push_back(makeTaringTask());
          break;
        case Action::Calibrate:
          runLoop.push_back(makeCalibrationTask());
          break;
        }
      }
    }

    void handleViewAction(DashboardViewAction action)
    {
      switch (action)
      {
      case DashboardViewAction::Tare:
        actions.push_back(Action::Tare);
        break;
      case DashboardViewAction::Calibrate:
        actions.push_back(Action::Calibrate);
        break;
      }
    }

    TaringTaskFactory makeTaringTask;
    CalibrationTaskFactory makeCalibrationTask;
    ViewModelFactory makeViewModel;
    std::deque<Action> actions{Action::Init};
  };

}

#endif
