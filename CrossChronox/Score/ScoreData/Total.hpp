#pragma once

#include "pch.hpp"
#include "Mode.hpp"

enum : bool {
    TOTAL_RELATIVE = 0,
    TOTAL_ABSOLUTE = 1
};

class Total {
    bool type = TOTAL_RELATIVE;
    double value = 100;

  public:
    Total() {}
    Total(bool type, double value) : type(type), value(value) {}

    bool GetType() const {
        return type;
    }
    void SetType(bool type) {
        this->type = type;
    }
    double GetValue() const {
        return value;
    }
    void SetValue(double value) {
        this->value = value;
    }

    //template <class Context>
    //constexpr static void serde(Context& context, Total& value) {
    //    serde::serde_struct(context, value)
    //        .field(&Total::type, "type")
    //        .field(&Total::value, "value");
    //}
};
