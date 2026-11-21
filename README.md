# close_virtual_desktop_win

<img src="resources/logos/app.png" />

## problem

I tend to open multiple virtual-desktops by mistake.  
opened apps are kept in another desktop, and I eventually open another copy.  
this makes <kbd>ALT</kbd>+<kbd>TAB</kbd>/<kbd>WIN</kbd>+<kbd>TAB</kbd> laggy.  

<hr/>

## other, semi-working "tricks" I've tried..

- keep the virtual-desktop button ("taskview") in the taskbar always, and check from time to time (takes up space).

- try to disable it - registry (does nothing)

```reg
Windows Registry Editor Version 5.00

[HKEY_CURRENT_USER\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced]
"MultiDesktopEnabled"=dword:00000000
"VirtualDesktopEnabled"=dword:00000000
"Multimon"=dword:00000000
```

- disable desktop creation hotkey (also disables every native `D` combination) - I've replaced it with AutoHotKey (kind of working).

```reg
Windows Registry Editor Version 5.00

;just D related key combination.
[HKEY_CURRENT_USER\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced]
"DisabledHotkeys"="D"
```

- disable all antive win hotkey combination, and use AutoHotKey to set specific ones - (I've skipped this one, although it should be very effective).

```reg
Windows Registry Editor Version 5.00

;all WIN key combinations.
[HKEY_CURRENT_USER\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer]
"NoWinKeys"=dword:00000001
```

- install a newer windows build just to have somewhat more control over the key combination (not really did much).

<img src="resources/logos/screenshot_system_multitasking.png" />

<hr/>

here is an example for AutoHotKey script that does desktop reveal, as well as opening the desktop of this an another computer,  
it overrides the native key-combination so you don't have to worry about pre-disabling it (although it might work better if you do).  

```autohotkey
#Requires AutoHotkey v2.0
#SingleInstance  force         ; skip prompt and replace old instance automatically.
ListLines        false              ; disable key history / script lines listing for efficiency.
SetWorkingDir    A_ScriptDir  ; set working folder to be the script's folder.

;-------- desktop folder
#^d:: {
  Sleep 100  ; allow shell to restore

  Run A_Desktop,, "Max"

  ;--- alternative way to get special-folder desktop. in-case user has changed the folder from "C:\Users\" . A_UserName . "\Desktop" .
  ;static objShell := ComObject("Shell.Application")   ; local static reusable object (repeat key press will not re-create the object)
  ;static desktop_folder := objShell.Namespace(0)
  ;Run desktop_folder.Self.Path,, "Max"

  Sleep 100  ; allow shell to restore
}

;------------------- disable WIN+CTRL+LEFT/RIGHT from switching virtual desktops by mistake.
#^Left::return
#^Right::return

;-------- desktop folder - network device
#^+d:: {
  Sleep 100  ; allow shell to restore
  ;Run "\\other-computer\c$\Users\Elad\Desktop",, "Max"
  Run "N:\Users\Elad\Desktop",, "Max"
  Sleep 100  ; allow shell to restore
}

#d:: {       ;shell32 - "ToggleDesktop" verb, alternative is IShellDispatch::MinimizeAll / UndoMinimizeALL .
  Sleep 100  ; allow shell to restore
  static objShell := ComObject("Shell.Application")   ; local static reusable object (repeat key press will not re-create the object)
  objShell.ToggleDesktop()
  Sleep 100  ; allow shell to restore
}


;# = WIN
;! = ALT
;^ = CTRL
;+ = SHIFT
;https://eladkarako.github.io/docs_autohotkey_v2/docs/lib/Run.htm
;https://eladkarako.github.io/docs_autohotkey_v2/docs/misc/CLSID-List.htm
;https://eladkarako.github.io/docs_autohotkey_v2/docs/lib/HotIf.htm#IfWin
```

<hr/>

# what this program does.

it sends a key-combination (<kbd>WIN</kbd>+<kbd>CTRL</kbd>+<kbd>F4</kbd>) 32 times,  
it closes whatever desktop you are in, safely,  
moving all opened programs to a previously opened desktop.  
it does nothing if you are only having one desktop.  

you can run it manually,  
or as a scheduled-task.  

but it must be as part of user interactive session (logged-in).

<hr/>

## better solutions.

if there is a Windows API for virtual-desktop,  
that does that directly (not though a key combination),  
I didn't find it. feel free to open a bug/feature request.  

<hr/>
<br/>

<a href="https://paypal.me/31adkarak0" target="_blank" rel="noopener noreferrer">


