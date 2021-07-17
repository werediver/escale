#include "debug.hpp"
#include <Arduino.h>

void debug_init()
{
  Serial.begin(115200 /* Ignored with USB CDC */);
  while (!Serial)
    ;
}

void debug_send(const std::string &message)
{
  if (Serial)
  {
    Serial.println(message.c_str());
  }
}