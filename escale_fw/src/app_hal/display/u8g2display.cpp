#include <U8g2lib.h>
#include "u8g2display.hpp"

namespace AppHAL
{

  void U8G2Display::clearBuffer()
  {
    u8g2.clearBuffer();
  }

  void U8G2Display::sendBuffer()
  {
    u8g2.sendBuffer();
  }

  void U8G2Display::drawStr(Coord coord, std::string str)
  {
    u8g2.drawStr(coord.x, coord.y, str.c_str());
  }

}
