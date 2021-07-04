#ifndef __VIEW_HPP__
#define __VIEW_HPP__

#include <utility> // std::move()
#include "input.hpp"

namespace AppHAL
{
  struct Display;
}

namespace UI
{

  template <typename State>
  struct View : InputHandler
  {
    virtual ~View() {}

    virtual void build(const State &state) = 0;
    virtual void render(AppHAL::Display &display) = 0;
    virtual bool needsRender() const = 0;
  };

  template <typename State, typename ViewModel, typename Action>
  class SomeView final : public View<State>
  {
  public:
    using ViewModelFactory = ViewModel (*)(const State &);
    using ActionDispatcher = void (*)(Action);
    using ViewRenderer = void (*)(const ViewModel &, AppHAL::Display &);
    using InputHandler = void (*)(const ViewModel &, const InputEvent &, ActionDispatcher);

    SomeView(
        ViewModelFactory makeViewModel,
        ActionDispatcher dispatch,
        ViewRenderer renderView,
        InputHandler handleInputEvent)
        : makeViewModel{makeViewModel},
          _needsRender{true},
          renderView{renderView},
          dispatch{dispatch},
          _handleInputEvent{handleInputEvent} {}

    void build(const State &state) override
    {
      ViewModel newViewModel = makeViewModel(state);
      if (viewModel != newViewModel)
      {
        viewModel = std::move(newViewModel);
        _needsRender = true;
      }
    }

    void render(AppHAL::Display &display) override
    {
      renderView(viewModel, display);
      _needsRender = false;
    }

    bool needsRender() const override
    {
      return _needsRender;
    }

    void handleInputEvent(const InputEvent &inputEvent) override
    {
      _handleInputEvent(viewModel, inputEvent, dispatch);
    }

  private:
    ViewModelFactory makeViewModel;
    ViewModel viewModel;
    bool _needsRender;
    ViewRenderer renderView;
    ActionDispatcher dispatch;
    InputHandler _handleInputEvent;
  };

}

#endif
