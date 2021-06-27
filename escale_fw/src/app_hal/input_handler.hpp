#ifndef __INPUT_HANDLER_HPP__
#define __INPUT_HANDLER_HPP__

namespace AppHAL
{

  template <typename ButtonTag>
  struct InputHandler
  {
    virtual ~InputHandler() {}

    virtual void onButtonDown(ButtonTag buttonTag) = 0;
  };

}

#endif
