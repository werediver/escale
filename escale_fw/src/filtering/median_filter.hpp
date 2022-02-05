#ifndef MEDIAN_FILTER_HPP
#define MEDIAN_FILTER_HPP

#include "filter.hpp"

namespace filtering
{

  template <typename T>
  class MedianFilter final : public Filter<T>
  {
  public:
    explicit MedianFilter(std::size_t w)
        : xs(w, 0) {}

    auto apply(T input) -> T override
    {
      if (isInitialized)
      {
        xs[head] = input;
        if (++head == xs.size())
          head = 0;

        std::vector<T> buffer(xs.size() / 2 + 1);
        std::partial_sort_copy(xs.begin(), xs.end(), buffer.begin(), buffer.end());
        return buffer.back();
      }
      else
      {
        std::fill(xs.begin(), xs.end(), input);
        isInitialized = true;
        return input;
      }
    }

    void reset() override
    {
      isInitialized = false;
    }

  private:
    bool isInitialized = false;
    std::size_t head = 0;
    std::vector<T> xs;
  };

}

#endif
