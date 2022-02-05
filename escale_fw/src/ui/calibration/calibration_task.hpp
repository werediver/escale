#ifndef UI_CALIBRATION_CALIBRATION_TASK_HPP
#define UI_CALIBRATION_CALIBRATION_TASK_HPP

#include "../../run_loop/run_loop.hpp"
#include "../view_stack_task.hpp"
#include "calibration_view.hpp"
#include "../../unit.hpp"

#include <deque>

namespace UI
{

  using CalibrationTaskState = Unit;

  template <typename State>
  class CalibrationTask final : public RunLoop::BaseTask<State, CalibrationTaskState>
  {
  private:
    enum class Action
    {
      Init,
      Terminate,
      Calibrate
    };

  public:
    using TaskStateFactory = CalibrationTaskState (*)(State &);
    using Calibrate = void (*)();
    using ViewModelFactory = typename CalibrationView<State>::ViewModelFactory;

    CalibrationTask(
        Calibrate calibrate,
        ViewModelFactory makeViewModel)
        : RunLoop::BaseTask<State, CalibrationTaskState>{
              [](auto)
              { return Unit{}; }},
          calibrate{calibrate}, makeViewModel{makeViewModel} {}

  private:
    void run(RunLoop::RunLoop<State> &runLoop, CalibrationTaskState state) override
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
                .push_back(std::make_shared<CalibrationView<State>>(
                    makeViewModel,
                    [weakSelf](CalibrationViewAction action)
                    {
                      if (auto self = weakSelf.lock())
                        self->handleViewAction(action);
                    }));
          }
          break;
        }
        case Action::Terminate:
        {
          auto pViewStack = runLoop.template find<ViewStackTask<State>>();
          if (pViewStack)
            (*pViewStack).template remove<CalibrationView<State>>();
          runLoop.remove(this);
          break;
        }
        case Action::Calibrate:
          calibrate();
          actions.push_back(Action::Terminate);
          break;
        }
      }
    }

    void handleViewAction(CalibrationViewAction action)
    {
      switch (action)
      {
      case CalibrationViewAction::Calibrate:
        actions.push_back(Action::Calibrate);
        break;
      case CalibrationViewAction::Cancel:
        actions.push_back(Action::Terminate);
        break;
      }
    }

    Calibrate calibrate;
    ViewModelFactory makeViewModel;
    std::deque<Action> actions{Action::Init};
  };
}

#endif
