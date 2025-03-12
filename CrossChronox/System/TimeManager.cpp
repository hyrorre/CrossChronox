#include "TimeManager.hpp"

namespace TimeManager {
    ms_type lastframe_ms_ = 0;
    ms_type now_ms_ = 0;
    ms_type delta_ms_ = 0;

    const ms_type& lastframe_ms = lastframe_ms_;
    const ms_type& now_ms = now_ms_;
    const ms_type& delta_ms = delta_ms_;

    void Update() {
        lastframe_ms_ = now_ms_;
        now_ms_ = static_cast<ms_type>(GetTime() * 1000);
        delta_ms_ = now_ms_ - lastframe_ms_;
    }

    ms_type GetRealtime() {
        return static_cast<ms_type>(GetTime() * 1000);
    }
} // namespace TimeManager
