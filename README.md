# `close_virtual_desktop_win`

safely moves all opened applications across all virtual desktops to one desktop,  
and close all other virtual-desktops, leaving you with just one desktop,  
and all the opened applications and windows in it (same position).

<img src="resources/logos/app.png" />

```
close_virtual_desktop_win.exe
close_virtual_desktop_win.exe --sleep-timeout 250
close_virtual_desktop_win.exe --force-number 3
close_virtual_desktop_win.exe --force-number 3 --sleep-timeout 30
```

## 👇  download (direct URLs - latest release)

- [![](resources/logos/64.png)![](resources/logos/windows.png)x86_64-pc-windows-msvc.zip](https://github.com/eladkarako/close_virtual_desktop_win/releases/latest/download/x86_64-pc-windows-msvc.zip)
- [![](resources/logos/aarch64.png)![](resources/logos/windows.png)aarch64-pc-windows-msvc.zip](https://github.com/eladkarako/close_virtual_desktop_win/releases/latest/download/aarch64-pc-windows-msvc.zip)
- [![](resources/logos/32.png)![](resources/logos/windows.png)i686-pc-windows-msvc.zip](https://github.com/eladkarako/close_virtual_desktop_win/releases/latest/download/i686-pc-windows-msvc.zip)
- [![](resources/logos/txt.png)version.txt](https://github.com/eladkarako/close_virtual_desktop_win/releases/latest/download/version.txt)
- [![](resources/logos/txt.png)changelog.txt](https://github.com/eladkarako/close_virtual_desktop_win/releases/latest/download/changelog.txt)

<hr/>

### 😩 The Problem

I tend to open multiple virtual desktops by mistake.  
Opened apps are kept in another desktop, and I eventually open another copy. 📋📋  
This makes <kbd>ALT</kbd>+<kbd>TAB</kbd> / <kbd>WIN</kbd>+<kbd>TAB</kbd> laggy. 🐌

<hr/>

### ✨ What This Program Does

gradually (but very fast) it sends the hot-key combination to close the current virtual-desktop  
(<kbd>WIN</kbd>+<kbd>CTRL</kbd>+<kbd>F4</kbd>) it does so blindly to the actual amount of opened virtual-desktops you currently have,  
it is safe since it does nothing if you're only have one desktop left.  

Windows itself handles everything. it moves all the windows and opened applications to the next recently opened desktop,  
and closes the virtual-desktop. repeating this will eventually leave you with only one desktop,  
and all you opened programs and windows, in same place.

I've currently hard-coded 32 actions,  
so if you have more opened virtual-desktops you need to re-run the program.

<img src="resources/logos/screenshot_how_it_works.gif" />

you can set to run the program manually,  
or use scheduled-tasks to run it every few hours or when idle (for example).
this program is only used in normal desktop interactive mode (nothing like a remote console),  
so if you choose to use scheduled-task, make sure to set it for a logged-on user.

<hr/>

## 🧪 Other, Semi-Working "Tricks" I've Tried

- 📌 Keep the virtual desktop button ("Task View") in the taskbar at all times, and check from time to time (takes up space).

- 🚫 Try to disable it via the registry (does nothing).

```reg
Windows Registry Editor Version 5.00

[HKEY_CURRENT_USER\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced]
"MultiDesktopEnabled"=dword:00000000
"VirtualDesktopEnabled"=dword:00000000
"Multimon"=dword:00000000
```

- ⌨️ Disable the desktop creation hotkey. This also disables every native `D` key combination. I've replaced it with AutoHotKey (kind of working). 🤏

```reg
Windows Registry Editor Version 5.00

;just D related key combination.
[HKEY_CURRENT_USER\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced]
"DisabledHotkeys"="D"
```

- 🔒 Disable all native Win hotkey combinations and use AutoHotKey to set specific ones. (I've skipped this one, although it should be very effective. 🤔)

```reg
Windows Registry Editor Version 5.00

;all WIN key combinations.
[HKEY_CURRENT_USER\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer]
"NoWinKeys"=dword:00000001
```

- 🪟 Install a newer Windows build to have somewhat more control over the key combinations. (Not really much help. 😅)

<img src="resources/logos/screenshot_system_multitasking.png" />

<hr/>

## 🤖 AutoHotKey Example

Here is an example AutoHotKey script that reveals the desktop, and also opens the desktop of another computer. 🖥️↔️🖥️  
It overrides the native key combination, so you don't have to worry about pre-disabling it (although it might work better if you do). 💪

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

## 🐢 Slowness

You'll see the immediate result, most likely right away. ⚡  
However, the program still waits until all 32 key events finish, which takes about ~6 seconds. ⏳  
If you have more than 32 desktops, you can re-run the tool. 🔁

- ➕ `Win + Ctrl + D`: create a new desktop.
- ❌ `Win + Ctrl + F4`: close the current desktop.
- ◀️▶️ `Win + Ctrl + ←/→`: cycle desktops.
- 🔀 `Alt + Tab` / `Win + Tab`: surf apps across all desktops (configurable).

💡 Note: As far as I can tell, extra virtual desktops by themselves are not the cause of low RAM or CPU spikes. The cause is often when you "lose" an application in another desktop 👻

<hr/>

## 🛠️ Alternative

If you want, you can use `autohotkey` instead of this program. You can even "compile" it to an `.exe` 📦.

```autohotkey
#Requires AutoHotkey v2.0

^!q:: CloseExtraDesktops()  ; Press Ctrl+Alt+Q to start

CloseExtraDesktops() {
    loop 32 {
        SendInput "{LWin down}{LCtrl down}{F4 down}{F4 up}{LCtrl up}{LWin up}"
        Sleep 200
    }
}
```

Or even simpler:

```autohotkey
^!q::Loop 32 {
    Send "#^{F4}"
    Sleep 200
}
```

<hr/>

## 🔮 Better Solutions (In the Future, and Reasoning)

There is no "close virtual desktop" Windows API (yet). 🚫

Microsoft documents `IVirtualDesktopManager`, a COM interface available since Windows 10. 📚  
It can tell you which desktop a window is on (`GetWindowDesktopId`).  
It can tell you whether a window is on the current desktop (`IsWindowOnCurrentVirtualDesktop`).  
It can even move a window to another desktop (`MoveWindowToDesktop`). 🪟➡️🪟  
It is implemented by the `VirtualDesktopManager` class, created with `CoCreateInstance`.

It can't create, switch, or close desktops. 🙅  
Microsoft's guidance is that creating and manipulating desktops is reserved for the user, so the documented API doesn't offer that. 👤

Closing a desktop (currently) only works through the undocumented internal interfaces that tools like PowerToys use, which Microsoft doesn't support. ⚠️  
Otherwise, you can use the keyboard shortcut. ⌨️

- 📖 https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ivirtualdesktopmanager
- 📖 https://learn.microsoft.com/en-us/windows/win32/shell/virtualdesktopmanager
- 🐙 https://github.com/microsoft/PowerToys/issues/6797

That's why this program simulates `Win+Ctrl+F4` with `SendInput`. 🎹  
It's the only public "route" for closing a desktop. 🛣️

- 🐛 Feel free to open a bug or feature request!

<hr/>
<br/>

<a href="https://paypal.me/31adkarak0" target="_blank" rel="noopener noreferrer"></a>
