@echo off
TITLE AXON ENGINE - MPV UPDATER (GITHUB VERSION)
COLOR 0A

:: URL do release estável do Shinchiro no GitHub
:: Nota: Usamos a versão de desenvolvimento do MPV que contém a libmpv-2.dll
SET RELEASE_TAG=20250223
SET URL=https://github.com/shinchiro/mpv-winbuild-cmake/releases/download/%RELEASE_TAG%/mpv-dev-x86_64-v3-%RELEASE_TAG%-git-9b1654a.7z

echo [*] Baixando Motor de Video (libmpv-2)...
echo [*] Origem: GitHub Shinchiro Releases

:: Baixa o pacote de desenvolvimento (contem .dll e .dll.a)
curl -L "%URL%" -o "mpv-dev.7z"

if %ERRORLEVEL% NEQ 0 (
    echo [!] Erro no download. Verifique sua internet.
    pause
    exit /b
)

echo [*] Download concluido.
echo.
echo [*] PROCEDIMENTO AUTOMATIZADO:
echo [1] Extraia o 'mpv-dev.7z'.
echo [2] Mantenha o nome 'libmpv-2.dll'.
echo [3] Renomeie 'libmpv.dll.a' para 'libmpv-2.lib' para o Linker.
echo.
echo Abrindo pasta do projeto...
explorer .
pause
