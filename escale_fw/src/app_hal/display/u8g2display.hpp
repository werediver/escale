#ifndef APP_HAL_DISPLAY_U8G2DISPLAY_HPP
#define APP_HAL_DISPLAY_U8G2DISPLAY_HPP

#include "display.hpp"

class U8G2;

namespace AppHAL
{

  class U8G2Display final : public Display
  {
  public:
    explicit U8G2Display(U8G2 &u8g2) : u8g2{u8g2} {};

    void clearBuffer() override;
    void sendBuffer() override;
    void drawStr(Coord coord, std::string str) override;

  private:
    U8G2 &u8g2;
  };

}

#endif
