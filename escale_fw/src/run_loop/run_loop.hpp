#ifndef RUN_LOOP_RUN_LOOP_HPP
#define RUN_LOOP_RUN_LOOP_HPP

#include <algorithm>
#include <memory>
#include <vector>

#include "task.hpp"

namespace RunLoop
{

  template <typename State>
  class RunLoop final
  {
  public:
    using TaskPtr = std::shared_ptr<Task<State>>;

    void run(State &state)
    {
      for (auto &pTask : tasks)
      {
        (*pTask).run(*this, state);
      }
    }

    void push_back(TaskPtr task)
    {
      tasks.push_back(task);
    }

    /// NOTE: If a task removes itself, the task object may get destructed while running.
    void remove(const Task<State> *const target)
    {
      tasks.erase(
          std::remove_if(
              std::begin(tasks),
              std::end(tasks),
              [target](const TaskPtr &pTask)
              { return pTask.get() == target; }),
          std::end(tasks));
    }

    template <typename T>
    std::shared_ptr<T> find() const
    {
      for (auto &pTask : tasks)
      {
        if (auto result = std::dynamic_pointer_cast<T>(pTask))
        {
          return result;
        }
      }
      return nullptr;
    }

    template <typename T>
    std::shared_ptr<T> find(const T *const target) const
    {
      for (auto &pTask : tasks)
      {
        if (pTask.get() == target)
        {
          return std::dynamic_pointer_cast<T>(pTask);
        }
      }
      return nullptr;
    }

  private:
    std::vector<std::shared_ptr<Task<State>>> tasks;
  };

}

#endif
