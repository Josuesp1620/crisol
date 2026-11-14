; Crisol installer for Windows. Build with:
;   makensis /DVERSION=0.2.0 /DSOURCE=<folder with crisol.exe> /DOUTPUT=<setup.exe> crisol.nsi

Unicode true
!include "MUI2.nsh"

!ifndef VERSION
  !define VERSION "0.0.0"
!endif
!ifndef SOURCE
  !define SOURCE "..\target\release"
!endif
!ifndef OUTPUT
  !define OUTPUT "crisol-setup.exe"
!endif

!define APP_NAME "Crisol"
!define APP_EXE "crisol.exe"
!define PUBLISHER "Josuesp1620"
!define UNINSTALL_KEY "Software\Microsoft\Windows\CurrentVersion\Uninstall\Crisol"

Name "${APP_NAME} ${VERSION}"
OutFile "${OUTPUT}"
InstallDir "$PROGRAMFILES64\${APP_NAME}"
InstallDirRegKey HKLM "Software\${APP_NAME}" "InstallDir"
RequestExecutionLevel admin
SetCompressor /SOLID lzma

!define MUI_ICON "crisol.ico"
!define MUI_UNICON "crisol.ico"
!define MUI_ABORTWARNING
!define MUI_FINISHPAGE_RUN "$INSTDIR\${APP_EXE}"
!define MUI_FINISHPAGE_RUN_TEXT "Start Crisol"

!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_LICENSE "..\LICENSE"
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH
!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES
!insertmacro MUI_LANGUAGE "English"

Section "Crisol" SectionMain
  SectionIn RO
  SetOutPath "$INSTDIR"
  File "${SOURCE}\${APP_EXE}"
  File "${SOURCE}\crisol-cli.exe"
  File "crisol.ico"
  File "..\LICENSE"
  File "..\README.md"
  File "..\CHANGELOG.md"
  SetOutPath "$INSTDIR\examples"
  File "..\examples\customer\customer.crisol"
  File "${SOURCE}\customer-form.exe"

  WriteRegStr HKLM "Software\${APP_NAME}" "InstallDir" "$INSTDIR"
  WriteUninstaller "$INSTDIR\uninstall.exe"

  CreateDirectory "$SMPROGRAMS\${APP_NAME}"
  CreateShortcut "$SMPROGRAMS\${APP_NAME}\${APP_NAME}.lnk" "$INSTDIR\${APP_EXE}" "" "$INSTDIR\crisol.ico"
  CreateShortcut "$SMPROGRAMS\${APP_NAME}\Uninstall ${APP_NAME}.lnk" "$INSTDIR\uninstall.exe"
  CreateShortcut "$DESKTOP\${APP_NAME}.lnk" "$INSTDIR\${APP_EXE}" "" "$INSTDIR\crisol.ico"

  ; Double-clicking a form or a project opens it in Crisol.
  WriteRegStr HKCR ".crisol" "" "Crisol.Form"
  WriteRegStr HKCR "Crisol.Form" "" "Crisol form"
  WriteRegStr HKCR "Crisol.Form\DefaultIcon" "" "$INSTDIR\crisol.ico"
  WriteRegStr HKCR "Crisol.Form\shell\open\command" "" '"$INSTDIR\${APP_EXE}" "%1"'
  WriteRegStr HKCR ".crisolproj" "" "Crisol.Project"
  WriteRegStr HKCR "Crisol.Project" "" "Crisol project"
  WriteRegStr HKCR "Crisol.Project\DefaultIcon" "" "$INSTDIR\crisol.ico"
  WriteRegStr HKCR "Crisol.Project\shell\open\command" "" '"$INSTDIR\${APP_EXE}" "%1"'

  WriteRegStr HKLM "${UNINSTALL_KEY}" "DisplayName" "${APP_NAME}"
  WriteRegStr HKLM "${UNINSTALL_KEY}" "DisplayVersion" "${VERSION}"
  WriteRegStr HKLM "${UNINSTALL_KEY}" "Publisher" "${PUBLISHER}"
  WriteRegStr HKLM "${UNINSTALL_KEY}" "DisplayIcon" "$INSTDIR\crisol.ico"
  WriteRegStr HKLM "${UNINSTALL_KEY}" "InstallLocation" "$INSTDIR"
  WriteRegStr HKLM "${UNINSTALL_KEY}" "UninstallString" '"$INSTDIR\uninstall.exe"'
  WriteRegDWORD HKLM "${UNINSTALL_KEY}" "NoModify" 1
  WriteRegDWORD HKLM "${UNINSTALL_KEY}" "NoRepair" 1
SectionEnd

Section "Uninstall"
  Delete "$INSTDIR\${APP_EXE}"
  Delete "$INSTDIR\crisol-cli.exe"
  Delete "$INSTDIR\crisol.ico"
  Delete "$INSTDIR\LICENSE"
  Delete "$INSTDIR\README.md"
  Delete "$INSTDIR\CHANGELOG.md"
  Delete "$INSTDIR\examples\customer.crisol"
  Delete "$INSTDIR\examples\customer-form.exe"
  RMDir "$INSTDIR\examples"
  Delete "$INSTDIR\uninstall.exe"
  RMDir "$INSTDIR"

  Delete "$SMPROGRAMS\${APP_NAME}\${APP_NAME}.lnk"
  Delete "$SMPROGRAMS\${APP_NAME}\Uninstall ${APP_NAME}.lnk"
  RMDir "$SMPROGRAMS\${APP_NAME}"
  Delete "$DESKTOP\${APP_NAME}.lnk"

  DeleteRegKey HKCR ".crisol"
  DeleteRegKey HKCR "Crisol.Form"
  DeleteRegKey HKCR ".crisolproj"
  DeleteRegKey HKCR "Crisol.Project"
  DeleteRegKey HKLM "${UNINSTALL_KEY}"
  DeleteRegKey HKLM "Software\${APP_NAME}"
SectionEnd
