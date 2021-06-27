#ifndef __MESSAGE_VIEW_HPP__
#define __MESSAGE_VIEW_HPP__

struct MessageViewModel final
{
  enum Message
  {
    MessageTaring,
    MessageCalibrating,
    MessageNAU7802NotFound
  };

  Message message;
};

namespace AppHAL
{
  struct Display;
}

void displayMessageView(const MessageViewModel &viewModel, AppHAL::Display &display);

#endif
