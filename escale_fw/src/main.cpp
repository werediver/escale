#include "app_hal/button.hpp"
#include "app_hal/display/u8g2display.hpp"
#include "app_state.hpp"
#include "run_loop/run_loop.hpp"
#include "ui/dashboard/dashboard_task.hpp"
#include "ui/dashboard/dashboard_view.hpp"
#include "ui/message/message_view.hpp"
#include "ui/taring/taring_task.hpp"
#include "ui/view_stack_task.hpp"

#include <Arduino.h>
#include <SparkFun_Qwiic_Scale_NAU7802_Arduino_Library.h>
#include <U8g2lib.h>

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

void readButtons(RunLoop::RunLoop<AppState> &runLoop)
{
  if (const auto pViewStack = runLoop.find<UI::ViewStackTask<AppState>>())
  {
    if (const auto pInputHandler = pViewStack->back())
    {
      if (buttonA.clearIsDownPending())
        pInputHandler->handleInputEvent(
            {UI::ButtonEvent::ButtonTag::A,
             UI::ButtonEvent::Type::ButtonDown});
      if (buttonB.clearIsDownPending())
        pInputHandler->handleInputEvent(
            {UI::ButtonEvent::ButtonTag::B,
             UI::ButtonEvent::Type::ButtonDown});
    }
  }
}

void readWeight(float &w)
{
  w = nau7802.getWeight(true, 4);
}

void updateButtons()
{
  const unsigned long now = millis();
  buttonA.update(
      digitalRead(buttonAPin) == 0
          ? AppHAL::Button::State::Down
          : AppHAL::Button::State::Up,
      now);
  buttonB.update(
      digitalRead(buttonBPin) == 0
          ? AppHAL::Button::State::Down
          : AppHAL::Button::State::Up,
      now);
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
    nau7802.setCalibrationFactor((20000 - nau7802.getZeroOffset()) / 1.0);
  }

  runLoop.push_back(std::make_shared<RunLoop::FuncTask<AppState>>(
      [](RunLoop::RunLoop<AppState> &runLoop, AppState &state)
      { readButtons(runLoop); }));
  runLoop.push_back(std::make_shared<RunLoop::FuncTask<AppState>>(
      [](RunLoop::RunLoop<AppState> &, AppState &state)
      { readWeight(state.w); }));
  runLoop.push_back(std::make_shared<UI::ViewStackTask<AppState>>(display));
  runLoop.push_back(std::make_shared<UI::DashboardTask<AppState>>(
      []()
      { return std::make_shared<UI::TaringTask<AppState>>(
            [](std::uint8_t sampleCount)
            { nau7802.calculateZeroOffset(sampleCount); }); },
      [](const AppState &state)
      { return UI::DashboardViewModel{state.w}; }));
}

void loop()
{
  runLoop.run(state);
}