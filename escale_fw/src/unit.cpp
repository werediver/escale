#include "unit.hpp"

bool Unit::operator==(const Unit &other) const
{
  return true;
}

bool Unit::operator!=(const Unit &other) const
{
  return !(other == *this);
}
