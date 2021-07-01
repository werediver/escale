#ifndef __APP_INPUT_HPP__
#define __APP_INPUT_HPP__

#include "app_hal/input_handler.hpp"

namespace AppInput
{

  enum ButtonTag
  {
    ButtonA,
    ButtonB
  };

  using InputHandler = AppHAL::InputHandler<ButtonTag>;

}

#endif
