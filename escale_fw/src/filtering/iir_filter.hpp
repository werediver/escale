#ifndef IIR_FILTER_HPP
#define IIR_FILTER_HPP

#include <array>

#include "filter.hpp"

namespace filtering
{

  class IIRFilter final : public Filter<Real>
  {
  public:
    auto apply(Real x) noexcept -> Real override;
    void reset() noexcept override;

  private:
    static constexpr Real bcoeff[] = {
        1.000000e+00,
        -5.649954e+00,
        1.330764e+01,
        -1.672533e+01,
        1.183003e+01,
        -4.464853e+00,
        7.024659e-01,
    };
    static constexpr Real acoeff[] = {
        3.085560e-09,
        1.851336e-08,
        4.628340e-08,
        6.171120e-08,
        4.628340e-08,
        1.851336e-08,
        3.085560e-09,
    };
    static constexpr Real a0 = 1;

    bool isInitialized = false;
    Real xs[std::size(bcoeff)];
    Real ys[std::size(acoeff)];
  };

}

#endif
