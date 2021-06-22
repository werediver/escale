#include "button.hpp"

Button::Button(unsigned long toggleHoldOffDuration)
    : state{ButtonUp},
      timestamp{0},
      isDownPending{false},
      isUpPending{false},
      toggleHoldOffDuration{toggleHoldOffDuration} {}

void Button::update(const ButtonState newState, const unsigned long now)
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
      case ButtonDown:
        isDownPending = true;
        break;
      case ButtonUp:
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
