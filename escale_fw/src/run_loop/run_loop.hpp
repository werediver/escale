#ifndef RUN_LOOP_RUN_LOOP_HPP
#define RUN_LOOP_RUN_LOOP_HPP

#include <memory>
#include <vector>

#include "task.hpp"

namespace RunLoop
{

  template <typename State>
  class RunLoop final
  {
  public:
    void run(State &state)
    {
      for (auto &task : tasks)
      {
        (*task).run(*this, state);
      }
    }

    void push_back(std::shared_ptr<Task<State>> task)
    {
      tasks.push_back(task);
    }

    template <typename T>
    std::shared_ptr<T> find()
    {
      for (auto &task : tasks)
      {
        if (auto result = std::dynamic_pointer_cast<T>(task))
        {
          return result;
        }
      }
      return nullptr;
    }

    template <typename T>
    std::shared_ptr<T> find(const T *target)
    {
      for (auto &task : tasks)
      {
        if (task.get() == target)
        {
          return std::dynamic_pointer_cast<T>(task);
        }
      }
      return nullptr;
    }

  private:
    std::vector<std::shared_ptr<Task<State>>> tasks;
  };

}

#endif
