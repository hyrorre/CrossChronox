//
//  Account.hpp
//  CrossChronox
//
//  Created by HY_RORRE on 2017/04/19.
//  Copyright © 2017年 hyrorre. All rights reserved.
//

#ifndef Account_hpp
#define Account_hpp

#include "pch.hpp"

class AccountInfo{
	std::string name; // DJ NAME, CARD NAME
	
	double hsbpm_step = 10;
	int sud_hid_step = 10;
	int lift_step = 10;
	
public:
	const std::string& GetName() const{
		return name;
	}
	double GetHsBpmStep() const{
		return hsbpm_step;
	}
	int GetSudHidStep() const{
		return sud_hid_step;
	}
	int GetLiftStep() const{
		return lift_step;
	}
};

class ResultDatabase{
	
};

struct Account{
	AccountInfo info;
	ResultDatabase result_database;
};

extern std::vector<Account> accounts;

void LoadAccounts(std::string directorypath);

#endif /* Account_hpp */
