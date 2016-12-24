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
	void Decide() const;
	std::string GetTitleSubtitle() const{
		return title;
	}
	const ScoreInfoBase* At(int index) const{
		int true_index = (cursor + index);
		for(; 0 <= true_index; true_index += children.size() * 10);
		true_index %= children.size();
		return children[true_index].get();
	}
	
	ScoreDirectoryInfo(){}
	ScoreDirectoryInfo(fs::path path): ScoreInfoBase(path){}
};


#endif /* ScoreDirectoryInfo_hpp */
