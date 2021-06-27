#ifndef __BUTTON_HPP__
#define __BUTTON_HPP__

namespace AppHAL
{

  class Button final
  {
  public:
    enum State
    {
      StateDown,
      StateUp
    };

    explicit Button(unsigned long toggleHoldOffDuration);

    /// Call this method to update the state of a button whenever a change is
    /// detected or more often.
    void update(State newState, unsigned long now);

    bool clearIsDownPending();

    bool clearIsUpPending();

  private:
    State state;
    /// The timestamp of the last state change according to `now` in
    /// the corresponding `update(newState, now)` call.
    unsigned long timestamp;
    volatile bool isDownPending;
    volatile bool isUpPending;
    const unsigned long toggleHoldOffDuration;
  };

}

#endif
