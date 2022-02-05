#include "fir_filter.hpp"

namespace filtering
{

  auto FIRFilter::apply(Real input) -> Real
  {
    if (isInitialized)
    {
      history[last_index] = input;
      if (++last_index == tapCount)
        last_index = 0;

      Real acc = 0;

      std::size_t index = last_index;
      for (std::size_t i = 0; i < tapCount; ++i)
      {
        index = index != 0 ? index - 1 : tapCount - 1;
        acc += history[index] * filter_taps[i];
      };

      return acc;
    }
    else
    {
      for (std::size_t i = 0; i < tapCount; ++i)
        history[i] = input;
      last_index = 0;

      isInitialized = true;

      return input;
    }
  }

  void FIRFilter::reset()
  {
    isInitialized = false;
  }

}
