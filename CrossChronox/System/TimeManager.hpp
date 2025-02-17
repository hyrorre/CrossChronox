#pragma once

#include "pch.hpp"

namespace TimeManager {
using ms_type = sf::Uint32;
using sec_type = double;
using min_type = double;

extern const ms_type& lastframe_ms;
extern const ms_type& now_ms;
extern const ms_type& delta_ms;

void Update();

inline min_type MsToMin(ms_type ms) {
    return ms / (1000.0 * 60.0);
}
inline ms_type MinToMs(min_type min) {
    return min * 1000.0 * 60.0;
}

ms_type GetRealtime();
}; // namespace TimeManager

using namespace TimeManager;
