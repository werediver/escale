#ifndef RUN_LOOP_TASK_HPP
#define RUN_LOOP_TASK_HPP

namespace RunLoop
{

  template <typename State>
  class RunLoop;

  template <typename State>
  struct Task
  {
    virtual ~Task() = default;

    virtual void run(RunLoop<State> &, State &) = 0;
  };

  template <typename State>
  class FuncTask final : public Task<State>
  {
  public:
    using Func = void (*)(RunLoop<State> &, State &);

    explicit FuncTask(Func run) : _run{run} {}

    void run(RunLoop<State> &runLoop, State &state) override
    {
      _run(runLoop, state);
    }

  private:
    Func _run;
  };

}

#endif
