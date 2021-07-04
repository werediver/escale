#include <Arduino.h>
#include <U8g2lib.h>
#include "SparkFun_Qwiic_Scale_NAU7802_Arduino_Library.h"
#include <memory>
#include <vector>

#include "app_hal/button.hpp"
#include "app_hal/display/u8g2display.hpp"
#include "app_state.hpp"
#include "ui/dashboard/dashboard_input_handler.hpp"
#include "ui/dashboard/dashboard_view.hpp"
#include "ui/message/message_view.hpp"
#include "ui/view.hpp"
#include "unit.hpp"

U8G2_SSD1306_128X64_NONAME_F_HW_I2C u8g2{U8G2_R0, U8X8_PIN_NONE, SCL, SDA};
AppHAL::U8G2Display display{u8g2};
NAU7802 nau7802;

const uint32_t buttonAPin = 7;
const uint32_t buttonBPin = 8;
const unsigned long buttonToggleHoldOffDuration = 20; // ms
AppHAL::Button buttonA{buttonToggleHoldOffDuration};
AppHAL::Button buttonB{buttonToggleHoldOffDuration};

AppState state;

template <typename Context>
using Task = void (*)(Context &);

std::vector<Task<AppState>> tasks;

std::vector<std::shared_ptr<UI::View<AppState>>> viewStack;
bool needsRender = true;

void readButtons(int32_t &n)
{
  UI::InputHandler &inputHandler = *viewStack.back();
  if (buttonA.clearIsDownPending())
    inputHandler.handleInputEvent(
        {UI::ButtonEvent::ButtonTagA,
         UI::ButtonEvent::TypeButtonDown});
  if (buttonB.clearIsDownPending())
    inputHandler.handleInputEvent(
        {UI::ButtonEvent::ButtonTagB,
         UI::ButtonEvent::TypeButtonDown});
}

void readWeight(float &w)
{
  w = nau7802.getWeight(true, 1);
}

void updateDisplay(const AppState &state)
{
  if (!viewStack.empty())
  {
    auto &view = *viewStack.back();
    view.build(state);
    if (needsRender || view.needsRender())
    {
      view.render(display);
      needsRender = false;
    }
  }
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

  viewStack.push_back(std::make_shared<UI::SomeView<AppState, DashboardViewModel, Unit>>(
      [](const AppState &state)
      {
        return DashboardViewModel{state.n, state.w};
      },
      [](Unit) {},
      renderDashboardView,
      [](const DashboardViewModel &, const UI::ButtonEvent &buttonEvent, void (*)(Unit))
      {
        switch (buttonEvent.buttonTag)
        {
        case UI::ButtonEvent::ButtonTagA:
          if (buttonEvent.type == UI::ButtonEvent::TypeButtonDown)
            state.n += 1;
          break;
        case UI::ButtonEvent::ButtonTagB:
          if (buttonEvent.type == UI::ButtonEvent::TypeButtonDown)
            state.n -= 1;
          break;
        }
      }));

  tasks.push_back([](AppState &state)
                  { readButtons(state.n); });
  tasks.push_back([](AppState &state)
                  { readWeight(state.w); });
  tasks.push_back([](AppState &state)
                  { updateDisplay(state); });
}

void loop()
{
  for (const auto &task : tasks)
    task(state);

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