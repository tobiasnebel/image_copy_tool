set loopcount=100

:loop

"target/release/image_tool.exe" C:\BILDER\in C:\BILDER\slideshow
timeout /t 10

set /a loopcount=loopcount-1
if %loopcount%==0 goto theexit
goto loop

theexit:
pause

