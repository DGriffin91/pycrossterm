# copy "..\target\debug\pycrossterm.dll" "pycrossterm.pyd" /Y
import pycrossterm as term
import time
import atexit


def exit_handler():
    term.style.set_foreground_color(255, 255, 255)
    term.cursor.enable_blinking()
    term.cursor.show()


atexit.register(exit_handler)

term.event.enable_mouse_capture()


term.style.set_foreground_color(128, 128, 0)
print("!!!")
term.style.set_foreground_color(255, 128, 0)
print("!!!")
term.style.set_foreground_color(255, 255, 255)
print("!!!")
event = term.event.read()
print(event)
# term.test_stdout()

term.terminal.clear("All")
term.cursor.disable_blinking()
while True:
    term.cursor.hide()
    event = term.event.read()
    if event.event == "MouseDown":
        term.style.set_foreground_color(255, 0, 0)
        term.cursor.move_to(event.x, event.y)
        print("P")
    elif event.event == "MouseUp":
        term.style.set_foreground_color(0, 255, 0)
        term.cursor.move_to(event.x, event.y)
        print("R")
    elif event.event == "MouseDrag":
        term.style.set_foreground_color(0, 0, 255)
        term.cursor.move_to(event.x, event.y)
        print("D")
