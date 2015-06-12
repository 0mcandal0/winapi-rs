// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to dbghelp.
#![cfg(windows)]
extern crate winapi;
use winapi::*;
extern "system" {
    // pub fn DbgHelpCreateUserDump();
    // pub fn DbgHelpCreateUserDumpW();
    // pub fn EnumDirTree();
    // pub fn EnumDirTreeW();
    // pub fn EnumerateLoadedModules();
    // pub fn EnumerateLoadedModules64();
    // pub fn EnumerateLoadedModulesEx();
    // pub fn EnumerateLoadedModulesExW();
    // pub fn EnumerateLoadedModulesW64();
    // pub fn ExtensionApiVersion();
    // pub fn FindDebugInfoFile();
    // pub fn FindDebugInfoFileEx();
    // pub fn FindDebugInfoFileExW();
    // pub fn FindExecutableImage();
    // pub fn FindExecutableImageEx();
    // pub fn FindExecutableImageExW();
    // pub fn FindFileInPath();
    // pub fn FindFileInSearchPath();
    // pub fn GetSymLoadError();
    // pub fn GetTimestampForLoadedLibrary();
    // pub fn ImageDirectoryEntryToData();
    // pub fn ImageDirectoryEntryToDataEx();
    // pub fn ImageNtHeader();
    // pub fn ImageRvaToSection();
    // pub fn ImageRvaToVa();
    // pub fn ImagehlpApiVersion();
    // pub fn ImagehlpApiVersionEx();
    // pub fn MakeSureDirectoryPathExists();
    // pub fn MiniDumpReadDumpStream();
    // pub fn MiniDumpWriteDump();
    // pub fn RangeMapAddPeImageSections();
    // pub fn RangeMapCreate();
    // pub fn RangeMapFree();
    // pub fn RangeMapRead();
    // pub fn RangeMapRemove();
    // pub fn RangeMapWrite();
    // pub fn RemoveInvalidModuleList();
    // pub fn ReportSymbolLoadSummary();
    // pub fn SearchTreeForFile();
    // pub fn SearchTreeForFileW();
    // pub fn SetCheckUserInterruptShared();
    // pub fn SetSymLoadError();
    // pub fn StackWalk();
    pub fn StackWalk64(MachineType: DWORD,
                       hProcess: HANDLE,
                       hThread: HANDLE,
                       StackFrame: LPSTACKFRAME64,
                       ContextRecord: PVOID,
                       ReadMemoryRoutine: PREAD_PROCESS_MEMORY_ROUTINE64,
                       FunctionTableAccessRoutine: PFUNCTION_TABLE_ACCESS_ROUTINE64,
                       GetModuleBaseRoutine: PGET_MODULE_BASE_ROUTINE64,
                       TranslateAddress: PTRANSLATE_ADDRESS_ROUTINE64) -> BOOL;
    // pub fn StackWalkEx();
    // pub fn SymAddSourceStream();
    // pub fn SymAddSourceStreamA();
    // pub fn SymAddSourceStreamW();
    // pub fn SymAddSymbol();
    // pub fn SymAddSymbolW();
    // pub fn SymAddrIncludeInlineTrace();
    pub fn SymCleanup(hProcess: HANDLE) -> BOOL;
    // pub fn SymCompareInlineTrace();
    // pub fn SymDeleteSymbol();
    // pub fn SymDeleteSymbolW();
    // pub fn SymEnumLines();
    // pub fn SymEnumLinesW();
    // pub fn SymEnumProcesses();
    // pub fn SymEnumSourceFileTokens();
    // pub fn SymEnumSourceFiles();
    // pub fn SymEnumSourceFilesW();
    // pub fn SymEnumSourceLines();
    // pub fn SymEnumSourceLinesW();
    // pub fn SymEnumSym();
    // pub fn SymEnumSymbols();
    // pub fn SymEnumSymbolsEx();
    // pub fn SymEnumSymbolsExW();
    // pub fn SymEnumSymbolsForAddr();
    // pub fn SymEnumSymbolsForAddrW();
    // pub fn SymEnumSymbolsW();
    // pub fn SymEnumTypes();
    // pub fn SymEnumTypesByName();
    // pub fn SymEnumTypesByNameW();
    // pub fn SymEnumTypesW();
    // pub fn SymEnumerateModules();
    // pub fn SymEnumerateModules64();
    // pub fn SymEnumerateModulesW64();
    // pub fn SymEnumerateSymbols();
    // pub fn SymEnumerateSymbols64();
    // pub fn SymEnumerateSymbolsW();
    // pub fn SymEnumerateSymbolsW64();
    // pub fn SymFindDebugInfoFile();
    // pub fn SymFindDebugInfoFileW();
    // pub fn SymFindExecutableImage();
    // pub fn SymFindExecutableImageW();
    // pub fn SymFindFileInPath();
    // pub fn SymFindFileInPathW();
    // pub fn SymFromAddr();
    pub fn SymFromAddrW(hProcess: HANDLE,
                        Address: DWORD64,
                        Displacement: PDWORD64,
                        Symbol: PSYMBOL_INFOW) -> BOOL;
    // pub fn SymFromIndex();
    // pub fn SymFromIndexW();
    // pub fn SymFromInlineContext();
    // pub fn SymFromInlineContextW();
    // pub fn SymFromName();
    // pub fn SymFromNameW();
    // pub fn SymFromToken();
    // pub fn SymFromTokenW();
    // pub fn SymFunctionTableAccess();
    pub fn SymFunctionTableAccess64(hProcess: HANDLE, AddrBase: DWORD64) -> PVOID;
    // pub fn SymFunctionTableAccess64AccessRoutines();
    // pub fn SymGetFileLineOffsets64();
    // pub fn SymGetHomeDirectory();
    // pub fn SymGetHomeDirectoryW();
    // pub fn SymGetLineFromAddr();
    // pub fn SymGetLineFromAddr64();
    // pub fn SymGetLineFromAddrW64();
    pub fn SymGetLineFromAddrW64(hProcess: HANDLE,
                                 dwAddr: DWORD64,
                                 pdwDisplacement: PDWORD,
                                 Line: PIMAGEHLP_LINEW64) -> BOOL;
    // pub fn SymGetLineFromInlineContext();
    // pub fn SymGetLineFromInlineContextW();
    // pub fn SymGetLineFromName();
    // pub fn SymGetLineFromName64();
    // pub fn SymGetLineFromNameW64();
    // pub fn SymGetLineNext();
    // pub fn SymGetLineNext64();
    // pub fn SymGetLineNextW64();
    // pub fn SymGetLinePrev();
    // pub fn SymGetLinePrev64();
    // pub fn SymGetLinePrevW64();
    pub fn SymGetModuleBase64(hProcess: HANDLE, AddrBase: DWORD64) -> DWORD64;
    // pub fn SymGetModuleBase64();
    // pub fn SymGetModuleInfo();
    // pub fn SymGetModuleInfo64();
    // pub fn SymGetModuleInfoW();
    // pub fn SymGetModuleInfoW64();
    // pub fn SymGetOmaps();
    // pub fn SymGetOptions();
    // pub fn SymGetScope();
    // pub fn SymGetScopeW();
    // pub fn SymGetSearchPath();
    // pub fn SymGetSearchPathW();
    // pub fn SymGetSourceFile();
    // pub fn SymGetSourceFileFromToken();
    // pub fn SymGetSourceFileFromTokenW();
    // pub fn SymGetSourceFileToken();
    // pub fn SymGetSourceFileTokenW();
    // pub fn SymGetSourceFileW();
    // pub fn SymGetSourceVarFromToken();
    // pub fn SymGetSourceVarFromTokenW();
    // pub fn SymGetSymFromAddr();
    pub fn SymGetSymFromAddr64(hProcess: HANDLE,
                               Address: DWORD64,
                               Displacement: PDWORD64,
                               Symbol: PIMAGEHLP_SYMBOL64) -> BOOL;
    // pub fn SymGetSymFromName();
    // pub fn SymGetSymFromName64();
    // pub fn SymGetSymNext();
    // pub fn SymGetSymNext64();
    // pub fn SymGetSymPrev();
    // pub fn SymGetSymPrev64();
    // pub fn SymGetSymbolFile();
    // pub fn SymGetSymbolFileW();
    // pub fn SymGetTypeFromName();
    // pub fn SymGetTypeFromNameW();
    // pub fn SymGetTypeInfo();
    // pub fn SymGetTypeInfoEx();
    // pub fn SymGetUnwindInfo();
    // pub fn SymInitialize();
    pub fn SymInitializeW(hProcess: HANDLE,
                          UserSearchPath: PCWSTR,
                          fInvadeProcess: BOOL) -> BOOL;
    // pub fn SymLoadModule();
    // pub fn SymLoadModule64();
    // pub fn SymLoadModuleEx();
    // pub fn SymLoadModuleExW();
    // pub fn SymMatchFileName();
    // pub fn SymMatchFileNameW();
    // pub fn SymMatchString();
    // pub fn SymMatchStringA();
    // pub fn SymMatchStringW();
    // pub fn SymNext();
    // pub fn SymNextW();
    // pub fn SymPrev();
    // pub fn SymPrevW();
    // pub fn SymQueryInlineTrace();
    // pub fn SymRefreshModuleList();
    // pub fn SymRegisterCallback();
    // pub fn SymRegisterCallback64();
    // pub fn SymRegisterCallbackW64();
    // pub fn SymRegisterFunctionEntryCallback();
    // pub fn SymRegisterFunctionEntryCallback64();
    // pub fn SymSearch();
    // pub fn SymSearchW();
    // pub fn SymSetContext();
    // pub fn SymSetHomeDirectory();
    // pub fn SymSetHomeDirectoryW();
    // pub fn SymSetOptions();
    // pub fn SymSetParentWindow();
    // pub fn SymSetScopeFromAddr();
    // pub fn SymSetScopeFromIndex();
    // pub fn SymSetScopeFromInlineContext();
    // pub fn SymSetSearchPath();
    // pub fn SymSetSearchPathW();
    // pub fn SymSrvDeltaName();
    // pub fn SymSrvDeltaNameW();
    // pub fn SymSrvGetFileIndexInfo();
    // pub fn SymSrvGetFileIndexInfoW();
    // pub fn SymSrvGetFileIndexString();
    // pub fn SymSrvGetFileIndexStringW();
    // pub fn SymSrvGetFileIndexes();
    // pub fn SymSrvGetFileIndexesW();
    // pub fn SymSrvGetSupplement();
    // pub fn SymSrvGetSupplementW();
    // pub fn SymSrvIsStore();
    // pub fn SymSrvIsStoreW();
    // pub fn SymSrvStoreFile();
    // pub fn SymSrvStoreFileW();
    // pub fn SymSrvStoreSupplement();
    // pub fn SymSrvStoreSupplementW();
    // pub fn SymUnDName();
    // pub fn SymUnDName64();
    // pub fn SymUnloadModule();
    // pub fn SymUnloadModule64();
    // pub fn UnDecorateSymbolName();
    // pub fn UnDecorateSymbolNameW();
    // pub fn WinDbgExtensionDllInit();
    // pub fn block();
    // pub fn chksym();
    // pub fn dbghelp();
    // pub fn dh();
    // pub fn fptr();
    // pub fn homedir();
    // pub fn inlinedbg();
    // pub fn itoldyouso();
    // pub fn lmi();
    // pub fn lminfo();
    // pub fn omap();
    // pub fn optdbgdump();
    // pub fn optdbgdumpaddr();
    // pub fn srcfiles();
    // pub fn stack_force_ebp();
    // pub fn stackdbg();
    // pub fn sym();
    // pub fn symsrv();
    // pub fn vc7fpo();
}
#[cfg(any(target_arch = "x86", target_arch = "arm"))]
extern "system" {
    // pub fn MapDebugInformation();
    // pub fn UnmapDebugInformation();
}
