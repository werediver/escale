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

  /// A (partially) type-erased version of `SomeView`.
  template <typename State>
  struct View : InputHandler
  {
    virtual ~View() {}

    virtual void build(const State &state) = 0;
    virtual void render(AppHAL::Display &display) = 0;
    virtual bool needsRender() const = 0;
  };

  template <typename State, typename ViewModel, typename Action>
  class SomeView : public View<State>
  {
  public:
    using ViewModelFactory = ViewModel (*)(const State &);
    using ActionDispatcher = void (*)(Action);

    SomeView(
        ViewModelFactory makeViewModel,
        ActionDispatcher dispatch)
        : makeViewModel{makeViewModel},
          _needsRender{true},
          dispatch{dispatch} {}

    void build(const State &state) final override
    {
      ViewModel newViewModel = makeViewModel(state);
      if (viewModel != newViewModel)
      {
        viewModel = std::move(newViewModel);
        _needsRender = true;
      }
    }

    void render(AppHAL::Display &display) final override
    {
      render(viewModel, display);
      _needsRender = false;
    }

    bool needsRender() const final override
    {
      return _needsRender;
    }

    void handleInputEvent(const InputEvent &inputEvent) final override
    {
      handleInputEvent(viewModel, inputEvent, dispatch);
    }

  private:
    virtual void render(const ViewModel &, AppHAL::Display &) const = 0;
    virtual void handleInputEvent(const ViewModel &, const InputEvent &, ActionDispatcher) const = 0;

    ViewModelFactory makeViewModel;
    ViewModel viewModel;
    bool _needsRender;
    ActionDispatcher dispatch;
  };

}

#endif
