#ifndef __INPUT_HANDLER_HPP__
#define __INPUT_HANDLER_HPP__

#include <memory>
#include <vector>

namespace AppHAL
{

  template <typename InputEvent>
  struct InputHandler
  {
    virtual ~InputHandler() {}

    virtual void handleInputEvent(const InputEvent &) = 0;
  };

  template <typename InputEvent>
  struct BlankInputHandler final : InputHandler<InputEvent>
  {
    void handleInputEvent(const InputEvent &) override {}
  };

}

#endif
