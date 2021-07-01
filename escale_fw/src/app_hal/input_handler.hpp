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

  template <typename ButtonTag>
  class InputHandlerStack final
  {
  public:
    InputHandlerStack()
        : inputHandlers{std::shared_ptr<BlankInputHandler<ButtonTag>>(
              new BlankInputHandler<ButtonTag>{})} {}

    void push_back(std::shared_ptr<InputHandler<ButtonTag>> inputHandler)
    {
      inputHandlers.push_back(inputHandler);
    }

    void pop_back()
    {
      inputHandlers.pop_back();
    }

    std::shared_ptr<InputHandler<ButtonTag>> back()
    {
      return inputHandlers.back();
    }

  private:
    std::vector<std::shared_ptr<InputHandler<ButtonTag>>> inputHandlers;
  };

}

#endif
