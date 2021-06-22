#ifndef __DISPLAY_HPP__
#define __DISPLAY_HPP__

#include <cstdint>
#include <string>

class Display
{
public:
  struct Coord
  {
    std::uint8_t x;
    std::uint8_t y;
  };

  virtual void clearBuffer() = 0;
  virtual void sendBuffer() = 0;
  virtual void drawStr(Coord coord, std::string str) = 0;
};

#endif
