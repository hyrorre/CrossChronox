//
//  JudgeRank.hpp
//  CrossChronox
//
//  Created by HY_RORRE on 2017/01/11.
//  Copyright © 2017年 hyrorre. All rights reserved.
//

#ifndef JudgeRank_hpp
#define JudgeRank_hpp

#include "pch.hpp"
#include "Mode.hpp"

enum Judge{
	JUDGE_YET = -1,
	PGREAT = 0,
	GREAT,
	GOOD,
	BAD,
	POOR,
	
	MAX_JUDGE
};

class JudgeRank{
	bool type = RANK_RELATIVE;
	double value = 100;
public:
	enum : bool{
		RANK_RELATIVE = 0,
		RANK_ABSOLUTE = 1
	};
	
	enum{
		VHARD = 0,
		HARD,
		NORMAL,
		EASY,
		VEASY,
		
		MAX_RANK,
	};
	
	JudgeRank(){}
	JudgeRank(bool type, double value): type(type), value(value){}
	
	bool GetType() const{
		return type;
	}
	void SetType(bool type){
		this->type = type;
	}
	double GetValue() const{
		return value;
	}
	void SetValue(double value){
		this->value = value;
	}
	
private:
	friend class boost::serialization::access;
	template<class Archive>
	void serialize(Archive& ar, const unsigned int version){
		ar & BOOST_SERIALIZATION_NVP(type);
		ar & BOOST_SERIALIZATION_NVP(value);
	}
};

BOOST_CLASS_VERSION(JudgeRank, 1);

#endif /* JudgeRank_hpp */
