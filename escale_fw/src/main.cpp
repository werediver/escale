#include <Arduino.h>
#include <U8g2lib.h>

#include "button.hpp"

U8G2_SSD1306_128X64_NONAME_F_HW_I2C u8g2(U8G2_R0, U8X8_PIN_NONE, SCL, SDA);

const uint32_t buttonAPin = 7;
const uint32_t buttonBPin = 8;
const unsigned long buttonToggleHoldOffDuration = 20; // ms
Button buttonA(buttonToggleHoldOffDuration);
Button buttonB(buttonToggleHoldOffDuration);

volatile int32_t n = 0;

void buttonHandler()
{
  const unsigned long now = millis();
  buttonA.update(digitalRead(buttonAPin) == 0 ? ButtonDown : ButtonUp, now);
  buttonB.update(digitalRead(buttonBPin) == 0 ? ButtonDown : ButtonUp, now);
}

void setup()
{
  pinMode(buttonAPin, INPUT_PULLUP);
  pinMode(buttonBPin, INPUT_PULLUP);

  attachInterrupt(buttonAPin, buttonHandler, CHANGE);
  attachInterrupt(buttonBPin, buttonHandler, CHANGE);

  u8g2.begin();
  u8g2.setFont(u8g2_font_profont12_tf);
}

void loop()
{
  if (buttonA.clearIsDownPending())
  {
    n += 1;
  }
  if (buttonB.clearIsDownPending())
  {
    n -= 1;
  }

  char s[32];
  sprintf(s, "n=%li", n);

  u8g2.clearBuffer();
  u8g2.drawStr(0, 10, s);
  u8g2.sendBuffer();
}