@ECHO OFF

GOTO start

:start

IF NOT [%1] == [] CALL SET build_type=%1
IF [%build_type%] == [] CALL SET build_type=release

IF [%build_type%] == [debug] GOTO valid_args
IF [%build_type%] == [release] GOTO valid_args

ECHO Usage: build [release^|debug]
ECHO.
ECHO Build defaults to 'release'
EXIT /B

:valid_args

SET build_arch=windows-x%PROCESSOR_ARCHITECTURE:~-2%
SET build_dir=.\build\%build_type%\%build_arch%

ECHO Building derby-stats in %build_type% configuration for %build_arch%

CALL:get_time
SET /A start_time = %ERRORLEVEL%

CD src\ui
RMDIR /S /Q .\dist
CALL npm ci
CALL npm run build
CD ..\..

CALL cargo build --%build_type%

RMDIR /S /Q %build_dir%
MKDIR %build_dir%\ui

XCOPY .\target\%build_type%\derby-stats.exe %build_dir%
XCOPY .\src\ui\dist\ %build_dir%\ui\ /E

CALL:get_time
SET /A end_time = %ERRORLEVEL%

SET /A elapsed = (%end_time% - %start_time%)
SET /A elapsed_seconds = %elapsed% / 100
SET /A elapsed_milliseconds = %elapsed% %% 100
ECHO Finished in %elapsed_seconds%.%elapsed_milliseconds% seconds

EXIT /B

:get_time
FOR /F "tokens=1-4 delims=:.," %%a in ("%time%") DO (
    SET /A "calculated_time=(((%%a*60)+1%%b %% 100)*60+1%%c %% 100)*100+1%%d %% 100"
)
EXIT /B %calculated_time%
