#ifndef __APP_INPUT_HPP__
#define __APP_INPUT_HPP__

#include "app_hal/input_handler.hpp"

namespace AppInput
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
