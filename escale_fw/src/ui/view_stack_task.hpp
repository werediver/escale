#ifndef UI_VIEW_STACK_TASK_HPP
#define UI_VIEW_STACK_TASK_HPP

#include <algorithm>

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
    using ViewPtr = std::shared_ptr<View<State>>;

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

    void push_back(ViewPtr view)
    {
      viewStack.push_back(view);
    }

    void remove(const View<State> *const target)
    {
      auto originalEnd = std::end(viewStack);
      auto newEnd = std::remove_if(
          std::begin(viewStack),
          originalEnd,
          [target](const ViewPtr &pView)
          { return pView.get() == target; });

      needsRender = needsRender || newEnd != originalEnd;

      viewStack.erase(newEnd, originalEnd);
    }

    template <typename T>
    void remove()
    {
      auto originalEnd = std::end(viewStack);
      auto newEnd = std::remove_if(
          std::begin(viewStack),
          originalEnd,
          [](const ViewPtr &pView)
          { return dynamic_cast<T *>(pView.get()) != nullptr; });

      needsRender = needsRender || newEnd != originalEnd;

      viewStack.erase(newEnd, originalEnd);
    }

    ViewPtr back()
    {
      return !viewStack.empty() ? viewStack.back() : nullptr;
    }

  private:
    std::vector<ViewPtr> viewStack;
    bool needsRender = true;
    AppHAL::Display &display;
  };

}

#endif
