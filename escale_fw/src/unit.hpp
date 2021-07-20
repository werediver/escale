#ifndef UNIT_HPP
#define UNIT_HPP

struct Unit final
{
  bool operator==(const Unit &other) const;
  bool operator!=(const Unit &other) const;
};

#endif
