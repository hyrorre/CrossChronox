#include "Result.hpp"

std::string Result::GetResultStr() const {
    std::stringstream ss;
    ss << "PGREAT: " << GetJudgeCount(Judge::PGREAT) << '\n';
    ss << "GREAT: " << GetJudgeCount(Judge::GREAT) << '\n';
    ss << "GOOD: " << GetJudgeCount(Judge::GOOD) << '\n';
    ss << "BAD: " << GetJudgeCount(Judge::BAD) << '\n';
    ss << "POOR: " << GetJudgeCount(Judge::POOR) << '\n';
    ss << "CB: " << GetComboBreak() << '\n';
    ss << "EXSCORE: " << GetExScore() << '\n';
    ss << "COMBO: " << GetNowCombo() << '\n';
    ss << "MAX COMBO: " << GetMaxCombo() << '\n';
    ss << std::endl;

    return ss.str();
}
