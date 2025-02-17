#ifndef JudgeRank_hpp
#define JudgeRank_hpp

#include "pch.hpp"
#include "Mode.hpp"

enum Judge{
	LN_PUSHING_PGREAT = -10,
	LN_PUSHING_GREAT = -9,
	LN_PUSHING_GOOD = -8,
	JUDGE_YET = -1,
	PGREAT = 0,
	GREAT = 1,
	GOOD = 2,
	BAD = 3,
	POOR = 4,
	
	MAX_JUDGE = 5
};

enum : bool{
	RANK_RELATIVE = 0,
	RANK_ABSOLUTE = 1
};

enum{
	RANK_VHARD = 0,
	RANK_HARD,
	RANK_NORMAL,
	RANK_EASY,
	RANK_VEASY,
	
	MAX_RANK,
};
	
class JudgeRank{
	bool type = RANK_RELATIVE;
	double value = 100;

public:
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
	
//private:
//	friend class boost::serialization::access;
//	template<class Archive>
//	void serialize(Archive& ar, const unsigned int version){
//		ar & BOOST_SERIALIZATION_NVP(type);
//		ar & BOOST_SERIALIZATION_NVP(value);
//	}
};

//BOOST_CLASS_VERSION(JudgeRank, 1);

#endif /* JudgeRank_hpp */
