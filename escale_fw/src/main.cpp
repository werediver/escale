#include <Arduino.h>
#include <U8g2lib.h>
#include "SparkFun_Qwiic_Scale_NAU7802_Arduino_Library.h"
#include <vector>

#include "overloaded.hpp"
#include "button.hpp"
#include "u8g2display.hpp"
#include "views.hpp"
#include "state.hpp"

U8G2_SSD1306_128X64_NONAME_F_HW_I2C u8g2{U8G2_R0, U8X8_PIN_NONE, SCL, SDA};
U8G2Display display{u8g2};
NAU7802 nau7802;

const uint32_t buttonAPin = 7;
const uint32_t buttonBPin = 8;
const unsigned long buttonToggleHoldOffDuration = 20; // ms
Button buttonA{buttonToggleHoldOffDuration};
Button buttonB{buttonToggleHoldOffDuration};

State state;

template <typename Context>
using Task = void (*)(Context &);

std::vector<Task<State>> tasks;

void handleButtons(int32_t &n)
{
  if (buttonA.clearIsDownPending())
    n += 1;
  if (buttonB.clearIsDownPending())
    n -= 1;
}

void readWeight(float &w)
{
  w = nau7802.getWeight(true, 1);
}

void updateDisplay(const State &state)
{
  switch (state.mode)
  {
  case ModeNormal:
  {
    displayDashboardView(DashboardViewModel{state.n, state.w}, display);
    break;
  }
  case ModeNau7802NotFound:
    displayErrorView(ErrorViewModel{ErrorViewModel::ErrorNAU7802NotFound}, display);
    break;
  case ModeHalt:
    break;
  }
}

void updateButtons()
{
  const unsigned long now = millis();
  buttonA.update(digitalRead(buttonAPin) == 0 ? ButtonDown : ButtonUp, now);
  buttonB.update(digitalRead(buttonBPin) == 0 ? ButtonDown : ButtonUp, now);
}

void setup()
{
  pinMode(buttonAPin, INPUT_PULLUP);
  pinMode(buttonBPin, INPUT_PULLUP);

  attachInterrupt(buttonAPin, updateButtons, CHANGE);
  attachInterrupt(buttonBPin, updateButtons, CHANGE);

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

  tasks.push_back([](State &state)
                  { handleButtons(state.n); });
  tasks.push_back([](State &state)
                  { readWeight(state.w); });
  tasks.push_back([](State &state)
                  { updateDisplay(state); });
}

void loop()
{
  for (const auto &task : tasks)
    task(state);

  switch (state.mode)
  {
  case ModeNormal:
    break;
  case ModeNau7802NotFound:
    state.mode = ModeHalt;
    break;
  case ModeHalt:
    break;
  }
}