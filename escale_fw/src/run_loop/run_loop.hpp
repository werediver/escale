#ifndef __RUN_LOOP_HPP__
#define __RUN_LOOP_HPP__

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

  private:
    std::vector<std::shared_ptr<Task<State>>> tasks;
  };

}

#endif
