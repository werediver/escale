#ifndef APP_HAL_DISPLAY_DISPLAY_HPP
#define APP_HAL_DISPLAY_DISPLAY_HPP

#include <cstdint>
#include <string>

namespace AppHAL
{

  struct Display
  {
    struct Coord
    {
      std::uint8_t x;
      std::uint8_t y;
    };

    virtual ~Display() = default;

    virtual void clearBuffer() = 0;
    virtual void sendBuffer() = 0;
    virtual void drawStr(Coord coord, std::string str) = 0;
  };

}

#endif
