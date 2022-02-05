#include "iir_filter.hpp"

namespace filtering
{

  Real IIRFilter::apply(Real x) noexcept
  {
    if (isInitialized)
    {
      Real y = 0;

      for (auto i = 0; i < order; ++i)
      {
        xs[i] = xs[i + 1];
        ys[i] = ys[i + 1];
      }
      xs[order] = x / gain;

      for (auto i = 0; i < order; ++i)
        y += xs[i] * bcoeff[i] - ys[i] * acoeff[i];
      y += xs[order] * bcoeff[order];

      ys[order] = y;

      return y;
    }
    else
    {
      for (auto i = 0; i < order + 1; ++i)
      {
        xs[i] = x / gain;
        ys[i] = x;
      }

      isInitialized = true;

      return x;
    }
  }

  void IIRFilter::reset() noexcept
  {
    isInitialized = false;
  }

}
