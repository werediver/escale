#ifndef UI_VIEW_STACK_TASK_HPP
#define UI_VIEW_STACK_TASK_HPP

#include "../run_loop/task.hpp"
#include "view.hpp"

namespace AppHAL
{
  struct Display;
}

namespace UI
{

  template <typename State>
  class ViewStackTask final : public RunLoop::Task<State>
  {
  public:
    explicit ViewStackTask(AppHAL::Display &display) : display{display} {}

    void run(RunLoop::RunLoop<State> &, State &state) override
    {
      if (!viewStack.empty())
      {
        auto &view = *viewStack.back();
        view.build(state);
        if (needsRender || view.needsRender())
        {
          view.render(display);
          needsRender = false;
        }
      }
    }

    void push_back(std::shared_ptr<View<State>> view)
    {
      viewStack.push_back(view);
    }

    std::shared_ptr<View<State>> back()
    {
      return viewStack.back();
    }

  private:
    std::vector<std::shared_ptr<View<State>>> viewStack;
    bool needsRender = true;
    AppHAL::Display &display;
  };

}

#endif
