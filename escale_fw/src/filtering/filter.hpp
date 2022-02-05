#ifndef FILTERING_FILTER_HPP
#define FILTERING_FILTER_HPP

namespace filtering
{

  using Real = float;

  template <typename T>
  struct Filter
  {
    virtual ~Filter() = default;

    virtual auto apply(T) -> T = 0;
    virtual void reset() = 0;
  };

}

#endif
