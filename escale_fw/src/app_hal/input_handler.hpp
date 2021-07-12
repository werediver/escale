#ifndef APP_HAL_INPUT_HANDLER_HPP
#define APP_HAL_INPUT_HANDLER_HPP

#include <memory>
#include <vector>

namespace AppHAL
{

  template <typename InputEvent>
  struct InputHandler
  {
    virtual ~InputHandler() = default;

    virtual void handleInputEvent(const InputEvent &) = 0;
  };

  template <typename InputEvent>
  struct BlankInputHandler final : InputHandler<InputEvent>
  {
    void handleInputEvent(const InputEvent &) override {}
  };

}

#endif
