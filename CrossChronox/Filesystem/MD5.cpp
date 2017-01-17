//
//  MD5.cpp
//  CrossChronox
//
//  Created by HY_RORRE on 2016/10/14.
//  Copyright © 2016年 hyrorre. All rights reserved.
//

#include "MD5.hpp"

bool StringToMD5(const char* str, std::string* out, size_t len){
	
	byte digest[CryptoPP::Weak::MD5::DIGESTSIZE];
	
	CryptoPP::Weak::MD5 hash;
	hash.CalculateDigest(digest, (const byte*)str, len);
	
	CryptoPP::HexEncoder encoder;
	
	encoder.Attach(new CryptoPP::StringSink(*out));
	encoder.Put(digest, sizeof(digest));
	encoder.MessageEnd();
	return true;
}

bool FileToMD5(const std::string& path, std::string* out){
	std::ifstream ifs(path);
	if(!ifs) return false;
	std::istreambuf_iterator<char> it(ifs);
	std::istreambuf_iterator<char> last;
	std::string file_str(it, last);
	
	return StringToMD5(file_str, out);
}


bool StringToMD5(const std::string& str, std::string* out){
	return StringToMD5(str.c_str(), out, str.length());
}

bool StringToMD5(const char* str, std::string* out){
	return StringToMD5(str, out, strlen(str));
}
