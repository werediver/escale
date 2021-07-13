#ifndef APP_HAL_BUTTON_HPP
#define APP_HAL_BUTTON_HPP

namespace AppHAL
{

  class Button final
  {
  public:
    enum class State
    {
      Down,
      Up
    };

    explicit Button(unsigned long toggleHoldOffDuration);

    /// Call this method to update the state of a button whenever a change is
    /// detected or more often.
    void update(State newState, unsigned long now);

    bool clearIsDownPending();

    bool clearIsUpPending();

  private:
    State state = State::Up;
    /// The timestamp of the last state change according to `now` in
    /// the corresponding `update(newState, now)` call.
    unsigned long timestamp = 0;
    volatile bool isDownPending = false;
    volatile bool isUpPending = false;
    const unsigned long toggleHoldOffDuration;
  };

}

#endif
