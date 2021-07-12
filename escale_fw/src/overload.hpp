#ifndef OVERLOAD_HPP
#define OVERLOAD_HPP

// References:
// - std::visit is everything wrong with modern C++
//   https://bitbashing.io/std-visit.html
// - Overloading Lambdas in C++17
//   https://dev.to/tmr232/that-overloaded-trick-overloading-lambdas-in-c17
// - Overloaded lambda with variadic template on SO
//   https://stackoverflow.com/a/32476942/3541063

template <typename... Ts>
struct overload : Ts...
{
  // To support aggregate initialization (only) this template requires either
  // a user-defined deduction guide (until C++20)
  //
  //     template <typename... Ts>
  //     overload(Ts...) -> overload<Ts...>;
  //
  // or a deleted constructor declaration
  //
  //     overload(Ts...) = delete;
  //
  // To support both the normal and aggregate initialization a proper
  // constructor declaration is required
  //
  //     overload(Ts... ts) : Ts(ts)... {}
  //
  overload(Ts...) = delete;

  // g++ requires this explicit using-declaration, clang doesn't
  using Ts::operator()...;
};

#endif
