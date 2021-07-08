#include "app_hal/button.hpp"
#include "app_hal/display/u8g2display.hpp"
#include "app_state.hpp"
#include "run_loop/run_loop.hpp"
#include "ui/dashboard/dashboard_view.hpp"
#include "ui/message/message_view.hpp"
#include "ui/view_stack_task.hpp"

#include <Arduino.h>
#include <U8g2lib.h>
#include <SparkFun_Qwiic_Scale_NAU7802_Arduino_Library.h>

U8G2_SSD1306_128X64_NONAME_F_HW_I2C u8g2{U8G2_R0, U8X8_PIN_NONE, SCL, SDA};
AppHAL::U8G2Display display{u8g2};
NAU7802 nau7802;

const uint32_t buttonAPin = 7;
const uint32_t buttonBPin = 8;
const unsigned long buttonToggleHoldOffDuration = 20; // ms
AppHAL::Button buttonA{buttonToggleHoldOffDuration};
AppHAL::Button buttonB{buttonToggleHoldOffDuration};

AppState state;
RunLoop::RunLoop<AppState> runLoop;

void readButtons(int32_t &n)
{
  if (auto viewStack = runLoop.find<UI::ViewStackTask<AppState>>())
  {
    UI::InputHandler &inputHandler = *(*viewStack).back();
    if (buttonA.clearIsDownPending())
      inputHandler.handleInputEvent(
          {UI::ButtonEvent::ButtonTagA,
           UI::ButtonEvent::TypeButtonDown});
    if (buttonB.clearIsDownPending())
      inputHandler.handleInputEvent(
          {UI::ButtonEvent::ButtonTagB,
           UI::ButtonEvent::TypeButtonDown});
  }
}

void readWeight(float &w)
{
  w = nau7802.getWeight(true, 1);
}

void updateButtons()
{
  const unsigned long now = millis();
  buttonA.update(digitalRead(buttonAPin) == 0 ? AppHAL::Button::StateDown : AppHAL::Button::StateUp, now);
  buttonB.update(digitalRead(buttonBPin) == 0 ? AppHAL::Button::StateDown : AppHAL::Button::StateUp, now);
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
    state.mode = AppModeTagNau7802NotFound;
  }

  runLoop.push_back(std::make_shared<RunLoop::FuncTask<AppState>>(
      [](RunLoop::RunLoop<AppState> &, AppState &state)
      { readButtons(state.n); }));
  runLoop.push_back(std::make_shared<RunLoop::FuncTask<AppState>>(
      [](RunLoop::RunLoop<AppState> &, AppState &state)
      { readWeight(state.w); }));
  runLoop.push_back(std::make_shared<UI::ViewStackTask<AppState>>(display));

  if (auto viewStack = runLoop.find<UI::ViewStackTask<AppState>>())
  {
    (*viewStack)
        .push_back(std::make_shared<UI::DashboardView<AppState>>(
            [](const AppState &state)
            { return UI::DashboardViewModel{state.n, state.w}; },
            [](UI::DashboardAction action)
            {
              switch (action)
              {
              case UI::DashboardActionIncrementN:
                state.n += 1;
                break;
              case UI::DashboardActionDecrementN:
                state.n -= 1;
                break;
              }
            }));
  }
}

void loop()
{
  runLoop.run(state);

  switch (state.mode)
  {
  case AppModeTagNormal:
    break;
  case AppModeTagNau7802NotFound:
    state.mode = AppModeTagHalt;
    break;
  case AppModeTagHalt:
    break;
  }
}