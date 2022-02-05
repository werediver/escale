#ifndef FILTERING_THRESHOLD_CHANGE_DETECTOR_HPP
#define FILTERING_THRESHOLD_CHANGE_DETECTOR_HPP

#include <cmath>  // std::abs()
#include <memory> // std::unique_ptr
#include <optional>
#include <utility>

// #include "filter.hpp"

namespace filtering
{

  template <typename T>
  class ThresholdChangeDetector final
  {
  public:
    using ChangeHandler = void (*)();

    ThresholdChangeDetector(T threshold, ChangeHandler onChange)
        : threshold{threshold}, onChange{onChange} {}

    void apply(T input)
    {
      if (lastInput.has_value() && std::abs(input - *lastInput) >= threshold)
        onChange();
      lastInput = input;
    }

  private:
    const T threshold;
    std::optional<T> lastInput;
    ChangeHandler onChange;
  };

}

#endif
