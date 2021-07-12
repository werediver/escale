#ifndef UI_INPUT_HPP
#define UI_INPUT_HPP

#include "app_hal/input_handler.hpp"

namespace UI
{

  struct ButtonEvent final
  {
    enum ButtonTag
    {
      ButtonTagA,
      ButtonTagB
    };

    enum Type
    {
      TypeButtonDown
    };

    ButtonTag buttonTag;
    Type type;
  };

  // Use `std:variant` to add other event types.
  using InputEvent = ButtonEvent;

  using InputHandler = AppHAL::InputHandler<InputEvent>;

}

#endif
