#include "app_hal/button.hpp"
#include "app_hal/display/u8g2display.hpp"
#include "app_state.hpp"
#include "run_loop/run_loop.hpp"
#include "ui/calibration/calibration_task.hpp"
#include "ui/calibration/calibration_view.hpp"
#include "ui/dashboard/dashboard_task.hpp"
#include "ui/dashboard/dashboard_view.hpp"
#include "ui/message/message_view.hpp"
#include "ui/taring/taring_task.hpp"
#include "ui/view_stack_task.hpp"

#include "filtering/median_filter.hpp"
#include "filtering/fir_filter.hpp"
#include "filtering/threshold_change_detector.hpp"

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

filtering::MedianFilter<std::int32_t> rawWeightFilter1{5};
filtering::FIRFilter rawWeightFilter2;
filtering::ThresholdChangeDetector<filtering::Real> weightChangeDetector(
    2.0f,
    []()
    {
      rawWeightFilter1.reset();
      rawWeightFilter2.reset();
    });

void readWeight(AppState &state)
{
  auto rawToWeight = [&](std::int32_t rawWeight)
  { return (rawWeight - state.zeroOffset) / state.calibrationFactor; };

  for (auto i = 0; i < 64; ++i)
  {
    const auto newRawWeigh = rawWeightFilter1.apply(nau7802.getAverage(1));
    weightChangeDetector.apply(rawToWeight(newRawWeigh));
    state.rawWeigh = rawWeightFilter2.apply(newRawWeigh);
    state.weight = rawToWeight(state.rawWeigh);
    state.readCount += 1;
  }
}

static constexpr auto refSampleCount = 320 * 3;

void calculateZeroOffset(AppState &state)
{
  rawWeightFilter1.reset();
  rawWeightFilter2.reset();

  std::int64_t rawWeight = 0;
  auto n = 0;
  for (const auto endReadCount = state.readCount + refSampleCount; state.readCount < endReadCount;)
  {
    readWeight(state);
    rawWeight += state.rawWeigh;
    n += 1;
  }
  rawWeight /= n;
  state.zeroOffset = (std::int32_t)rawWeight;
}

void calculateCalibrationFactor(AppState &state)
{
  static constexpr auto refWeight = 100;
  std::int64_t rawWeight = 0;
  auto n = 0;
  for (const auto endReadCount = state.readCount + refSampleCount; state.readCount < endReadCount;)
  {
    readWeight(state);
    rawWeight += state.rawWeigh;
    n += 1;
  }
  rawWeight /= n;
  state.calibrationFactor = ((std::int32_t)rawWeight - state.zeroOffset) / refWeight;
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
    nau7802.setGain(NAU7802_GAIN_128);
    nau7802.setSampleRate(NAU7802_SPS_320);
    nau7802.calibrateAFE();

    const auto rawWeightSample = nau7802.getAverage(1);
    calculateZeroOffset(state);

    state.rawWeigh = rawWeightSample;
  }

  runLoop.push_back(std::make_shared<RunLoop::FuncTask<AppState>>(
      [](RunLoop::RunLoop<AppState> &runLoop, AppState &state)
      {
        readButtons(runLoop);
        readWeight(state);
      }));
  runLoop.push_back(std::make_shared<UI::ViewStackTask<AppState>>(display));
  runLoop.push_back(std::make_shared<UI::DashboardTask<AppState>>(
      []()
      { return std::make_shared<UI::TaringTask<AppState>>(
            []()
            { calculateZeroOffset(state); }); },
      []()
      { return std::make_shared<UI::CalibrationTask<AppState>>(
            []()
            { calculateCalibrationFactor(state); },
            [](const AppState &state)
            {
              return UI::CalibrationViewModel{state.zeroOffset, state.rawWeigh};
            }); },
      [](const AppState &state)
      { return UI::DashboardViewModel{state.weight}; }));
}

void loop()
{
  runLoop.run(state);
}