# copy "..\..\target\debug\pycrossterm.dll" "pycrossterm.pyd" /Y
import time
import atexit
from pycrossterm import (
    cursor,
    event,
    style,
    terminal,
    attribute,
    StyledContent,
    color,
    rgb,
)


def exit_handler():
    style.set_foreground_color(color.Reset)
    cursor.enable_blinking()
    cursor.show()


atexit.register(exit_handler)

event.enable_mouse_capture()

terminal.set_size(80, 40)

style.set_foreground_color(rgb(128, 128, 0))
print("!!!")
style.set_foreground_color(rgb(255, 128, 0))
print("!!!")
style.set_foreground_color(rgb(255, 255, 255))
print("!!!")
StyledContent("TEST TEST TEST\nTEST TEST").color(rgb(255, 0, 255)).on(
    rgb(50, 50, 50)
).attribute(
    attribute.CrossedOut  # Wont work in windows CMD
).print()
e = event.read()
print(e)


terminal.clear_all()
cursor.disable_blinking()
print(terminal.size())
while True:
    cursor.hide()
    e = event.read()
    if e.event == event.MouseDown:
        style.set_foreground_color(rgb(255, 0, 0))
        cursor.move_to(e.x, e.y)
        print("P")
    elif e.event == event.MouseUp:
        cursor.move_to(e.x, e.y)
        StyledContent("X").color(color.Black).on(color.White).print()
    elif e.event == event.MouseDrag:
        cursor.move_to(e.x, e.y)
        style.new("X").color(rgb(0, 0, 255)).on(rgb(50, 50, 50)).print()
