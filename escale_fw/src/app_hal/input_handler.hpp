#ifndef __INPUT_HANDLER_HPP__
#define __INPUT_HANDLER_HPP__

#include <memory>
#include <vector>

namespace AppHAL
{

  template <typename _ButtonTag>
  struct InputHandler
  {
    using ButtonTag = _ButtonTag;

    virtual ~InputHandler() {}

    virtual void onButtonDown(ButtonTag buttonTag) = 0;
  };

  template <typename ButtonTag>
  struct BlankInputHandler final : InputHandler<ButtonTag>
  {
    void onButtonDown(ButtonTag buttonTag) override {}
  };

}

#endif
