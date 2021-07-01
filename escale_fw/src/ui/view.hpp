#ifndef __VIEW_HPP__
#define __VIEW_HPP__

#include <utility> // std::move()

namespace AppHAL
{
  struct Display;
}

template <typename State>
struct View
{
  virtual void build(const State &state) = 0;
  virtual void render(AppHAL::Display &display) = 0;
  virtual bool needsRender() const = 0;
};

template <typename State, typename ViewModel>
class SomeView final : public View<State>
{
public:
  using ViewModelFactory = ViewModel (*)(const State &);
  using ViewRenderer = void (*)(const ViewModel &, AppHAL::Display &);

  SomeView(ViewModelFactory makeViewModel, ViewRenderer renderView)
      : makeViewModel{makeViewModel},
        _needsRender{true},
        renderView{renderView} {}

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

private:
  ViewModelFactory makeViewModel;
  ViewModel viewModel;
  bool _needsRender;
  ViewRenderer renderView;
};

#endif
