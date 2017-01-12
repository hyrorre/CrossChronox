//
//  Total.hpp
//  CrossChronox
//
//  Created by HY_RORRE on 2017/01/11.
//  Copyright © 2017年 hyrorre. All rights reserved.
//

#ifndef Total_hpp
#define Total_hpp

#include "Mode.hpp"

class Total{
	bool type = RELATIVE;
	double value = 100;
public:
	enum : bool{
		RELATIVE = 0,
		ABSOLUTE = 1,
	};
	
	Total(){}
	Total(bool type, double value): type(type), value(value){}
	
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

BOOST_CLASS_VERSION(Total, 1);

#endif /* Total_hpp */
