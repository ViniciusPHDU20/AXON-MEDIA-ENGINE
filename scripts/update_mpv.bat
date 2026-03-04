@echo off
TITLE AXON ENGINE - MPV UPDATER
COLOR 0A

:: URL direta para o build estável do Shinchiro no SourceForge
SET URL=https://sourceforge.net/projects/mpv-player-windows/files/stable/mpv-x86_64-latest.7z/download

echo [*] Downloading latest MPV Engine (64-bit)...
echo [*] Usando CURL para garantir redirecionamentos corretos...

:: Tenta baixar com curl (nativo no Win 10/11)
curl -L "%URL%" -o "mpv.7z"

if %ERRORLEVEL% NEQ 0 (
    echo [!] Erro no download automatico.
    echo [!] Por favor, baixe MANUALMENTE em: https://mpv.io/installation/
    echo [!] Procure pela sessao "Windows by shinchiro" e baixe o arquivo .7z
    pause
    exit /b
)

echo.
echo [*] Download concluido com sucesso!
echo [*] PROCEDIMENTO MANUAL NECESSARIO:
echo [1] Extraia o arquivo 'mpv.7z'.
echo [2] Copie 'mpv-1.dll' para a RAIZ DO PROJETO (onde esta o Cargo.toml).
echo [3] Renomeie 'mpv-1.dll' para 'mpv.dll'.
echo.
explorer .
pause
