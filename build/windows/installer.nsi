; installer.nsi — Yuhina Windows installer (NSIS 3.x)
;
; Build with:
;   makensis /DVERSION=0.1.0 /DAPP_DIR=C:\path\to\stage /DINSTALLER=out.exe installer.nsi
;
; Features: install / uninstall, Start Menu + Desktop shortcuts,
; optional portable mode (no registry entries, no shortcuts).

!include "MUI2.nsh"
!include "LogicLib.nsh"
!include "FileFunc.nsh"
!include "nsDialogs.nsh"
!include "x64.nsh"

Var PortableMode
Var CheckPortable

; ---------------------------------------------------------------------------
; Configuration (overridable via -D)
; ---------------------------------------------------------------------------
!ifndef VERSION
  !define VERSION "0.1.0"
!endif
!ifndef APP_DIR
  !define APP_DIR "..\..\dist\yuhina-${VERSION}-windows-x64"
!endif
!ifndef INSTALLER
  !define INSTALLER "yuhina-${VERSION}-windows-x64-setup.exe"
!endif
!ifndef APP_ICON
  !define APP_ICON "..\..\yuhina\assets\icon.ico"
!endif

!define PRODUCT_NAME "Yuhina"
!define PRODUCT_EXE "yuhina.exe"
!define PRODUCT_UNINST_KEY "Software\Microsoft\Windows\CurrentVersion\Uninstall\${PRODUCT_NAME}"

; ---------------------------------------------------------------------------
; Installer settings
; ---------------------------------------------------------------------------
Name "${PRODUCT_NAME}"
OutFile "${INSTALLER}"
Unicode true
RequestExecutionLevel admin
SetCompressor /SOLID lzma

!ifdef APP_ICON
  !if exists "${APP_ICON}"
    Icon "${APP_ICON}"
    UninstallIcon "${APP_ICON}"
  !endif
!endif

InstallDir "$PROGRAMFILES64\Yuhina"
InstallDirRegKey HKLM "Software\Yuhina" "InstallDir"
ShowInstDetails show
ShowUnInstDetails show

!define MUI_ABORTWARNING

!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_DIRECTORY

; portable mode check page (after directory selection, before install)
Page custom PortablePageCreate PortablePageLeave

!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES
!insertmacro MUI_LANGUAGE "SimpChinese"
!insertmacro MUI_LANGUAGE "English"

Function PortablePageCreate
  nsDialogs::Create 1018
  Pop $0
  ${If} $0 == error
    Abort
  ${EndIf}
  ${NSD_CreateLabel} 0 0 100% 24u "Install as portable mode?"
  Pop $0
  ${NSD_CreateCheckBox} 0 30u 100% 16u "Portable mode (no registry, no shortcuts)"
  Pop $CheckPortable
  ${NSD_SetState} $CheckPortable ${BST_UNCHECKED}
  nsDialogs::Show
FunctionEnd

Function PortablePageLeave
  ${NSD_GetState} $CheckPortable $PortableMode
FunctionEnd

; ---------------------------------------------------------------------------
; Install section
; ---------------------------------------------------------------------------
Section "Install" SecInstall
  SetOutPath "$INSTDIR"
  File /r "${APP_DIR}\*.*"

  ${If} $PortableMode == ${BST_CHECKED}
    Goto done
  ${EndIf}

  ; desktop + start menu shortcuts
  CreateDirectory "$SMPROGRAMS\Yuhina"
  CreateShortCut "$SMPROGRAMS\Yuhina\Yuhina.lnk" "$INSTDIR\${PRODUCT_EXE}"
  CreateShortCut "$DESKTOP\Yuhina.lnk" "$INSTDIR\${PRODUCT_EXE}"

  ; uninstaller + registry
  WriteUninstaller "$INSTDIR\uninstall.exe"
  WriteRegStr HKLM "Software\Yuhina" "InstallDir" "$INSTDIR"
  WriteRegStr HKLM "${PRODUCT_UNINST_KEY}" "DisplayName" "${PRODUCT_NAME}"
  WriteRegStr HKLM "${PRODUCT_UNINST_KEY}" "DisplayVersion" "${VERSION}"
  WriteRegStr HKLM "${PRODUCT_UNINST_KEY}" "DisplayIcon" "$INSTDIR\${PRODUCT_EXE}"
  WriteRegStr HKLM "${PRODUCT_UNINST_KEY}" "UninstallString" "$INSTDIR\uninstall.exe"
  WriteRegDWORD HKLM "${PRODUCT_UNINST_KEY}" "NoModify" 1
  WriteRegDWORD HKLM "${PRODUCT_UNINST_KEY}" "NoRepair" 1

done:
SectionEnd

; ---------------------------------------------------------------------------
; Uninstall section
; ---------------------------------------------------------------------------
Section "Uninstall" SecUninstall
  Delete "$INSTDIR\uninstall.exe"
  RMDir /r "$INSTDIR"
  Delete "$SMPROGRAMS\Yuhina\Yuhina.lnk"
  RMDir "$SMPROGRAMS\Yuhina"
  Delete "$DESKTOP\Yuhina.lnk"
  DeleteRegKey HKLM "${PRODUCT_UNINST_KEY}"
  DeleteRegKey HKLM "Software\Yuhina"
SectionEnd