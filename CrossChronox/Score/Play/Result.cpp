//
//  Result.cpp
//  CrossChronox
//
//  Created by HY_RORRE on 2016/12/18.
//  Copyright © 2016年 hyrorre. All rights reserved.
//

#include "Result.hpp"

std::wstring Result::GetResultStr() const{
	return std::wstring(L"PGREAT: ") + std::to_wstring(GetJudgeCount(Judge::PGREAT)) + L'\n';
	 + L"GREAT: " + std::to_wstring(GetJudgeCount(Judge::GREAT)) + L'\n';
	 + L"GOOD: " + std::to_wstring(GetJudgeCount(Judge::GOOD)) + L'\n';
	 + L"BAD: " + std::to_wstring(GetJudgeCount(Judge::BAD)) + L'\n';
	 + L"POOR: " + std::to_wstring(GetJudgeCount(Judge::POOR)) + L'\n';
	 + L"CB: " + std::to_wstring(GetComboBreak()) + L'\n';
	 + L"EXSCORE: " + std::to_wstring(GetExScore()) + L'\n';
	 + L"COMBO: " + std::to_wstring(GetNowCombo()) + L'\n';
	 + L"MAX COMBO: " + std::to_wstring(GetMaxCombo());
}
