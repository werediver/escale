#ifndef __STATE_HPP__
#define __STATE_HPP__

template <typename Tag>
struct State
{
  State(Tag tag) : tag{tag} {}

  virtual ~State() {}

  virtual void onEnter() = 0;
  virtual void onExit() = 0;

  Tag tag;
};

#endif
