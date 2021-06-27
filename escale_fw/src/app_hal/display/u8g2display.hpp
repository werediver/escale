#ifndef __U8G2DISPLAY_HPP__
#define __U8G2DISPLAY_HPP__

#include "display.hpp"

class U8G2;

namespace AppHAL
{

  class U8G2Display final : public Display
  {
  public:
    U8G2Display(U8G2 &u8g2) : u8g2{u8g2} {};

    void clearBuffer() override;
    void sendBuffer() override;
    void drawStr(Coord coord, std::string str) override;

  private:
    U8G2 &u8g2;
  };

}

#endif
