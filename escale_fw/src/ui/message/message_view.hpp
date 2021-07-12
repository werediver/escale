#ifndef UI_MESSAGE_MESSAGE_VIEW_HPP
#define UI_MESSAGE_MESSAGE_VIEW_HPP

struct MessageViewModel final
{
  enum Message
  {
    MessageTaring,
    MessageCalibrating,
    MessageNAU7802NotFound
  };

  Message message;

  bool operator==(const MessageViewModel &other) const;
  bool operator!=(const MessageViewModel &other) const;
};

namespace AppHAL
{
  struct Display;
}

void renderMessageView(const MessageViewModel &viewModel, AppHAL::Display &display);

#endif
