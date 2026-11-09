#define UNICODE
#define _UNICODE
#include <windows.h>
#include <shlobj.h>
#include <shobjidl.h>
#include <shellapi.h>
#include <string>
#include <vector>
#include <filesystem>

static HINSTANCE g_hInst;
static long g_refs=0;
static const CLSID CLSID_UltraArchiver={0x8a72bfa1,0x3a2d,0x4d21,{0x91,0x17,0x4a,0x20,0x71,0x33,0x2c,0x90}};
static std::wstring module_dir(){wchar_t p[MAX_PATH]{};GetModuleFileNameW(g_hInst,p,MAX_PATH);return std::filesystem::path(p).parent_path().wstring();}
static std::wstring quote(const std::wstring&s){return L"\""+s+L"\"";}
class Menu final:public IShellExtInit,public IContextMenu{
 ULONG refs=1; std::vector<std::wstring> paths;
 public:
 HRESULT STDMETHODCALLTYPE QueryInterface(REFIID riid,void**ppv)override{if(!ppv)return E_POINTER;*ppv=nullptr;if(riid==IID_IUnknown||riid==IID_IShellExtInit)*ppv=static_cast<IShellExtInit*>(this);else if(riid==IID_IContextMenu)*ppv=static_cast<IContextMenu*>(this);else return E_NOINTERFACE;AddRef();return S_OK;}
 ULONG STDMETHODCALLTYPE AddRef()override{return InterlockedIncrement((volatile LONG*)&refs);}
 ULONG STDMETHODCALLTYPE Release()override{ULONG n=InterlockedDecrement((volatile LONG*)&refs);if(!n)delete this;return n;}
 HRESULT STDMETHODCALLTYPE Initialize(PCIDLIST_ABSOLUTE,IDataObject*data,HKEY)override{
  paths.clear();if(!data)return E_INVALIDARG;FORMATETC f={CF_HDROP,nullptr,DVASPECT_CONTENT,-1,TYMED_HGLOBAL};STGMEDIUM m{};
  if(FAILED(data->GetData(&f,&m)))return E_INVALIDARG;HDROP h=(HDROP)m.hGlobal;UINT n=DragQueryFileW(h,0xffffffff,nullptr,0);
  for(UINT i=0;i<n;i++){UINT len=DragQueryFileW(h,i,nullptr,0);std::wstring p(len+1,L'\0');DragQueryFileW(h,i,p.data(),len+1);p.resize(len);paths.push_back(p);}ReleaseStgMedium(&m);return paths.empty()?E_INVALIDARG:S_OK;
 }
 HRESULT STDMETHODCALLTYPE QueryContextMenu(HMENU menu,UINT index,UINT first,UINT,UINT flags)override{
  if(flags&CMF_DEFAULTONLY)return MAKE_HRESULT(SEVERITY_SUCCESS,0,0);
  InsertMenuW(menu,index++,MF_BYPOSITION|MF_STRING,first,L"Додати до архіву…");
  InsertMenuW(menu,index++,MF_BYPOSITION|MF_STRING,first+1,L"Додати до ZIP");
  InsertMenuW(menu,index++,MF_BYPOSITION|MF_STRING,first+2,L"Розпакувати тут");
  InsertMenuW(menu,index,MF_BYPOSITION|MF_STRING,first+3,L"Розпакувати в окрему папку");
  return MAKE_HRESULT(SEVERITY_SUCCESS,0,4);
 }
 HRESULT STDMETHODCALLTYPE InvokeCommand(LPCMINVOKECOMMANDINFO info)override{
  if(HIWORD(info->lpVerb))return E_INVALIDARG;int id=LOWORD(info->lpVerb);if(paths.empty())return E_FAIL;
  std::wstring exe=module_dir()+L"\\UltraArchiver.exe";
  if(id==0){std::wstring args=L"\""+exe+L"\" --add";for(auto&p:paths)args+=L" "+quote(p);ShellExecuteW(nullptr,L"open",exe.c_str(),args.c_str(),module_dir().c_str(),SW_SHOWNORMAL);return S_OK;}
  std::wstring seven=module_dir()+L"\\7z.exe";
  for(auto&p:paths){
   std::filesystem::path src(p);std::wstring parent=src.parent_path().wstring();std::wstring base=src.filename().wstring();
   std::wstring cmd=L"\""+seven+L"\" ";
   if(id==1){cmd+=L"a -tzip "+quote(p+L".zip")+L" "+quote(p);}
   else if(id==2){cmd+=L"x "+quote(p)+L" -o"+quote(parent)+L" -y";}
   else {std::wstring dest=p+L".extracted";cmd+=L"x "+quote(p)+L" -o"+quote(dest)+L" -y";}
   STARTUPINFOW si{sizeof(si)};PROCESS_INFORMATION pi{};std::vector<wchar_t>b(cmd.begin(),cmd.end());b.push_back(0);
   if(CreateProcessW(nullptr,b.data(),nullptr,nullptr,FALSE,CREATE_NO_WINDOW,nullptr,parent.c_str(),&si,&pi)){CloseHandle(pi.hThread);CloseHandle(pi.hProcess);}
  }return S_OK;
 }
 HRESULT STDMETHODCALLTYPE GetCommandString(UINT_PTR,UINT,UINT*,LPSTR,UINT)override{return E_NOTIMPL;}
};
class Factory final:public IClassFactory{
 ULONG refs=1;
 public:
 HRESULT STDMETHODCALLTYPE QueryInterface(REFIID r,void**p)override{if(!p)return E_POINTER;*p=nullptr;if(r==IID_IUnknown||r==IID_IClassFactory)*p=this;else return E_NOINTERFACE;AddRef();return S_OK;}
 ULONG STDMETHODCALLTYPE AddRef()override{return InterlockedIncrement((volatile LONG*)&refs);}
 ULONG STDMETHODCALLTYPE Release()override{ULONG n=InterlockedDecrement((volatile LONG*)&refs);if(!n)delete this;return n;}
 HRESULT STDMETHODCALLTYPE CreateInstance(IUnknown*outer,REFIID r,void**p)override{if(outer)return CLASS_E_NOAGGREGATION;auto*m=new(std::nothrow)Menu();if(!m)return E_OUTOFMEMORY;HRESULT h=m->QueryInterface(r,p);m->Release();return h;}
 HRESULT STDMETHODCALLTYPE LockServer(BOOL lock)override{if(lock)InterlockedIncrement(&g_refs);else InterlockedDecrement(&g_refs);return S_OK;}
};
static HRESULT setreg(HKEY root,const std::wstring&key,const std::wstring&name,const std::wstring&value){HKEY h; if(RegCreateKeyExW(root,key.c_str(),0,nullptr,0,KEY_WRITE,nullptr,&h,nullptr)!=ERROR_SUCCESS)return E_FAIL;LONG e=RegSetValueExW(h,name.empty()?nullptr:name.c_str(),0,REG_SZ,(const BYTE*)value.c_str(),(DWORD)((value.size()+1)*sizeof(wchar_t)));RegCloseKey(h);return e==ERROR_SUCCESS?S_OK:E_FAIL;}
STDAPI DllCanUnloadNow(){return g_refs==0?S_OK:S_FALSE;}
STDAPI DllGetClassObject(REFCLSID c,REFIID r,void**p){if(c!=CLSID_UltraArchiver)return CLASS_E_CLASSNOTAVAILABLE;auto*f=new(std::nothrow)Factory();if(!f)return E_OUTOFMEMORY;HRESULT h=f->QueryInterface(r,p);f->Release();return h;}
STDAPI DllRegisterServer(){
 wchar_t cls[64];StringFromGUID2(CLSID_UltraArchiver,cls,64);std::wstring id=cls,base=L"Software\\Classes\\CLSID\\"+id;
 if(FAILED(setreg(HKEY_CURRENT_USER,base,L"",L"UltraArchiver Context Menu")))return E_FAIL;
 if(FAILED(setreg(HKEY_CURRENT_USER,base+L"\\InprocServer32",L"",module_dir()+L"\\UltraArchiverShell.dll")))return E_FAIL;
 setreg(HKEY_CURRENT_USER,base+L"\\InprocServer32",L"ThreadingModel",L"Apartment");
 for(auto key:{L"*\\shellex\\ContextMenuHandlers\\UltraArchiver",L"Directory\\shellex\\ContextMenuHandlers\\UltraArchiver",L"Folder\\shellex\\ContextMenuHandlers\\UltraArchiver"})setreg(HKEY_CURRENT_USER,std::wstring(L"Software\\Classes\\")+key,L"",id);
 return S_OK;
}
STDAPI DllUnregisterServer(){
 wchar_t cls[64];StringFromGUID2(CLSID_UltraArchiver,cls,64);std::wstring id=cls;
 for(auto key:{L"*\\shellex\\ContextMenuHandlers\\UltraArchiver",L"Directory\\shellex\\ContextMenuHandlers\\UltraArchiver",L"Folder\\shellex\\ContextMenuHandlers\\UltraArchiver"})RegDeleteTreeW(HKEY_CURRENT_USER,(std::wstring(L"Software\\Classes\\")+key).c_str());
 RegDeleteTreeW(HKEY_CURRENT_USER,(L"Software\\Classes\\CLSID\\"+id).c_str());return S_OK;
}
BOOL WINAPI DllMain(HINSTANCE h,DWORD reason,LPVOID){if(reason==DLL_PROCESS_ATTACH){g_hInst=h;DisableThreadLibraryCalls(h);}return TRUE;}
