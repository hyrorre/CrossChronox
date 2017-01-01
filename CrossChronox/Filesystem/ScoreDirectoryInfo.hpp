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
	std::wstring title;
	
public:
	void DownMusic(){
		++cursor;
	}
	void UpMusic(){
		--cursor;
	}
	std::wstring GetTitleSubtitle() const{
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
	void Clear(){
		children.clear();
	}
	
	ScoreDirectoryInfo(){}
	ScoreDirectoryInfo(fs::wpath path): ScoreInfoBase(path){}
	
//	template<class Archive>
//	void serialize__(Archive& ar, unsigned int ver){
//	}
private: // ここがシリアライズ処理の実装
	BOOST_SERIALIZATION_SPLIT_MEMBER();
	friend class boost::serialization::access;
	template<class Archive>
	void save(Archive& ar, const unsigned int version) const{
		ar & BOOST_SERIALIZATION_BASE_OBJECT_NVP(ScoreInfoBase);
		ar & boost::serialization::make_nvp("children", children);
		ar & boost::serialization::make_nvp("title", title);
	}
	template<class Archive>
	void load(Archive& ar, const unsigned int version){
		ar & BOOST_SERIALIZATION_BASE_OBJECT_NVP(ScoreInfoBase);
		ar & boost::serialization::make_nvp("children", children);
		std::string tmp;
		std::wstring_convert<std::codecvt_utf8<wchar_t>,wchar_t> cv;
		ar & boost::serialization::make_nvp("title", tmp);
		title = cv.from_bytes(tmp);
		for(const auto& child : children){
			child->SetParent(this);
		}
	}
};

//BOOST_CLASS_EXPORT_GUID(ScoreDirectoryInfo, "ScoreDirectoryInfo");
BOOST_CLASS_VERSION(ScoreDirectoryInfo, 1);

#endif /* ScoreDirectoryInfo_hpp */
