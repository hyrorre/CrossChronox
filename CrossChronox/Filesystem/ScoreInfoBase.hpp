//
//  ScoreInfoBase.hpp
//  CrossChronox
//
//  Created by HY_RORRE on 2016/12/22.
//  Copyright © 2016年 hyrorre. All rights reserved.
//

#ifndef ScoreInfoBase_hpp
#define ScoreInfoBase_hpp

#include "pch.hpp"

class ScoreInfoBase;
class ScoreDirectoryInfo;

//template<class Archive>
//void serialize(ScoreInfoBase* ptr, Archive& ar, unsigned int ver);

class ScoreInfoBase{
protected:
public:
	virtual std::wstring GetTitleSubtitle() const = 0;
	virtual std::wstring GetInfoStr() const = 0;
	virtual void SetParent(ScoreDirectoryInfo* parent){}
	fs::path path;
	
	ScoreInfoBase(){}
	ScoreInfoBase(fs::wpath path): path(path){}
	virtual ~ScoreInfoBase(){}
	
private: // ここがシリアライズ処理の実装
	friend class boost::serialization::access;
	template<class Archive>
	void serialize(Archive& ar, unsigned int ver){
		std::string p = path.string();
		ar & boost::serialization::make_nvp("path", p);
		path.clear();
		path = p;
	}
};

//BOOST_CLASS_EXPORT(ScoreInfoBase);
BOOST_CLASS_VERSION(ScoreInfoBase, 1);

#endif /* ScoreInfoBase_hpp */
