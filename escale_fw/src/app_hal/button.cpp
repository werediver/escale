#include "button.hpp"

namespace AppHAL
{

  Button::Button(unsigned long toggleHoldOffDuration)
      : state{StateUp},
        timestamp{0},
        isDownPending{false},
        isUpPending{false},
        toggleHoldOffDuration{toggleHoldOffDuration} {}

  void Button::update(const State newState, const unsigned long now)
  {
    if (newState != state)
    {
      const bool holdOff = now - timestamp <= toggleHoldOffDuration;

      state = newState;
      timestamp = now;

      if (!holdOff)
      {
        switch (state)
        {
        case StateDown:
          isDownPending = true;
          break;
        case StateUp:
          isUpPending = true;
          break;
        }
      }
    }
  }

  bool Button::clearIsDownPending()
  {
    const bool result = isDownPending;
    isDownPending = false;
    return result;
  }

  bool Button::clearIsUpPending()
  {
    const bool result = isUpPending;
    isUpPending = false;
    return result;
  }

}