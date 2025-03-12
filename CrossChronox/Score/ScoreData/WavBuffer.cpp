#include "WavBuffer.hpp"

bool WavBuffer::Load(const std::string& score_directory) {
    assert(score_directory.back() == '/' || score_directory.back() == '\\');
    fs::path score_path = score_directory + filename;
    if (!fs::exists(score_path)) {
        // windows ignore uppercase <-> lowercase
        static const std::vector<std::string> extentions = {
            "ogg",
            "wav"
#if !defined(_WIN64) && !defined(_WIN32) // if not Windows
            ,
            "Ogg",
            "OGG",
            "Wav",
            "WAV"
#endif
        };
        for (const auto& extention : extentions) {
            score_path.replace_extension(extention);
            if (fs::exists(score_path))
                goto File_Exist;
        }
        return false;
    }
File_Exist:
    //return buf.loadFromFile(score_path.string());
    return false;
}
