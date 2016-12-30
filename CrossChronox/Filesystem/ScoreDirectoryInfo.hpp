//
//  ScoreDirectoryInfo.hpp
//  CrossChronox
//
//  Created by HY_RORRE on 2016/12/22.
//  Copyright © 2016年 hyrorre. All rights reserved.
//

#ifndef ScoreDirectoryInfo_hpp
#define ScoreDirectoryInfo_hpp

#include "pch.hpp"
#include "ScoreInfoBase.hpp"

class ScoreDirectoryLoader;

class ScoreDirectoryInfo : public ScoreInfoBase{
	friend ScoreDirectoryLoader;
	
	ScoreDirectoryInfo* parent = nullptr;
	std::vector<std::unique_ptr<ScoreInfoBase>> children;
	int cursor = 0;
	std::string title;
	
public:
	void MusicDown(){
		++cursor;
	}
	void MusicUp(){
		--cursor;
	}
	std::string GetTitleSubtitle() const{
		return title;
	}
	ScoreDirectoryInfo* GetParent(){
		return parent;
	}
	void SetParent(ScoreDirectoryInfo* parent){
		this->parent = parent;
	}
	const ScoreInfoBase* At(int index) const{
		int true_index = (cursor + index);
		for(; true_index < 0; true_index += children.size() * 10);
		true_index %= children.size();
		return children[true_index].get();
	}
	ScoreInfoBase* At(int index){
		return const_cast<ScoreInfoBase*>(static_cast<const ScoreDirectoryInfo*>(this)->At(index));
	}
	bool Empty(){
		return children.empty();
	}
	
	ScoreDirectoryInfo(){}
	ScoreDirectoryInfo(fs::path path): ScoreInfoBase(path){}
	
//	template<class Archive>
//	void serialize__(Archive& ar, unsigned int ver){
//	}
private: // ここがシリアライズ処理の実装
	friend class boost::serialization::access;
	template<class Archive>
	void serialize(Archive& ar, unsigned int ver){
		ar & BOOST_SERIALIZATION_BASE_OBJECT_NVP(ScoreInfoBase);
		ar & boost::serialization::make_nvp("children", children);
		ar & boost::serialization::make_nvp("title", title);
		for(const auto& child : children){
			child->SetParent(this);
		}
	}
};

//BOOST_CLASS_EXPORT_GUID(ScoreDirectoryInfo, "ScoreDirectoryInfo");
BOOST_CLASS_VERSION(ScoreDirectoryInfo, 1);

#endif /* ScoreDirectoryInfo_hpp */
