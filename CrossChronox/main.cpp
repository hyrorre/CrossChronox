#include "Application.hpp"
#include "Filesystem/Path.hpp"

#if !defined(_WIN64) && !defined(_WIN32) // if not Windows
int main(int argc, char* argv[]) {
#else // if Windows
#include <windows.h>
int WINAPI WinMain(_In_ HINSTANCE hInstance, _In_opt_ HINSTANCE hPrevInstance, _In_ LPSTR lpCmdLine, _In_ int nShowCmd) {
    int argc = __argc;
    char** argv = __argv;
#endif
    Application app(argc, argv);
    try {
        app.Init();
        return app.Run();
    } catch (std::exception& e) {
        app.HandleException(e);
        return EXIT_FAILURE;
    }
}
