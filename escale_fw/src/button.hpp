#ifndef __BUTTON_HPP__
#define __BUTTON_HPP__

enum ButtonState
{
  ButtonDown,
  ButtonUp
};

class Button final
{
public:
  explicit Button(unsigned long toggleHoldOffDuration);

  /// Call this method to update the state of a button whenever a change is
  /// detected or more often.
  void update(ButtonState newState, unsigned long now);

  bool clearIsDownPending();

  bool clearIsUpPending();

private:
  ButtonState state;
  /// The timestamp of the last state change according to `now` in
  /// the corresponding `update(newState, now)` call.
  unsigned long timestamp;
  volatile bool isDownPending;
  volatile bool isUpPending;
  const unsigned long toggleHoldOffDuration;
};

#endif
