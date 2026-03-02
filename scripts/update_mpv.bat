@echo off
TITLE AXON ENGINE - MPV UPDATER
COLOR 0A

:: URL do Build Shinchiro (Stable)
SET URL=https://sourceforge.net/projects/mpv-player-windows/files/stable/mpv-x86_64-latest.7z/download

echo [*] Downloading latest MPV Engine...
powershell -Command "Invoke-WebRequest -Uri '%URL%' -OutFile 'mpv.7z'"

echo [*] Extracting Engine...
:: Nota: Expand-Archive nativo não suporta 7z. O usuário precisa de 7zip ou usamos um zip.
:: Para garantir compatibilidade zero-dependency, o ideal é baixar o zip do Bootstrapper do Shinchiro.
:: Mas para simplificar o "God Mode", assumimos que o usuário tem 7z no path ou baixamos o exe.

echo [!] Por favor, extraia o arquivo 'mpv.7z' na pasta raiz do projeto.
explorer .
pause
