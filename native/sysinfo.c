#ifdef _WIN32
#include <windows.h>
#include <stdint.h>
uint32_t ua_windows_build(void) { return (uint32_t)GetVersion(); }
int ua_path_is_file(const wchar_t *path) {
 DWORD a=GetFileAttributesW(path);
 return a!=INVALID_FILE_ATTRIBUTES && !(a & FILE_ATTRIBUTE_DIRECTORY);
}
#else
#include <stdint.h>
uint32_t ua_windows_build(void) { return 0; }
int ua_path_is_file(const wchar_t *path) { (void)path; return 0; }
#endif
