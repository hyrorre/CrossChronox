#pragma once

#include "pch.hpp"
#include "Score/Play/PlayOption.hpp"

class AccountInfo{
	std::string name; // DJ NAME, CARD NAME
	PlayOption option;
	
	double hsbpm_step = 10;
	int sud_hid_step = 10;
	int lift_step = 10;
	
public:
	const std::string& GetName() const{
		return name;
	}
	const PlayOption& GetPlayOption() const{
		return option;
	}
	PlayOption& GetVariablePlayOption(){
		return option;
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

extern Account guest_account;

extern std::vector<Account> accounts;

void LoadAccounts(std::string directorypath);
