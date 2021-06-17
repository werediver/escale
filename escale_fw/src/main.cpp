#include <Arduino.h>
#include <U8g2lib.h>
#include "SparkFun_Qwiic_Scale_NAU7802_Arduino_Library.h"

#include "button.hpp"
#include "state.hpp"

U8G2_SSD1306_128X64_NONAME_F_HW_I2C u8g2(U8G2_R0, U8X8_PIN_NONE, SCL, SDA);
NAU7802 nau7802;

const uint32_t buttonAPin = 7;
const uint32_t buttonBPin = 8;
const unsigned long buttonToggleHoldOffDuration = 20; // ms
Button buttonA(buttonToggleHoldOffDuration);
Button buttonB(buttonToggleHoldOffDuration);

State state;

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

  if (nau7802.begin())
  {
    nau7802.calculateZeroOffset();
    // Fake calibration to make `getWeight` return something more meaningful then ±inf.
    nau7802.setCalibrationFactor((20000 - nau7802.getZeroOffset()) / 1);
  }
  else
  {
    state.mode = ModeNau7802NotFound;
  }
}

void loop()
{
  switch (state.mode)
  {
  case ModeNormal:
  {
    if (buttonA.clearIsDownPending())
    {
      state.n += 1;
    }
    if (buttonB.clearIsDownPending())
    {
      state.n -= 1;
    }

    float w = nau7802.getWeight(true, 1);

    char s1[32];
    sprintf(s1, "n=%li", state.n);
    char s2[10], s3[32];
    dtostrf(w, 6, 1, s2);
    sprintf(s3, "w=%s", s2);

    u8g2.clearBuffer();
    u8g2.drawStr(0, 10, s1);
    u8g2.drawStr(0, 20, s3);
    u8g2.sendBuffer();
    break;
  }
  case ModeNau7802NotFound:
    u8g2.clearBuffer();
    u8g2.drawStr(0, 10, "E: NAU7802 not found");
    u8g2.sendBuffer();
    state.mode = ModeHalt;
    break;
  case ModeHalt:
    break;
  }
}