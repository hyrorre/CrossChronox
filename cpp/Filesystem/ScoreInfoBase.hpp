#pragma once

#include "pch.hpp"

class ScoreInfoBase;
class ScoreDirectoryInfo;

class ScoreInfoBase {
  public:
    virtual std::string GetTitleSubtitle() const = 0;
    virtual std::string GetInfoStr() const = 0;
    virtual void SetParent(ScoreDirectoryInfo* parent) {}
    std::string path;

    ScoreInfoBase() {}
    ScoreInfoBase(std::string path) : path(path) {}
    virtual ~ScoreInfoBase() {}

    // template <class Context>
    // constexpr static void serde(Context& context, ScoreInfoBase& value) {
    //     serde::serde_struct(context, value)
    //         .field(&ScoreInfoBase::path, "path");
    // }
};
