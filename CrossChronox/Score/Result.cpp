//
//  Result.cpp
//  CrossChronox
//
//  Created by HY_RORRE on 2016/12/18.
//  Copyright © 2016年 hyrorre. All rights reserved.
//

#include "Result.hpp"

std::wstring Result::GetResultStr() const{
	std::wstringstream ss;
	ss << L"PGREAT: " << GetJudge(Judge::PGREAT) << L'\n';
	ss << L"GREAT: " << GetJudge(Judge::GREAT) << L'\n';
	ss << L"GOOD: " << GetJudge(Judge::GOOD) << L'\n';
	ss << L"BAD: " << GetJudge(Judge::BAD) << L'\n';
	ss << L"POOR: " << GetJudge(Judge::POOR) << L'\n';
	ss << L"CB: " << GetComboBreak() << L'\n';
	ss << L"EXSCORE: " << GetExScore() << L'\n';
	ss << L"COMBO: " << GetNowCombo() << L'\n';
	ss << L"MAX COMBO: " << GetMaxCombo() << L'\n';
	ss << std::endl;
	
	return ss.str();
}
